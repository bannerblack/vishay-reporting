use crate::AppState;
use entity::{fg, report, test};
use sea_orm::sea_query::ExprTrait;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Serialize;
use tauri::State;

// Re-export response types from other modules
use crate::fg::FGResponse;
use crate::report::ReportResponse;
use crate::test::TestResponse;

// ============================================================================
// Join Response DTOs
// ============================================================================

#[derive(Debug, Serialize)]
pub struct FGWithReportsResponse {
    pub id: i32,
    pub fg: String,
    pub rev: String,
    pub customer: String,
    pub reports: Vec<ReportResponse>,
}

#[derive(Debug, Serialize)]
pub struct FGWithTestsResponse {
    pub id: i32,
    pub fg: String,
    pub rev: String,
    pub customer: String,
    pub tests: Vec<TestResponse>,
}

#[derive(Debug, Serialize)]
pub struct FGCompleteResponse {
    pub id: i32,
    pub fg: String,
    pub rev: String,
    pub customer: String,
    pub reports: Vec<ReportResponse>,
    pub tests: Vec<TestResponse>,
}

#[derive(Debug, Serialize)]
pub struct ReportWithFGResponse {
    pub id: i32,
    pub fg_id: i32,
    pub attributes: String,
    pub fg: FGResponse,
}

#[derive(Debug, Serialize)]
pub struct ReportWithTestsResponse {
    pub id: i32,
    pub fg_id: i32,
    pub attributes: String,
    pub tests: Vec<TestResponse>,
}

#[derive(Debug, Serialize)]
pub struct ReportCompleteResponse {
    pub id: i32,
    pub fg_id: i32,
    pub attributes: String,
    pub fg: FGResponse,
    pub tests: Vec<TestResponse>,
}

#[derive(Debug, Serialize)]
pub struct TestWithFGResponse {
    pub id: i32,
    pub report_id: Option<i32>,
    pub test_type: String,
    pub frequency: Option<f64>,
    pub voltage: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub uo_m: String,
    pub fg: FGResponse,
}

#[derive(Debug, Serialize)]
pub struct TestWithReportResponse {
    pub id: i32,
    pub fg_id: i32,
    pub test_type: String,
    pub frequency: Option<f64>,
    pub voltage: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub uo_m: String,
    pub report: Option<ReportResponse>,
}

#[derive(Debug, Serialize)]
pub struct TestCompleteResponse {
    pub id: i32,
    pub test_type: String,
    pub frequency: Option<f64>,
    pub voltage: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub uo_m: String,
    pub fg: FGResponse,
    pub report: Option<ReportResponse>,
}

// ============================================================================
// FG Join Operations
// ============================================================================

