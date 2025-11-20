use crate::manual::{operations, queries};
use crate::AppState;
use entity_manual::manual_test_results;
use serde::{Deserialize, Serialize};
use tauri::State;

// ============================================================================
// DTOs
// ============================================================================

#[derive(Debug, Serialize)]
pub struct ImportResult {
    pub files_processed: usize,
    pub records_imported: usize,
}

#[derive(Debug, Serialize)]
pub struct ManualTestResponse {
    pub id: i32,
    pub result: i32,
    pub test: String,
    pub fg: String,
    pub rev: String,
    pub batch: String,
    pub operator: String,
    pub date: String,
    pub time: String,
    pub sn: String,
    pub passfail: String,
    pub minimum: f64,
    pub reading: f64,
    pub maximum: f64,
    pub uom: String,
    pub file_path: String,
    pub normalized_date: String,
}

#[derive(Debug, Deserialize)]
pub struct ManualTestFilter {
    pub fg: Option<String>,
    pub batch: Option<String>,
    pub test_name: Option<String>,
    pub serial_num: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn model_to_response(model: manual_test_results::Model) -> ManualTestResponse {
    ManualTestResponse {
        id: model.id,
        result: model.result,
        test: model.test,
        fg: model.fg,
        rev: model.rev,
        batch: model.batch,
        operator: model.operator,
        date: model.date,
        time: model.time,
        sn: model.sn,
        passfail: model.passfail,
        minimum: model.minimum,
        reading: model.reading,
        maximum: model.maximum,
        uom: model.uom,
        file_path: model.file_path,
        normalized_date: model.normalized_date.to_string(),
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

#[tauri::command]
pub async fn import_manual_file(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<usize, String> {
    operations::import_manual_csv_file(&state.manual_db, &file_path).await
}

#[tauri::command]
pub async fn import_manual_fg_folder(
    state: State<'_, AppState>,
    fg: String,
) -> Result<ImportResult, String> {
    let (files, records) = operations::import_manual_fg_folder(&state.manual_db, &fg).await?;
    Ok(ImportResult {
        files_processed: files,
        records_imported: records,
    })
}

#[tauri::command]
pub async fn get_manual_test_names(
    state: State<'_, AppState>,
    fg: String,
) -> Result<Vec<String>, String> {
    queries::get_manual_test_names_for_fg(&state.manual_db, &fg)
        .await
        .map_err(|e| format!("Failed to get test names: {}", e))
}

#[tauri::command]
pub async fn get_manual_tests(
    state: State<'_, AppState>,
    filter: ManualTestFilter,
) -> Result<Vec<ManualTestResponse>, String> {
    let date_from = filter
        .date_from
        .as_ref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let date_to = filter
        .date_to
        .as_ref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let results = queries::get_manual_tests_filtered(
        &state.manual_db,
        filter.fg.as_deref(),
        filter.batch.as_deref(),
        filter.test_name.as_deref(),
        filter.serial_num.as_deref(),
        date_from,
        date_to,
    )
    .await
    .map_err(|e| format!("Failed to get manual tests: {}", e))?;

    Ok(results.into_iter().map(model_to_response).collect())
}

#[tauri::command]
pub async fn get_manual_summary(
    state: State<'_, AppState>,
    fg: String,
) -> Result<queries::ManualTestSummary, String> {
    queries::get_manual_test_summary_for_fg(&state.manual_db, &fg)
        .await
        .map_err(|e| format!("Failed to get manual test summary: {}", e))
}

#[tauri::command]
pub async fn get_manual_base_path(state: State<'_, AppState>) -> Result<String, String> {
    operations::get_base_path(&state.manual_db).await
}

#[tauri::command]
pub async fn set_manual_base_path(state: State<'_, AppState>, path: String) -> Result<(), String> {
    operations::set_base_path(&state.manual_db, &path).await
}
