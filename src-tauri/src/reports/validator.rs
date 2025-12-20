use crate::AppState;
use ::entity::test;
use entity_manual::manual_test_results;
use entity_voltech::test_results as voltech_test_results;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tauri::State;

/// Status of a single test in the validation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestStatus {
    pub test_id: i32,
    pub test_name: String,
    pub source_type: String,
    pub associated_test: Option<String>,
    pub has_data: bool,
    pub record_count: i32,
    pub available_sessions: Vec<AvailableSession>,
    pub search_method: String, // "serial_range" or "batch"
}

/// Represents an available test session (for batch mode)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvailableSession {
    pub date: String, // NaiveDate as string "YYYY-MM-DD"
    pub batch: String,
    pub record_count: i32,
}

/// Result of report validation
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_complete: bool,
    pub fg_serialized: bool,
    pub test_statuses: Vec<TestStatus>,
    pub missing_test_count: i32,
}

/// Validate report data availability
///
/// # Arguments
/// * `report_id` - Report ID to validate
/// * `batch` - Optional batch identifier (for batch mode)
/// * `serial_range` - Optional serial range (for serialized mode, e.g., "1001-1010")
/// * `core_db` - Core database connection
/// * `voltech_db` - Voltech database connection
/// * `manual_db` - Manual database connection
pub async fn validate_report_data(
    report_id: i32,
    batch: Option<String>,
    serial_range: Option<String>,
    core_db: &DbConn,
    voltech_db: &DbConn,
    manual_db: &DbConn,
) -> Result<ValidationResult, DbErr> {
    // Get all tests for this report
    let tests = test::Entity::find()
        .filter(test::Column::ReportId.eq(report_id))
        .order_by_asc(test::Column::Order)
        .all(core_db)
        .await?;

    if tests.is_empty() {
        return Ok(ValidationResult {
            is_complete: false,
            fg_serialized: false,
            test_statuses: vec![],
            missing_test_count: 0,
        });
    }

    // Get FG to determine if it's serialized
    let first_test = &tests[0];
    let fg = ::entity::fg::Entity::find_by_id(first_test.fg_id)
        .one(core_db)
        .await?
        .ok_or(DbErr::RecordNotFound("FG not found".to_string()))?;

    let is_serialized = fg.serialized;
    let search_method = if is_serialized {
        "serial_range"
    } else {
        "batch"
    };

    let mut test_statuses = Vec::new();
    let mut missing_count = 0;

    for test_model in tests {
        let status = if test_model.source_type == "voltech" {
            validate_voltech_test(
                &test_model,
                &fg.fg,
                &batch,
                &serial_range,
                is_serialized,
                voltech_db,
            )
            .await?
        } else if test_model.source_type == "manual" {
            validate_manual_test(&test_model, &batch, &serial_range, is_serialized, manual_db)
                .await?
        } else {
            // "other" source type - no validation needed, always has data
            TestStatus {
                test_id: test_model.id,
                test_name: test_model.test_type.clone(),
                source_type: test_model.source_type.clone(),
                associated_test: test_model.associated_test.clone(),
                has_data: true,
                record_count: 1,
                available_sessions: vec![],
                search_method: search_method.to_string(),
            }
        };

        if !status.has_data {
            missing_count += 1;
        }

        test_statuses.push(status);
    }

    Ok(ValidationResult {
        is_complete: missing_count == 0,
        fg_serialized: is_serialized,
        test_statuses,
        missing_test_count: missing_count,
    })
}

