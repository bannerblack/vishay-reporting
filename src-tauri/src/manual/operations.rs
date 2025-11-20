use crate::manual::parser::{parse_manual_csv, ManualTestResult};
use entity_manual::{manual_test_results, processed_files, settings};
use sea_orm::*;
use std::path::Path;

/// Import a single manual test CSV file into the database
pub async fn import_manual_csv_file(db: &DbConn, file_path: &str) -> Result<usize, String> {
    // Check if file has already been processed
    let existing = processed_files::Entity::find()
        .filter(processed_files::Column::FilePath.eq(file_path))
        .one(db)
        .await
        .map_err(|e| format!("Database error checking processed files: {}", e))?;

    if existing.is_some() {
        return Ok(0); // Already processed
    }

    // Parse CSV file
    let test_results = parse_manual_csv(file_path)?;
    let record_count = test_results.len();

    if record_count == 0 {
        return Ok(0);
    }

    // Insert all test results
    for result in test_results {
        let active_model = manual_test_results::ActiveModel {
            result: Set(result.result),
            test: Set(result.test),
            fg: Set(result.fg),
            rev: Set(result.rev),
            batch: Set(result.batch),
            operator: Set(result.operator),
            date: Set(result.date),
            time: Set(result.time),
            sn: Set(result.sn),
            passfail: Set(result.passfail),
            minimum: Set(result.minimum),
            reading: Set(result.reading),
            maximum: Set(result.maximum),
            uom: Set(result.uom),
            file_path: Set(result.file_path),
            created_at: Set(chrono::Utc::now().into()),
            normalized_date: Set(result.normalized_date),
            ..Default::default()
        };

        active_model
            .insert(db)
            .await
            .map_err(|e| format!("Failed to insert test result: {}", e))?;
    }

    // Mark file as processed
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let processed_file = processed_files::ActiveModel {
        file_name: Set(file_name),
        file_path: Set(file_path.to_string()),
        processed_at: Set(chrono::Utc::now().into()),
        record_count: Set(record_count as i32),
        ..Default::default()
    };

    processed_file
        .insert(db)
        .await
        .map_err(|e| format!("Failed to mark file as processed: {}", e))?;

    Ok(record_count)
}

/// Import all CSV files from an FG folder
pub async fn import_manual_fg_folder(db: &DbConn, fg: &str) -> Result<(usize, usize), String> {
    let base_path = get_base_path(db).await?;
    let fg_folder = format!("{}{}", base_path, fg);

    // Check if folder exists
    let path = Path::new(&fg_folder);
    if !path.exists() || !path.is_dir() {
        return Err(format!("FG folder does not exist: {}", fg_folder));
    }

    let mut total_files = 0;
    let mut total_records = 0;

    // Read all CSV files in directory
    let entries =
        std::fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let file_path = entry.path();

        if file_path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let file_path_str = file_path
                .to_str()
                .ok_or_else(|| "Invalid file path".to_string())?;

            match import_manual_csv_file(db, file_path_str).await {
                Ok(count) if count > 0 => {
                    total_files += 1;
                    total_records += count;
                    println!("Imported {} records from {}", count, file_path_str);
                }
                Ok(_) => {
                    // Already processed or empty, skip
                }
                Err(e) => {
                    eprintln!("Error importing {}: {}", file_path_str, e);
                }
            }
        }
    }

    Ok((total_files, total_records))
}

/// Get the base path from settings
pub async fn get_base_path(db: &DbConn) -> Result<String, String> {
    let setting = settings::Entity::find()
        .filter(settings::Column::Key.eq("base_path"))
        .one(db)
        .await
        .map_err(|e| format!("Failed to get base_path setting: {}", e))?
        .ok_or_else(|| "base_path setting not found".to_string())?;

    Ok(setting.value)
}

/// Set the base path in settings
pub async fn set_base_path(db: &DbConn, path: &str) -> Result<(), String> {
    let existing = settings::Entity::find()
        .filter(settings::Column::Key.eq("base_path"))
        .one(db)
        .await
        .map_err(|e| format!("Failed to check base_path setting: {}", e))?;

    if let Some(setting) = existing {
        // Update existing
        let mut active: settings::ActiveModel = setting.into();
        active.value = Set(path.to_string());
        active.updated_at = Set(chrono::Utc::now().into());

        active
            .update(db)
            .await
            .map_err(|e| format!("Failed to update base_path: {}", e))?;
    } else {
        // Insert new
        let new_setting = settings::ActiveModel {
            key: Set("base_path".to_string()),
            value: Set(path.to_string()),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        new_setting
            .insert(db)
            .await
            .map_err(|e| format!("Failed to insert base_path: {}", e))?;
    }

    Ok(())
}
