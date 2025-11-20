use crate::AppState;
use ::entity::fg as fg_entity;
use ::entity::report as report_entity;
use ::entity::test as test_entity;
use entity_manual::manual_test_results;
use entity_voltech::test_results as voltech_test_results;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tauri::State;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Serialize)]
pub struct SingleTestResult {
    pub serial_number: Option<String>,
    pub batch: String,
    pub date: String,
    pub result: String,                  // "PASS" or "FAIL"
    pub measurements: serde_json::Value, // JSON object with test-specific measurements
}

#[derive(Debug, Serialize)]
pub struct TestResultData {
    pub test_id: i32,
    pub test_name: String,
    pub source_type: String,
    pub associated_test: Option<String>,
    pub spec_min: Option<f64>,
    pub spec_max: Option<f64>,
    pub spec_unit: Option<String>,
    pub results: Vec<SingleTestResult>,
}

#[derive(Debug, Serialize)]
pub struct ReportData {
    pub report_id: i32,
    pub fg_number: String,
    pub fg_revision: String,
    pub fg_customer: String,
    pub is_serialized: bool,
    pub batch: Option<String>,
    pub serial_range: Option<String>,
    pub test_results: Vec<TestResultData>,
}

// ============================================================================
// Data Collection Functions
// ============================================================================

/// Collect all test data for a report
pub async fn collect_report_data(
    report_id: i32,
    batch: Option<String>,
    serial_range: Option<String>,
    selected_dates: Option<Vec<String>>,
    core_db: &DbConn,
    voltech_db: &DbConn,
    manual_db: &DbConn,
) -> Result<ReportData, DbErr> {
    // Get report with FG info
    let report = report_entity::Entity::find_by_id(report_id)
        .one(core_db)
        .await?
        .ok_or(DbErr::RecordNotFound("Report not found".to_string()))?;

    let fg = fg_entity::Entity::find_by_id(report.fg_id)
        .one(core_db)
        .await?
        .ok_or(DbErr::RecordNotFound("FG not found".to_string()))?;

    let is_serialized = fg.serialized;

    // Get all tests for this report
    let tests = test_entity::Entity::find()
        .filter(test_entity::Column::ReportId.eq(report_id))
        .all(core_db)
        .await?;

    let mut test_results = Vec::new();

    for test in tests {
        let test_data = match test.source_type.as_str() {
            "voltech" => {
                collect_voltech_test(
                    &test,
                    is_serialized,
                    batch.as_deref(),
                    serial_range.as_deref(),
                    selected_dates.as_ref(),
                    voltech_db,
                )
                .await?
            }
            "manual" => {
                collect_manual_test(
                    &test,
                    is_serialized,
                    batch.as_deref(),
                    serial_range.as_deref(),
                    selected_dates.as_ref(),
                    manual_db,
                )
                .await?
            }
            "other" => {
                // For "other" type, create empty result structure
                TestResultData {
                    test_id: test.id,
                    test_name: test.test_type.clone(),
                    source_type: test.source_type.clone(),
                    associated_test: test.associated_test.clone(),
                    spec_min: test.minimum,
                    spec_max: test.maximum,
                    spec_unit: Some(test.uo_m.clone()),
                    results: Vec::new(),
                }
            }
            _ => TestResultData {
                test_id: test.id,
                test_name: test.test_type.clone(),
                source_type: test.source_type.clone(),
                associated_test: test.associated_test.clone(),
                spec_min: test.minimum,
                spec_max: test.maximum,
                spec_unit: Some(test.uo_m.clone()),
                results: Vec::new(),
            },
        };

        test_results.push(test_data);
    }

    Ok(ReportData {
        report_id,
        fg_number: fg.fg.clone(),
        fg_revision: fg.rev.clone(),
        fg_customer: fg.customer.clone(),
        is_serialized,
        batch,
        serial_range,
        test_results,
    })
}