#[tauri::command]
pub async fn get_fg_with_reports(
    state: State<'_, AppState>,
    fg_id: i32,
) -> Result<FGWithReportsResponse, String> {
    let db = &*state.core_db;

    let fg_model = fg::Entity::find_by_id(fg_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    let reports = fg_model
        .find_related(report::Entity)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch reports: {}", e))?;

    Ok(FGWithReportsResponse {
        id: fg_model.id,
        fg: fg_model.fg,
        rev: fg_model.rev,
        customer: fg_model.customer,
        reports: reports
            .into_iter()
            .map(|r| ReportResponse {
                id: r.id,
                fg_id: r.fg_id,
                attributes: r.attributes,
            })
            .collect(),
    })
}

#[tauri::command]
pub async fn get_fg_with_tests(
    state: State<'_, AppState>,
    fg_id: i32,
) -> Result<FGWithTestsResponse, String> {
    let db = &*state.core_db;

    let fg_model = fg::Entity::find_by_id(fg_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    let tests = fg_model
        .find_related(test::Entity)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(FGWithTestsResponse {
        id: fg_model.id,
        fg: fg_model.fg,
        rev: fg_model.rev,
        customer: fg_model.customer,
        tests: tests
            .into_iter()
            .map(|t| TestResponse {
                id: t.id,
                report_id: t.report_id,
                fg_id: t.fg_id,
                test_type: t.test_type,
                frequency: t.frequency,
                voltage: t.voltage,
                minimum: t.minimum,
                maximum: t.maximum,
                uo_m: t.uo_m,
                primary_pins: t.primary_pins,
                secondary_pins: t.secondary_pins,
                shorted_pins: t.shorted_pins,
                description: t.description,
                created_at: t.created_at.to_string(),
                updated_at: t.updated_at.to_string(),
                order: t.order,
                source_type: t.source_type,
                associated_test: t.associated_test,
                manual_override: t.manual_override,
            })
            .collect(),
    })
}

#[tauri::command]
pub async fn get_fg_complete(
    state: State<'_, AppState>,
    fg_id: i32,
) -> Result<FGCompleteResponse, String> {
    let db = &*state.core_db;

    let fg_model = fg::Entity::find_by_id(fg_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    let reports = fg_model
        .find_related(report::Entity)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch reports: {}", e))?;

    let tests = fg_model
        .find_related(test::Entity)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(FGCompleteResponse {
        id: fg_model.id,
        fg: fg_model.fg,
        rev: fg_model.rev,
        customer: fg_model.customer,
        reports: reports
            .into_iter()
            .map(|r| ReportResponse {
                id: r.id,
                fg_id: r.fg_id,
                attributes: r.attributes,
            })
            .collect(),
        tests: tests
            .into_iter()
            .map(|t| TestResponse {
                id: t.id,
                report_id: t.report_id,
                fg_id: t.fg_id,
                test_type: t.test_type,
                frequency: t.frequency,
                voltage: t.voltage,
                minimum: t.minimum,
                maximum: t.maximum,
                uo_m: t.uo_m,
                primary_pins: t.primary_pins,
                secondary_pins: t.secondary_pins,
                shorted_pins: t.shorted_pins,
                description: t.description,
                created_at: t.created_at.to_string(),
                updated_at: t.updated_at.to_string(),
                order: t.order,
                source_type: t.source_type,
                associated_test: t.associated_test,
                manual_override: t.manual_override,
            })
            .collect(),
    })
}

// ============================================================================
// Report Join Operations
// ============================================================================

#[tauri::command]
pub async fn get_report_with_fg(
    state: State<'_, AppState>,
    report_id: i32,
) -> Result<ReportWithFGResponse, String> {
    let db = &*state.core_db;

    let report_model = report::Entity::find_by_id(report_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch report: {}", e))?
        .ok_or_else(|| "Report not found".to_string())?;

    let fg_model = fg::Entity::find_by_id(report_model.fg_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    Ok(ReportWithFGResponse {
        id: report_model.id,
        fg_id: report_model.fg_id,
        attributes: report_model.attributes,
        fg: FGResponse {
            id: fg_model.id,
            fg: fg_model.fg,
            rev: fg_model.rev,
            customer: fg_model.customer,
            serialized: fg_model.serialized,
        },
    })
}

#[tauri::command]
pub async fn get_report_with_tests(
    state: State<'_, AppState>,
    report_id: i32,
) -> Result<ReportWithTestsResponse, String> {
    let db = &*state.core_db;

    let report_model = report::Entity::find_by_id(report_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch report: {}", e))?
        .ok_or_else(|| "Report not found".to_string())?;

    let tests = report_model
        .find_related(test::Entity)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(ReportWithTestsResponse {
        id: report_model.id,
        fg_id: report_model.fg_id,
        attributes: report_model.attributes,
        tests: tests
            .into_iter()
            .map(|t| TestResponse {
                id: t.id,
                report_id: t.report_id,
                fg_id: t.fg_id,
                test_type: t.test_type,
                frequency: t.frequency,
                voltage: t.voltage,
                minimum: t.minimum,
                maximum: t.maximum,
                uo_m: t.uo_m,
                primary_pins: t.primary_pins,
                secondary_pins: t.secondary_pins,
                shorted_pins: t.shorted_pins,
                description: t.description,
                created_at: t.created_at.to_string(),
                updated_at: t.updated_at.to_string(),
                order: t.order,
                source_type: t.source_type,
                associated_test: t.associated_test,
                manual_override: t.manual_override,
            })
            .collect(),
    })
}

#[tauri::command]
pub async fn get_report_complete(
    state: State<'_, AppState>,
    report_id: i32,
) -> Result<ReportCompleteResponse, String> {
    let db = &*state.core_db;

    let report_model = report::Entity::find_by_id(report_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch report: {}", e))?
        .ok_or_else(|| "Report not found".to_string())?;

    let fg_model = fg::Entity::find_by_id(report_model.fg_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    let tests = report_model
        .find_related(test::Entity)
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(ReportCompleteResponse {
        id: report_model.id,
        fg_id: report_model.fg_id,
        attributes: report_model.attributes,
        fg: FGResponse {
            id: fg_model.id,
            fg: fg_model.fg,
            rev: fg_model.rev,
            customer: fg_model.customer,
            serialized: fg_model.serialized,
        },
        tests: tests
            .into_iter()
            .map(|t| TestResponse {
                id: t.id,
                report_id: t.report_id,
                fg_id: t.fg_id,
                test_type: t.test_type,
                frequency: t.frequency,
                voltage: t.voltage,
                minimum: t.minimum,
                maximum: t.maximum,
                uo_m: t.uo_m,
                primary_pins: t.primary_pins,
                secondary_pins: t.secondary_pins,
                shorted_pins: t.shorted_pins,
                description: t.description,
                created_at: t.created_at.to_string(),
                updated_at: t.updated_at.to_string(),
                order: t.order,
                source_type: t.source_type,
                associated_test: t.associated_test,
                manual_override: t.manual_override,
            })
            .collect(),
    })
}

// ============================================================================
// Test Join Operations
// ============================================================================

#[tauri::command]
pub async fn get_test_with_fg(
    state: State<'_, AppState>,
    test_id: i32,
) -> Result<TestWithFGResponse, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(test_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let fg_model = fg::Entity::find_by_id(test_model.fg_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    Ok(TestWithFGResponse {
        id: test_model.id,
        report_id: test_model.report_id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        fg: FGResponse {
            id: fg_model.id,
            fg: fg_model.fg,
            rev: fg_model.rev,
            customer: fg_model.customer,
            serialized: fg_model.serialized,
        },
    })
}

#[tauri::command]
pub async fn get_test_with_report(
    state: State<'_, AppState>,
    test_id: i32,
) -> Result<TestWithReportResponse, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(test_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let report_response = if let Some(report_id) = test_model.report_id {
        let report_model = report::Entity::find_by_id(report_id)
            .one(db)
            .await
            .map_err(|e| format!("Failed to fetch report: {}", e))?;

        report_model.map(|r| ReportResponse {
            id: r.id,
            fg_id: r.fg_id,
            attributes: r.attributes,
        })
    } else {
        None
    };

    Ok(TestWithReportResponse {
        id: test_model.id,
        fg_id: test_model.fg_id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        report: report_response,
    })
}

#[tauri::command]
pub async fn get_test_complete(
    state: State<'_, AppState>,
    test_id: i32,
) -> Result<TestCompleteResponse, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(test_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let fg_model = fg::Entity::find_by_id(test_model.fg_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    let report_response = if let Some(report_id) = test_model.report_id {
        let report_model = report::Entity::find_by_id(report_id)
            .one(db)
            .await
            .map_err(|e| format!("Failed to fetch report: {}", e))?;

        report_model.map(|r| ReportResponse {
            id: r.id,
            fg_id: r.fg_id,
            attributes: r.attributes,
        })
    } else {
        None
    };

    Ok(TestCompleteResponse {
        id: test_model.id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        fg: FGResponse {
            id: fg_model.id,
            fg: fg_model.fg,
            rev: fg_model.rev,
            customer: fg_model.customer,
            serialized: fg_model.serialized,
        },
        report: report_response,
    })
}

// ============================================================================
// List Operations with Joins
// ============================================================================

#[tauri::command]
pub async fn get_all_reports_with_fg(
    state: State<'_, AppState>,
) -> Result<Vec<ReportWithFGResponse>, String> {
    let db = &*state.core_db;

    let reports = report::Entity::find()
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch reports: {}", e))?;

    let mut result = Vec::new();

    for report_model in reports {
        let fg_model = fg::Entity::find_by_id(report_model.fg_id)
            .one(db)
            .await
            .map_err(|e| format!("Failed to fetch FG: {}", e))?
            .ok_or_else(|| format!("FG {} not found", report_model.fg_id))?;

        result.push(ReportWithFGResponse {
            id: report_model.id,
            fg_id: report_model.fg_id,
            attributes: report_model.attributes,
            fg: FGResponse {
                id: fg_model.id,
                fg: fg_model.fg,
                rev: fg_model.rev,
                customer: fg_model.customer,
                serialized: fg_model.serialized,
            },
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_all_tests_by_fg(
    state: State<'_, AppState>,
    fg_id: i32,
) -> Result<Vec<TestResponse>, String> {
    let db = &*state.core_db;

    let tests = test::Entity::find()
        .filter(test::Column::FgId.eq(fg_id))
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(tests
        .into_iter()
        .map(|t| TestResponse {
            id: t.id,
            report_id: t.report_id,
            fg_id: t.fg_id,
            test_type: t.test_type,
            frequency: t.frequency,
            voltage: t.voltage,
            minimum: t.minimum,
            maximum: t.maximum,
            uo_m: t.uo_m,
            primary_pins: t.primary_pins,
            secondary_pins: t.secondary_pins,
            shorted_pins: t.shorted_pins,
            description: t.description,
            created_at: t.created_at.to_string(),
            updated_at: t.updated_at.to_string(),
            order: t.order,
            source_type: t.source_type,
            associated_test: t.associated_test,
            manual_override: t.manual_override,
        })
        .collect())
}

#[tauri::command]
pub async fn get_all_tests_by_report(
    state: State<'_, AppState>,
    report_id: i32,
) -> Result<Vec<TestResponse>, String> {
    let db = &*state.core_db;

    let tests = test::Entity::find()
        .filter(test::Column::ReportId.eq(report_id))
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(tests
        .into_iter()
        .map(|t| TestResponse {
            id: t.id,
            report_id: t.report_id,
            fg_id: t.fg_id,
            test_type: t.test_type,
            frequency: t.frequency,
            voltage: t.voltage,
            minimum: t.minimum,
            maximum: t.maximum,
            uo_m: t.uo_m,
            primary_pins: t.primary_pins,
            secondary_pins: t.secondary_pins,
            shorted_pins: t.shorted_pins,
            description: t.description,
            created_at: t.created_at.to_string(),
            updated_at: t.updated_at.to_string(),
            order: t.order,
            source_type: t.source_type,
            associated_test: t.associated_test,
            manual_override: t.manual_override,
        })
        .collect())
}

// Get tests available for a report (tests from the same FG not yet assigned to this report)
#[tauri::command]
pub async fn get_available_tests_for_report(
    state: State<'_, AppState>,
    report_id: i32,
) -> Result<Vec<TestResponse>, String> {
    let db = &*state.core_db;

    // Get the report to find its FG
    let report_model = report::Entity::find_by_id(report_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch report: {}", e))?
        .ok_or_else(|| "Report not found".to_string())?;

    // Get all tests for this FG that are NOT assigned to this report
    let tests = test::Entity::find()
        .filter(test::Column::FgId.eq(report_model.fg_id))
        .filter(
            test::Column::ReportId
                .is_null()
                .or(test::Column::ReportId.ne(report_id)),
        )
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(tests
        .into_iter()
        .map(|t| TestResponse {
            id: t.id,
            report_id: t.report_id,
            fg_id: t.fg_id,
            test_type: t.test_type,
            frequency: t.frequency,
            voltage: t.voltage,
            minimum: t.minimum,
            maximum: t.maximum,
            uo_m: t.uo_m,
            primary_pins: t.primary_pins,
            secondary_pins: t.secondary_pins,
            shorted_pins: t.shorted_pins,
            description: t.description,
            created_at: t.created_at.to_string(),
            updated_at: t.updated_at.to_string(),
            order: t.order,
            source_type: t.source_type,
            associated_test: t.associated_test,
            manual_override: t.manual_override,
        })
        .collect())
}
