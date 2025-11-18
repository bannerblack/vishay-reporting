// Tauri commands for voltech functionality
use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use crate::AppState;
use crate::voltech::{operations, file_watcher, queries};
use entity_voltech;
use entity;

// Type aliases for query results
pub use queries::{PartListItem as PartInfo};

// ==================== DTOs ====================

#[derive(Debug, Serialize)]
pub struct WatcherStatusResponse {
    pub role: String, // "master", "follower", or "none"
    pub master_user: Option<String>,
    pub is_active: bool,
    pub is_paused: bool,
    pub can_force_master: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoltechSettings {
    pub server_path: String,
    pub db_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorFilter {
    pub acknowledged: Option<bool>,
    pub file_path: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DateRangeFilter {
    pub start_date: String,
    pub end_date: String,
}

// ==================== Helper Functions ====================

/// Check if user has admin permission
async fn check_admin_permission(state: &State<'_, AppState>, username: &str) -> Result<bool, String> {
    // Get user by username
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(username))
        .one(state.core_db.as_ref())
        .await
        .map_err(|e| format!("Failed to get user: {}", e))?;

    if let Some(user) = user {
        // Parse permissions JSON
        let permissions: Result<Vec<String>, _> = serde_json::from_str(&user.permissions);
        if let Ok(perms) = permissions {
            Ok(perms.contains(&"admin".to_string()))
        } else {
            Ok(false)
        }
    } else {
        Err("User not found".to_string())
    }
}

/// Get current Windows username
fn get_current_username() -> Result<String, String> {
    whoami::username().parse().map_err(|_| "Failed to get username".to_string())
}

/// Validate UNC path
fn validate_unc_path(path: &str) -> Result<(), String> {
    if !path.starts_with("\\\\") {
        return Err("Path must be a UNC path (start with \\\\)".to_string());
    }
    
    let p = std::path::Path::new(path);
    if !p.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    Ok(())
}

// ==================== File Processing Commands ====================

#[tauri::command]
pub async fn start_voltech_watcher(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<WatcherStatusResponse, String> {
    let username = get_current_username()?;
    
    // Get server path from settings
    let server_path = operations::get_setting(&state.voltech_db, "server_path")
        .await
        .map_err(|e| format!("Failed to get server_path: {}", e))?
        .ok_or("server_path not configured")?;

    // Check for stale lock
    let is_stale = operations::check_stale_lock(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to check lock: {}", e))?;

    // Get current lock info
    let lock_info = operations::get_lock_info(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to get lock info: {}", e))?;

    // Determine if we can acquire lock
    let can_acquire = lock_info.is_none() || !lock_info.as_ref().unwrap().is_active || is_stale;

    if can_acquire {
        // Acquire lock and become master
        let instance_id = operations::acquire_lock(&state.voltech_db, &username)
            .await
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        // Start watcher
        let control_tx = file_watcher::start_watcher(
            app,
            state.voltech_db.clone(),
            server_path,
            instance_id.clone()
        ).await?;

        // Update state
        let mut watcher_state = state.voltech_watcher_state.lock().await;
        watcher_state.is_active = true;
        watcher_state.is_paused = false;
        watcher_state.instance_id = instance_id;
        watcher_state.control_tx = Some(control_tx);

        Ok(WatcherStatusResponse {
            role: "master".to_string(),
            master_user: Some(username),
            is_active: true,
            is_paused: false,
            can_force_master: false,
        })
    } else {
        // We're a follower
        Ok(WatcherStatusResponse {
            role: "follower".to_string(),
            master_user: lock_info.map(|l| l.holder_name),
            is_active: false,
            is_paused: false,
            can_force_master: check_admin_permission(&state, &username).await.unwrap_or(false),
        })
    }
}

#[tauri::command]
pub async fn stop_voltech_watcher(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut watcher_state = state.voltech_watcher_state.lock().await;
    
    if let Some(control_tx) = watcher_state.control_tx.take() {
        control_tx.send(file_watcher::WatcherControl::Stop)
            .map_err(|e| format!("Failed to send stop signal: {}", e))?;
        
        watcher_state.is_active = false;
        watcher_state.is_paused = false;
        
        Ok("Watcher stopped".to_string())
    } else {
        Err("Watcher is not running".to_string())
    }
}

#[tauri::command]
pub async fn pause_voltech_watcher(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut watcher_state = state.voltech_watcher_state.lock().await;
    
    if let Some(control_tx) = &watcher_state.control_tx {
        control_tx.send(file_watcher::WatcherControl::Pause)
            .map_err(|e| format!("Failed to send pause signal: {}", e))?;
        
        watcher_state.is_paused = true;
        
        Ok("Watcher paused".to_string())
    } else {
        Err("Watcher is not running".to_string())
    }
}

#[tauri::command]
pub async fn resume_voltech_watcher(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut watcher_state = state.voltech_watcher_state.lock().await;
    
    if let Some(control_tx) = &watcher_state.control_tx {
        control_tx.send(file_watcher::WatcherControl::Resume)
            .map_err(|e| format!("Failed to send resume signal: {}", e))?;
        
        watcher_state.is_paused = false;
        
        Ok("Watcher resumed".to_string())
    } else {
        Err("Watcher is not running".to_string())
    }
}

#[tauri::command]
pub async fn get_voltech_watcher_status(
    state: State<'_, AppState>,
) -> Result<WatcherStatusResponse, String> {
    let username = get_current_username()?;
    let watcher_state = state.voltech_watcher_state.lock().await;
    
    // Get lock info
    let lock_info = operations::get_lock_info(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to get lock info: {}", e))?;

    if watcher_state.is_active {
        Ok(WatcherStatusResponse {
            role: "master".to_string(),
            master_user: Some(username.clone()),
            is_active: true,
            is_paused: watcher_state.is_paused,
            can_force_master: false,
        })
    } else if let Some(lock) = lock_info {
        if lock.is_active {
            Ok(WatcherStatusResponse {
                role: "follower".to_string(),
                master_user: Some(lock.holder_name),
                is_active: false,
                is_paused: false,
                can_force_master: check_admin_permission(&state, &username).await.unwrap_or(false),
            })
        } else {
            Ok(WatcherStatusResponse {
                role: "none".to_string(),
                master_user: None,
                is_active: false,
                is_paused: false,
                can_force_master: false,
            })
        }
    } else {
        Ok(WatcherStatusResponse {
            role: "none".to_string(),
            master_user: None,
            is_active: false,
            is_paused: false,
            can_force_master: false,
        })
    }
}

#[tauri::command]
pub async fn import_voltech_files(
    app: AppHandle,
    state: State<'_, AppState>,
    _date_range: DateRangeFilter,
) -> Result<String, String> {
    let username = get_current_username()?;
    
    // Check admin permission
    if !check_admin_permission(&state, &username).await? {
        return Err("Admin permission required".to_string());
    }

    // Get server path
    let server_path = operations::get_setting(&state.voltech_db, "server_path")
        .await
        .map_err(|e| format!("Failed to get server_path: {}", e))?
        .ok_or("server_path not configured")?;

    // Pause watcher if active
    let was_paused = {
        let mut watcher_state = state.voltech_watcher_state.lock().await;
        if watcher_state.is_active && !watcher_state.is_paused {
            if let Some(control_tx) = &watcher_state.control_tx {
                control_tx.send(file_watcher::WatcherControl::Pause)
                    .map_err(|e| format!("Failed to pause watcher: {}", e))?;
                watcher_state.is_paused = true;
            }
            false
        } else {
            true
        }
    };

    // Parse date range and get files
    // For now, we'll use maintenance scan which processes last 30 days
    let result = file_watcher::run_maintenance_scan(
        &app,
        &state.voltech_db,
        &server_path
    ).await;

    // Resume watcher if it was running
    if !was_paused {
        let watcher_state = state.voltech_watcher_state.lock().await;
        if let Some(control_tx) = &watcher_state.control_tx {
            let _ = control_tx.send(file_watcher::WatcherControl::Resume);
        }
    }

    result.map(|_| "Import completed".to_string())
}

#[tauri::command]
pub async fn force_acquire_voltech_master(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let username = get_current_username()?;
    
    // Check admin permission
    if !check_admin_permission(&state, &username).await? {
        return Err("Admin permission required".to_string());
    }

    // Force release any existing lock
    operations::force_release_lock(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to force release lock: {}", e))?;

    // Now start watcher (which will acquire the lock)
    start_voltech_watcher(app, state).await?;

    Ok("Master lock acquired".to_string())
}

// ==================== Settings Commands ====================

#[tauri::command]
pub async fn get_voltech_settings(
    state: State<'_, AppState>,
) -> Result<VoltechSettings, String> {
    let server_path = operations::get_setting(&state.voltech_db, "server_path")
        .await
        .map_err(|e| format!("Failed to get server_path: {}", e))?
        .unwrap_or_default();

    let db_path = operations::get_setting(&state.voltech_db, "db_path")
        .await
        .map_err(|e| format!("Failed to get db_path: {}", e))?
        .unwrap_or_default();

    Ok(VoltechSettings {
        server_path,
        db_path,
    })
}

#[tauri::command]
pub async fn set_voltech_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<String, String> {
    let username = get_current_username()?;
    
    // Check admin permission
    if !check_admin_permission(&state, &username).await? {
        return Err("Admin permission required".to_string());
    }

    // Validate UNC paths for server_path
    if key == "server_path" {
        validate_unc_path(&value)?;
    }

    operations::set_setting(&state.voltech_db, &key, &value)
        .await
        .map_err(|e| format!("Failed to set setting: {}", e))?;

    Ok(format!("Setting {} updated", key))
}

#[tauri::command]
pub async fn get_all_voltech_settings(
    state: State<'_, AppState>,
) -> Result<Vec<entity_voltech::settings::Model>, String> {
    operations::get_all_settings(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to get settings: {}", e))
}

#[tauri::command]
pub async fn delete_voltech_setting(
    state: State<'_, AppState>,
    key: String,
) -> Result<String, String> {
    let username = get_current_username()?;
    
    // Check admin permission
    if !check_admin_permission(&state, &username).await? {
        return Err("Admin permission required".to_string());
    }

    operations::delete_setting(&state.voltech_db, &key)
        .await
        .map_err(|e| format!("Failed to delete setting: {}", e))?;

    Ok(format!("Setting {} deleted", key))
}

// ==================== Error Commands ====================

#[tauri::command]
pub async fn get_voltech_errors(
    state: State<'_, AppState>,
    filter: ErrorFilter,
) -> Result<Vec<entity_voltech::parse_errors::Model>, String> {
    operations::get_errors(&state.voltech_db, filter.acknowledged, filter.file_path)
        .await
        .map_err(|e| format!("Failed to get errors: {}", e))
}

#[tauri::command]
pub async fn acknowledge_voltech_errors(
    state: State<'_, AppState>,
    error_ids: Vec<i32>,
) -> Result<u64, String> {
    operations::acknowledge_errors(&state.voltech_db, error_ids)
        .await
        .map_err(|e| format!("Failed to acknowledge errors: {}", e))
}

#[tauri::command]
pub async fn acknowledge_file_errors(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<u64, String> {
    operations::acknowledge_file_errors(&state.voltech_db, &file_path)
        .await
        .map_err(|e| format!("Failed to acknowledge file errors: {}", e))
}

#[tauri::command]
pub async fn cleanup_old_voltech_errors(
    state: State<'_, AppState>,
    days: i64,
) -> Result<u64, String> {
    operations::cleanup_old_errors(&state.voltech_db, days)
        .await
        .map_err(|e| format!("Failed to cleanup errors: {}", e))
}

// ==================== Lock Management Commands ====================

#[tauri::command]
pub async fn get_voltech_lock_status(
    state: State<'_, AppState>,
) -> Result<Option<entity_voltech::watcher_lock::Model>, String> {
    operations::get_lock_info(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to get lock status: {}", e))
}

#[tauri::command]
pub async fn force_release_voltech_lock(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let username = get_current_username()?;
    
    // Check admin permission
    if !check_admin_permission(&state, &username).await? {
        return Err("Admin permission required".to_string());
    }

    operations::force_release_lock(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to force release lock: {}", e))?;

    Ok("Lock released".to_string())
}

// ==================== Query Commands (Batch) ====================

#[tauri::command]
pub async fn get_recent_batches_for_part(
    state: State<'_, AppState>,
    part: String,
    limit: Option<u64>,
) -> Result<Vec<queries::BatchListItem>, String> {
    queries::get_recent_batches_for_part(&state.voltech_db, &part, limit)
        .await
        .map_err(|e| format!("Failed to get batches: {}", e))
}

#[tauri::command]
pub async fn get_batch_details(
    state: State<'_, AppState>,
    batch: String,
) -> Result<Option<queries::BatchSummary>, String> {
    queries::get_batch_details(&state.voltech_db, &batch)
        .await
        .map_err(|e| format!("Failed to get batch details: {}", e))
}

#[tauri::command]
pub async fn search_batches(
    state: State<'_, AppState>,
    filter: queries::BatchSearchFilter,
) -> Result<Vec<queries::BatchListItem>, String> {
    queries::search_batches(&state.voltech_db, filter)
        .await
        .map_err(|e| format!("Failed to search batches: {}", e))
}

#[tauri::command]
pub async fn get_batch_tests(
    state: State<'_, AppState>,
    batch: String,
) -> Result<Vec<entity_voltech::test_results::Model>, String> {
    queries::get_batch_tests(&state.voltech_db, &batch)
        .await
        .map_err(|e| format!("Failed to get batch tests: {}", e))
}

#[tauri::command]
pub async fn get_batches_for_part(
    state: State<'_, AppState>,
    part: String,
) -> Result<Vec<String>, String> {
    queries::get_batches_for_part(&state.voltech_db, &part)
        .await
        .map_err(|e| format!("Failed to get batches for part: {}", e))
}

// ==================== Query Commands (Part) ====================

#[tauri::command]
pub async fn get_all_parts(
    state: State<'_, AppState>,
    limit: Option<u64>,
) -> Result<Vec<PartInfo>, String> {
    queries::get_all_parts(&state.voltech_db, limit)
        .await
        .map_err(|e| format!("Failed to get parts: {}", e))
}

#[tauri::command]
pub async fn get_part_summary(
    state: State<'_, AppState>,
    part: String,
) -> Result<Option<queries::PartSummary>, String> {
    queries::get_part_summary(&state.voltech_db, &part)
        .await
        .map_err(|e| format!("Failed to get part summary: {}", e))
}

#[tauri::command]
pub async fn search_parts(
    state: State<'_, AppState>,
    pattern: String,
    limit: Option<u64>,
) -> Result<Vec<String>, String> {
    queries::search_parts(&state.voltech_db, &pattern, limit)
        .await
        .map_err(|e| format!("Failed to search parts: {}", e))
}

// ==================== Query Commands (Test) ====================

#[tauri::command]
pub async fn search_tests(
    state: State<'_, AppState>,
    filter: queries::TestSearchFilter,
) -> Result<Vec<entity_voltech::test_results::Model>, String> {
    queries::search_tests(&state.voltech_db, filter)
        .await
        .map_err(|e| format!("Failed to search tests: {}", e))
}

#[tauri::command]
pub async fn get_failed_tests(
    state: State<'_, AppState>,
    limit: Option<u64>,
) -> Result<Vec<entity_voltech::test_results::Model>, String> {
    queries::get_failed_tests(&state.voltech_db, limit)
        .await
        .map_err(|e| format!("Failed to get failed tests: {}", e))
}

#[tauri::command]
pub async fn get_test_by_serial(
    state: State<'_, AppState>,
    serial_num: String,
) -> Result<Vec<entity_voltech::test_results::Model>, String> {
    queries::get_tests_by_serial(&state.voltech_db, &serial_num)
        .await
        .map_err(|e| format!("Failed to get test by serial: {}", e))
}

// ==================== Query Commands (Stats) ====================

#[tauri::command]
pub async fn get_daily_stats(
    state: State<'_, AppState>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> Result<Vec<queries::DailyStats>, String> {
    queries::get_daily_stats(
        &state.voltech_db,
        date_from.as_deref(),
        date_to.as_deref()
    )
    .await
    .map_err(|e| format!("Failed to get daily stats: {}", e))
}

#[tauri::command]
pub async fn get_operator_stats(
    state: State<'_, AppState>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> Result<Vec<queries::OperatorStats>, String> {
    queries::get_operator_stats(
        &state.voltech_db,
        date_from.as_deref(),
        date_to.as_deref()
    )
    .await
    .map_err(|e| format!("Failed to get operator stats: {}", e))
}

#[tauri::command]
pub async fn get_overall_stats(
    state: State<'_, AppState>,
) -> Result<Option<queries::OverallStats>, String> {
    queries::get_overall_stats(&state.voltech_db)
        .await
        .map_err(|e| format!("Failed to get overall stats: {}", e))
}

#[tauri::command]
pub async fn get_part_stats(
    state: State<'_, AppState>,
    part: String,
) -> Result<Option<queries::OverallStats>, String> {
    queries::get_part_stats(&state.voltech_db, &part)
        .await
        .map_err(|e| format!("Failed to get part stats: {}", e))
}