/// Collect voltech test results
async fn collect_voltech_test(
    test: &test_entity::Model,
    is_serialized: bool,
    batch: Option<&str>,
    serial_range: Option<&str>,
    selected_dates: Option<&Vec<String>>,
    voltech_db: &DbConn,
) -> Result<TestResultData, DbErr> {
    let associated_test = test.associated_test.as_deref().unwrap_or("");
    let mut results = Vec::new();

    if is_serialized {
        // Serialized mode: filter by serial range
        if let Some(range) = serial_range {
            // Parse serial range (e.g., "1001-1010")
            if let Some((start_str, end_str)) = range.split_once('-') {
                if let (Ok(start), Ok(end)) = (start_str.parse::<i32>(), end_str.parse::<i32>()) {
                    // Query using measurements JSON search
                    let records = voltech_test_results::Entity::find()
                        .filter(
                            voltech_test_results::Column::Measurements
                                .contains(&format!("\"{}\"", associated_test)),
                        )
                        .all(voltech_db)
                        .await?;

                    for record in records {
                        // Check if serial is within range
                        if let Ok(serial_num) = record.serial_num.parse::<i32>() {
                            if serial_num >= start && serial_num <= end {
                                // Parse measurements JSON
                                let measurements: serde_json::Value =
                                    serde_json::from_str(&record.measurements)
                                        .unwrap_or(serde_json::json!({}));

                                results.push(SingleTestResult {
                                    serial_number: Some(record.serial_num.clone()),
                                    batch: record.batch.clone(),
                                    date: record
                                        .normalized_date
                                        .map(|d| d.to_string())
                                        .unwrap_or_default(),
                                    result: record.pass_fail.clone(),
                                    measurements,
                                });
                            }
                        }
                    }
                }
            }
        }
    } else {
        // Batch mode: filter by batch and optionally selected dates
        let mut query = voltech_test_results::Entity::find().filter(
            voltech_test_results::Column::Measurements
                .contains(&format!("\"{}\"", associated_test)),
        );

        if let Some(batch_val) = batch {
            query = query.filter(voltech_test_results::Column::Batch.eq(batch_val));
        }

        let records = query.all(voltech_db).await?;

        // Filter by selected dates if provided
        for record in records {
            let date_str = record
                .normalized_date
                .map(|d| d.to_string())
                .unwrap_or_default();

            let include = if let Some(dates) = selected_dates {
                dates.contains(&date_str)
            } else {
                true
            };

            if include {
                // Parse measurements JSON
                let measurements: serde_json::Value =
                    serde_json::from_str(&record.measurements).unwrap_or(serde_json::json!({}));

                results.push(SingleTestResult {
                    serial_number: Some(record.serial_num.clone()),
                    batch: record.batch.clone(),
                    date: date_str,
                    result: record.pass_fail.clone(),
                    measurements,
                });
            }
        }
    }

    Ok(TestResultData {
        test_id: test.id,
        test_name: test.test_type.clone(),
        source_type: test.source_type.clone(),
        associated_test: test.associated_test.clone(),
        spec_min: test.minimum,
        spec_max: test.maximum,
        spec_unit: Some(test.uo_m.clone()),
        results,
    })
}

/// Collect manual test results
async fn collect_manual_test(
    test: &test_entity::Model,
    is_serialized: bool,
    batch: Option<&str>,
    serial_range: Option<&str>,
    selected_dates: Option<&Vec<String>>,
    manual_db: &DbConn,
) -> Result<TestResultData, DbErr> {
    let associated_test = test.associated_test.as_deref().unwrap_or("");
    let mut results = Vec::new();

    if is_serialized {
        // Serialized mode: filter by serial range
        if let Some(range) = serial_range {
            // Parse serial range (e.g., "1001-1010")
            if let Some((start_str, end_str)) = range.split_once('-') {
                if let (Ok(start), Ok(end)) = (start_str.parse::<i32>(), end_str.parse::<i32>()) {
                    let records = manual_test_results::Entity::find()
                        .filter(manual_test_results::Column::Test.eq(associated_test))
                        .all(manual_db)
                        .await?;

                    for record in records {
                        // Check if serial is within range
                        if let Ok(serial_num) = record.sn.parse::<i32>() {
                            if serial_num >= start && serial_num <= end {
                                // Create measurements JSON from manual test data
                                let measurements = serde_json::json!({
                                    "minimum": record.minimum,
                                    "reading": record.reading,
                                    "maximum": record.maximum,
                                    "uom": record.uom
                                });

                                results.push(SingleTestResult {
                                    serial_number: Some(record.sn.clone()),
                                    batch: record.batch.clone(),
                                    date: record.normalized_date.to_string(),
                                    result: record.passfail.clone(),
                                    measurements,
                                });
                            }
                        }
                    }
                }
            }
        }
    } else {
        // Batch mode: filter by batch and optionally selected dates
        let mut query = manual_test_results::Entity::find()
            .filter(manual_test_results::Column::Test.eq(associated_test));

        if let Some(batch_val) = batch {
            query = query.filter(manual_test_results::Column::Batch.eq(batch_val));
        }

        let records = query.all(manual_db).await?;

        // Filter by selected dates if provided
        for record in records {
            let date_str = record.normalized_date.to_string();

            let include = if let Some(dates) = selected_dates {
                dates.contains(&date_str)
            } else {
                true
            };

            if include {
                // Create measurements JSON from manual test data
                let measurements = serde_json::json!({
                    "minimum": record.minimum,
                    "reading": record.reading,
                    "maximum": record.maximum,
                    "uom": record.uom
                });

                results.push(SingleTestResult {
                    serial_number: Some(record.sn.clone()),
                    batch: record.batch.clone(),
                    date: date_str,
                    result: record.passfail.clone(),
                    measurements,
                });
            }
        }
    }

    Ok(TestResultData {
        test_id: test.id,
        test_name: test.test_type.clone(),
        source_type: test.source_type.clone(),
        associated_test: test.associated_test.clone(),
        spec_min: test.minimum,
        spec_max: test.maximum,
        spec_unit: Some(test.uo_m.clone()),
        results,
    })
}

// ============================================================================
// Tauri Command
// ============================================================================

#[tauri::command]
pub async fn collect_report(
    report_id: i32,
    batch: Option<String>,
    serial_range: Option<String>,
    selected_dates: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<ReportData, String> {
    match collect_report_data(
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
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to collect report data: {}", e)),
    }
}
