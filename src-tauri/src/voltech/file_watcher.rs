// File watcher for Voltech .atr files with master/follower coordination
use chrono::Utc;
use regex::Regex;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;
use tokio::time::{interval, sleep};

use crate::voltech::operations;
use crate::voltech::parser;

// ==================== Watcher State ====================

#[derive(Debug, Clone)]
pub struct WatcherState {
    pub is_active: bool,
    pub is_paused: bool,
    pub instance_id: String,
    pub control_tx: Option<mpsc::UnboundedSender<WatcherControl>>,
}

impl WatcherState {
    pub fn new(instance_id: String) -> Self {
        Self {
            is_active: false,
            is_paused: false,
            instance_id,
            control_tx: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WatcherControl {
    Pause,
    Resume,
    Stop,
}

// ==================== Event Payloads ====================

#[derive(Debug, Clone, Serialize)]
pub struct BatchProgressEvent {
    pub files_processed: usize,
    pub records_inserted: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct WatcherPausedEvent {
    reason: String,
}

#[derive(Debug, Clone, Serialize)]
struct WatcherResumedEvent {
    message: String,
}

// ==================== File Pattern Matching ====================

/// Match Voltech file pattern: C#DDMMYY.atr where # is 0-9
fn is_voltech_file(filename: &str) -> bool {
    let re = Regex::new(r"^C[0-9]\d{6}\.atr$").unwrap();
    re.is_match(filename)
}

/// Get all Voltech files in directory (recursive)
pub async fn get_all_voltech_files(server_path: &str) -> Result<Vec<PathBuf>, String> {
    let path = Path::new(server_path);
    if !path.exists() {
        return Err(format!("Server path does not exist: {}", server_path));
    }

    let mut files = Vec::new();

    fn visit_dirs(
        dir: &Path,
        files: &mut Vec<PathBuf>,
        is_voltech_file: &dyn Fn(&str) -> bool,
    ) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, files, is_voltech_file)?;
                } else if let Some(filename) = path.file_name() {
                    if let Some(name) = filename.to_str() {
                        if is_voltech_file(name) {
                            files.push(path);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    visit_dirs(path, &mut files, &is_voltech_file)
        .map_err(|e| format!("Failed to scan directory: {}", e))?;

    Ok(files)
}

/// Get files from last N days
async fn get_recent_files(server_path: &str, days: i64) -> Result<Vec<PathBuf>, String> {
    let cutoff = Utc::now() - chrono::Duration::days(days);
    let all_files = get_all_voltech_files(server_path).await?;

    let mut recent = Vec::new();
    for file_path in all_files {
        if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                    let file_time = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                        .unwrap_or_else(|| Utc::now());

                    if file_time.timestamp() >= cutoff.timestamp() {
                        recent.push(file_path);
                    }
                }
            }
        }
    }

    Ok(recent)
}

// ==================== File Processing with Retry ====================

/// Process a single file with retry logic
async fn process_file_with_retry(
    db: &DatabaseConnection,
    file_path: &PathBuf,
    max_retries: u32,
) -> Result<usize, String> {
    let delays = [5, 15, 30]; // Immediate retry delays in seconds

    for attempt in 0..=max_retries {
        match parser::parse_and_insert_file(db, file_path.to_str().unwrap()).await {
            Ok(count) => return Ok(count),
            Err(e) => {
                let error_msg = format!("Attempt {} failed: {}", attempt + 1, e);
                eprintln!("{}", error_msg);

                if attempt < max_retries {
                    let delay = if attempt < delays.len() as u32 {
                        delays[attempt as usize]
                    } else {
                        300 // 5 minutes for subsequent retries
                    };
                    sleep(Duration::from_secs(delay)).await;
                } else {
                    // Log final error to database
                    if let Err(log_err) = operations::log_parse_error(
                        db,
                        file_path.to_str().unwrap(),
                        &e.to_string(),
                        None,
                    )
                    .await
                    {
                        eprintln!("Failed to log error: {}", log_err);
                    }
                    return Err(error_msg);
                }
            }
        }
    }

    Err("Max retries exceeded".to_string())
}

// ==================== Watcher Core Logic ====================

/// Main watcher loop
async fn watcher_loop(
    app: AppHandle,
    db: Arc<DatabaseConnection>,
    server_path: String,
    instance_id: String,
    mut control_rx: mpsc::UnboundedReceiver<WatcherControl>,
) {
    println!("Voltech watcher started: instance_id={}", instance_id);

    let mut is_paused = false;
    let mut heartbeat_interval = interval(Duration::from_secs(30));
    let mut poll_interval = interval(Duration::from_secs(10));
    let mut last_monthly_scan = get_last_monthly_scan(&db).await;

    loop {
        tokio::select! {
            // Handle control messages
            Some(control) = control_rx.recv() => {
                match control {
                    WatcherControl::Pause => {
                        is_paused = true;
                        println!("Watcher paused");
                        let _ = app.emit("voltech-watcher-paused", WatcherPausedEvent {
                            reason: "Manual pause".to_string(),
                        });
                    }
                    WatcherControl::Resume => {
                        is_paused = false;
                        println!("Watcher resumed");
                        let _ = app.emit("voltech-watcher-resumed", WatcherResumedEvent {
                            message: "Watcher resumed".to_string(),
                        });
                    }
                    WatcherControl::Stop => {
                        println!("Watcher stopping");
                        if let Err(e) = operations::release_lock(&db, &instance_id).await {
                            eprintln!("Failed to release lock: {}", e);
                        }
                        break;
                    }
                }
            }

            // Update heartbeat every 30 seconds
            _ = heartbeat_interval.tick() => {
                if !is_paused {
                    if let Err(e) = operations::update_heartbeat(&db, &instance_id).await {
                        eprintln!("Failed to update heartbeat: {}", e);
                    }
                }
            }

            // Poll for new files every 10 seconds
            _ = poll_interval.tick() => {
                if is_paused {
                    continue;
                }

                // Check if we're still the master
                match operations::get_lock_info(&db).await {
                    Ok(Some(lock)) => {
                        if lock.holder_id != instance_id || !lock.is_active {
                            println!("Lost master lock, stopping watcher");
                            break;
                        }
                    }
                    Ok(None) => {
                        println!("No active lock found, stopping watcher");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to check lock status: {}", e);
                        continue;
                    }
                }

                // Process new files
                match get_all_voltech_files(&server_path).await {
                    Ok(files) => {
                        let mut files_to_process = Vec::new();

                        for file_path in files {
                            // Check if file needs processing
                            if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
                                let file_size = metadata.len() as i32;
                                if let Ok(modified) = metadata.modified() {
                                    if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                                        let file_modified = duration.as_secs() as i32;

                                        match operations::needs_processing(
                                            &db,
                                            file_path.to_str().unwrap(),
                                            file_size,
                                            file_modified
                                        ).await {
                                            Ok(true) => files_to_process.push(file_path),
                                            Ok(false) => {},
                                            Err(e) => eprintln!("Error checking file: {}", e),
                                        }
                                    }
                                }
                            }
                        }

                        if !files_to_process.is_empty() {
                            let mut total_records = 0;
                            let mut total_files = 0;
                            let mut errors = Vec::new();

                            for file_path in &files_to_process {
                                match process_file_with_retry(&db, file_path, 3).await {
                                    Ok(count) => {
                                        if count > 0 {
                                            total_files += 1;
                                            total_records += count;
                                            println!("Processed: {:?} ({} records)", file_path, count);
                                        }
                                    }
                                    Err(e) => {
                                        errors.push(format!("{:?}: {}", file_path, e));
                                    }
                                }
                            }

                            if total_files > 0 || !errors.is_empty() {
                                let _ = app.emit("voltech-batch-progress", BatchProgressEvent {
                                    files_processed: total_files,
                                    records_inserted: total_records,
                                    errors,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to scan directory: {}", e);
                    }
                }

                // Check for monthly maintenance scan (30 days)
                let now = Utc::now();
                if let Some(last_scan) = last_monthly_scan {
                    let days_since = (now - last_scan).num_days();
                    if days_since >= 7 {
                        println!("Starting weekly 30-day maintenance scan");
                        if let Err(e) = run_maintenance_scan(&app, &db, &server_path).await {
                            eprintln!("Maintenance scan failed: {}", e);
                        } else {
                            last_monthly_scan = Some(now);
                            if let Err(e) = set_last_monthly_scan(&db, now).await {
                                eprintln!("Failed to update last monthly scan: {}", e);
                            }
                        }
                    }
                } else {
                    // No previous scan, do it now
                    println!("Starting initial 30-day maintenance scan");
                    if let Err(e) = run_maintenance_scan(&app, &db, &server_path).await {
                        eprintln!("Maintenance scan failed: {}", e);
                    } else {
                        last_monthly_scan = Some(now);
                        if let Err(e) = set_last_monthly_scan(&db, now).await {
                            eprintln!("Failed to update last monthly scan: {}", e);
                        }
                    }
                }
            }
        }
    }

    println!("Watcher loop ended");
}

// ==================== Maintenance Scan ====================

/// Run maintenance scan for files from last 30 days
pub async fn run_maintenance_scan(
    app: &AppHandle,
    db: &DatabaseConnection,
    server_path: &str,
) -> Result<(), String> {
    println!("Starting 30-day maintenance scan...");

    let files = get_recent_files(server_path, 30).await?;
    println!("Found {} files to scan", files.len());

    let mut files_to_process = Vec::new();
    for file_path in files {
        if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
            let file_size = metadata.len() as i32;
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                    let file_modified = duration.as_secs() as i32;

                    match operations::needs_processing(
                        db,
                        file_path.to_str().unwrap(),
                        file_size,
                        file_modified,
                    )
                    .await
                    {
                        Ok(true) => files_to_process.push(file_path.to_str().unwrap().to_string()),
                        Ok(false) => {}
                        Err(e) => eprintln!("Error checking file: {}", e),
                    }
                }
            }
        }
    }

    if !files_to_process.is_empty() {
        println!(
            "Processing {} files in maintenance scan",
            files_to_process.len()
        );

        match parser::process_files_batch(db, &files_to_process, 3).await {
            Ok((files_count, records_count, errors)) => {
                println!(
                    "Maintenance scan complete: {} files, {} records",
                    files_count, records_count
                );

                let _ = app.emit(
                    "voltech-batch-progress",
                    BatchProgressEvent {
                        files_processed: files_count,
                        records_inserted: records_count,
                        errors,
                    },
                );

                Ok(())
            }
            Err(e) => Err(format!("Batch processing failed: {}", e)),
        }
    } else {
        println!("No files to process in maintenance scan");
        Ok(())
    }
}

// ==================== Helper Functions ====================

async fn get_last_monthly_scan(db: &DatabaseConnection) -> Option<chrono::DateTime<chrono::Utc>> {
    match operations::get_setting(db, "last_monthly_scan").await {
        Ok(Some(value)) => {
            if let Ok(timestamp) = value.parse::<i64>() {
                chrono::DateTime::from_timestamp(timestamp, 0)
            } else {
                None
            }
        }
        _ => None,
    }
}

async fn set_last_monthly_scan(
    db: &DatabaseConnection,
    time: chrono::DateTime<chrono::Utc>,
) -> Result<(), String> {
    operations::set_setting(db, "last_monthly_scan", &time.timestamp().to_string())
        .await
        .map_err(|e| format!("Failed to set last_monthly_scan: {}", e))
}

// ==================== Public API ====================

/// Start the file watcher as a background task
pub async fn start_watcher(
    app: AppHandle,
    db: Arc<DatabaseConnection>,
    server_path: String,
    instance_id: String,
) -> Result<mpsc::UnboundedSender<WatcherControl>, String> {
    // Create control channel
    let (control_tx, control_rx) = mpsc::unbounded_channel();

    // Spawn watcher task
    tokio::spawn(async move {
        watcher_loop(app, db, server_path, instance_id, control_rx).await;
    });

    Ok(control_tx)
}