/// Validate a voltech test
async fn validate_voltech_test(
    test_model: &test::Model,
    fg_number: &str,
    batch: &Option<String>,
    serial_range: &Option<String>,
    is_serialized: bool,
    voltech_db: &DbConn,
) -> Result<TestStatus, DbErr> {
    let associated_test = test_model.associated_test.as_deref().unwrap_or("");

    if is_serialized {
        // Serialized mode: check if records exist for serial numbers in range
        let (has_data, record_count) = if let Some(range) = serial_range {
            // Parse serial range (e.g., "1001-1010")
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    // Query for passing tests with matching FG (starts_with for FG+type+rev), serial range, and measurement key
                    let count = voltech_test_results::Entity::find()
                        .filter(voltech_test_results::Column::Part.starts_with(fg_number))
                        .filter(voltech_test_results::Column::SerialNum.between(start.to_string(), end.to_string()))
                        .filter(voltech_test_results::Column::PassFail.eq("Pass"))
                        .filter(
                            voltech_test_results::Column::Measurements
                                .contains(&format!("\"{}\"", associated_test)),
                        )
                        .count(voltech_db)
                        .await?;
                    (count > 0, count as i32)
                } else {
                    (false, 0)
                }
            } else {
                (false, 0)
            }
        } else {
            (false, 0)
        };

        Ok(TestStatus {
            test_id: test_model.id,
            test_name: test_model.test_type.clone(),
            source_type: test_model.source_type.clone(),
            associated_test: test_model.associated_test.clone(),
            has_data,
            record_count,
            available_sessions: vec![],
            search_method: "serial_range".to_string(),
        })
    } else {
        // Batch mode: find available test sessions grouped by date
        let results = voltech_test_results::Entity::find()
            .filter(voltech_test_results::Column::Part.starts_with(fg_number))
            .filter(voltech_test_results::Column::PassFail.eq("Pass"))
            .filter(
                voltech_test_results::Column::Measurements
                    .contains(&format!("\"{}\"", associated_test)),
            )
            .all(voltech_db)
            .await?;

        // Group by normalized_date and batch
        let mut sessions_map: std::collections::HashMap<(String, String), i32> =
            std::collections::HashMap::new();

        for result in results {
            if let Some(date) = result.normalized_date {
                let key = (date.to_string(), result.batch.clone());
                *sessions_map.entry(key).or_insert(0) += 1;
            }
        }

        let mut available_sessions: Vec<AvailableSession> = sessions_map
            .into_iter()
            .map(|((date, batch_name), count)| AvailableSession {
                date,
                batch: batch_name,
                record_count: count,
            })
            .collect();

        // Sort by date descending
        available_sessions.sort_by(|a, b| b.date.cmp(&a.date));

        // Check if specific batch has data
        let has_data = if let Some(batch_filter) = batch {
            available_sessions.iter().any(|s| &s.batch == batch_filter)
        } else {
            !available_sessions.is_empty()
        };

        let record_count = if let Some(batch_filter) = batch {
            available_sessions
                .iter()
                .find(|s| &s.batch == batch_filter)
                .map(|s| s.record_count)
                .unwrap_or(0)
        } else {
            available_sessions.iter().map(|s| s.record_count).sum()
        };

        Ok(TestStatus {
            test_id: test_model.id,
            test_name: test_model.test_type.clone(),
            source_type: test_model.source_type.clone(),
            associated_test: test_model.associated_test.clone(),
            has_data,
            record_count,
            available_sessions,
            search_method: "batch".to_string(),
        })
    }
}

/// Validate a manual test
async fn validate_manual_test(
    test_model: &test::Model,
    batch: &Option<String>,
    serial_range: &Option<String>,
    is_serialized: bool,
    manual_db: &DbConn,
) -> Result<TestStatus, DbErr> {
    let associated_test = test_model.associated_test.as_deref().unwrap_or("");

    if is_serialized {
        // Serialized mode: check if records exist for serial numbers in range
        let (has_data, record_count) = if let Some(_range) = serial_range {
            // For manual tests, we need serial numbers from the data
            let count = manual_test_results::Entity::find()
                .filter(manual_test_results::Column::Test.eq(associated_test))
                .count(manual_db)
                .await?;
            (count > 0, count as i32)
        } else {
            (false, 0)
        };

        Ok(TestStatus {
            test_id: test_model.id,
            test_name: test_model.test_type.clone(),
            source_type: test_model.source_type.clone(),
            associated_test: test_model.associated_test.clone(),
            has_data,
            record_count,
            available_sessions: vec![],
            search_method: "serial_range".to_string(),
        })
    } else {
        // Batch mode: find available test sessions grouped by date
        let results = manual_test_results::Entity::find()
            .filter(manual_test_results::Column::Test.eq(associated_test))
            .all(manual_db)
            .await?;

        // Group by normalized_date and batch (manual has Date, not Option<Date>)
        let mut sessions_map: std::collections::HashMap<(String, String), i32> =
            std::collections::HashMap::new();

        for result in results {
            let key = (result.normalized_date.to_string(), result.batch.clone());
            *sessions_map.entry(key).or_insert(0) += 1;
        }

        let mut available_sessions: Vec<AvailableSession> = sessions_map
            .into_iter()
            .map(|((date, batch_name), count)| AvailableSession {
                date,
                batch: batch_name,
                record_count: count,
            })
            .collect();

        // Sort by date descending
        available_sessions.sort_by(|a, b| b.date.cmp(&a.date));

        // Check if specific batch has data
        let has_data = if let Some(batch_filter) = batch {
            available_sessions.iter().any(|s| &s.batch == batch_filter)
        } else {
            !available_sessions.is_empty()
        };

        let record_count = if let Some(batch_filter) = batch {
            available_sessions
                .iter()
                .find(|s| &s.batch == batch_filter)
                .map(|s| s.record_count)
                .unwrap_or(0)
        } else {
            available_sessions.iter().map(|s| s.record_count).sum()
        };

        Ok(TestStatus {
            test_id: test_model.id,
            test_name: test_model.test_type.clone(),
            source_type: test_model.source_type.clone(),
            associated_test: test_model.associated_test.clone(),
            has_data,
            record_count,
            available_sessions,
            search_method: "batch".to_string(),
        })
    }
}

// ============================================================================
// Tauri Command
// ============================================================================

#[tauri::command]
pub async fn validate_report(
    report_id: i32,
    batch: Option<String>,
    serial_range: Option<String>,
    state: State<'_, AppState>,
) -> Result<ValidationResult, String> {
    match validate_report_data(
        report_id,
        batch,
        serial_range,
        &state.core_db,
        &state.voltech_db,
        &state.manual_db,
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to validate report: {}", e)),
    }
}
