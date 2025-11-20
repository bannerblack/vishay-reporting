pub mod collector;
pub mod excel_generator;
pub mod template;
pub mod validator;

use crate::AppState;
use std::fs;
use std::path::Path;
use tauri::State;

// Re-export the Tauri commands
pub use collector::collect_report;
pub use validator::validate_report;

/// Generate Excel report from collected data
#[tauri::command]
pub async fn generate_excel_report(
    report_id: i32,
    batch: Option<String>,
    serial_range: Option<String>,
    selected_dates: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    // Collect report data
    let report_data = match collector::collect_report_data(
        report_id,
        batch,
        serial_range,
        selected_dates,
        &state.core_db,
        &state.voltech_db,
        &state.manual_db,
    )
    .await
    {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to collect report data: {}", e)),
    };

    // Generate Excel file
    match excel_generator::generate_report(&report_data) {
        Ok(buffer) => Ok(buffer),
        Err(e) => Err(format!("Failed to generate Excel report: {}", e)),
    }
}

/// Generate Excel report and save to specified file path
#[tauri::command]
pub async fn save_excel_report(
    report_id: i32,
    file_path: String,
    batch: Option<String>,
    serial_range: Option<String>,
    selected_dates: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Generate the Excel file buffer
    let buffer = generate_excel_report(report_id, batch, serial_range, selected_dates, state).await?;

    // Write to file using Rust std::fs
    let path = Path::new(&file_path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directories: {}", e))?;
    }
    
    // Write the file
    fs::write(path, buffer)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}
