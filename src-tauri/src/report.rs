use sea_orm::{Set, ActiveModelTrait, EntityTrait};
use entity::report;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ReportData {
    pub fg_id: i32,
    pub attributes: String,
    pub added_by: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ReportResponse {
    pub id: i32,
    pub fg_id: i32,
    pub attributes: String,
}

#[tauri::command]
pub async fn create_report(state: State<'_, AppState>, report_data: ReportData) -> Result<ReportResponse, String> {
    let db = &*state.core_db;

    let report = report::ActiveModel {
        fg_id: Set(report_data.fg_id),
        attributes: Set(report_data.attributes),
        added_by: Set(report_data.added_by),
        ..Default::default()
    };

    let report: report::Model = report
        .insert(db)
        .await
        .map_err(|e| format!("Failed to create report: {}", e))?;

    Ok(ReportResponse {
        id: report.id,
        fg_id: report.fg_id,
        attributes: report.attributes,
    })
}

#[tauri::command]
pub async fn get_report(state: State<'_, AppState>, id: i32) -> Result<ReportResponse, String> {
    let db = &*state.core_db;

    let report = report::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch report: {}", e))?
        .ok_or_else(|| "Report not found".to_string())?;

    Ok(ReportResponse {
        id: report.id,
        fg_id: report.fg_id,
        attributes: report.attributes,
    })
}

#[tauri::command]
pub async fn get_all_reports(state: State<'_, AppState>) -> Result<Vec<ReportResponse>, String> {
    let db = &*state.core_db;

    let reports = report::Entity::find()
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch reports: {}", e))?;

    Ok(reports
        .into_iter()
        .map(|report| ReportResponse {
            id: report.id,
            fg_id: report.fg_id,
            attributes: report.attributes,
        })
        .collect())
}

#[tauri::command]
pub async fn update_report(state: State<'_, AppState>, id: i32, report_data: ReportData) -> Result<ReportResponse, String> {
    let db = &*state.core_db;

    let report = report::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch report: {}", e))?
        .ok_or_else(|| "Report not found".to_string())?;

    let mut report: report::ActiveModel = report.into();
    report.fg_id = Set(report_data.fg_id);
    report.attributes = Set(report_data.attributes);
    report.added_by = Set(report_data.added_by);

    let report: report::Model = report
        .update(db)
        .await
        .map_err(|e| format!("Failed to update report: {}", e))?;

    Ok(ReportResponse {
        id: report.id,
        fg_id: report.fg_id,
        attributes: report.attributes,
    })
}

#[tauri::command]
pub async fn delete_report(state: State<'_, AppState>, id: i32) -> Result<String, String> {
    let db = &*state.core_db;

    let report = report::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch report: {}", e))?
        .ok_or_else(|| "Report not found".to_string())?;

    let report: report::ActiveModel = report.into();
    report
        .delete(db)
        .await
        .map_err(|e| format!("Failed to delete report: {}", e))?;

    Ok(format!("Report {} deleted successfully", id))
}