use sea_orm::{Set, ActiveModelTrait, EntityTrait};
use entity::test;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

// ============================================================================
// DTOs
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct TestData {
    pub report_id: Option<i32>,
    pub fg_id: i32,
    pub test_type: String,
    pub frequency: Option<f64>,
    pub voltage: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub uo_m: String,
    pub primary_pins: Option<String>,
    pub secondary_pins: Option<String>,
    pub shorted_pins: Option<String>,
    pub description: Option<String>,
    pub added_by: Option<i32>,
    pub order: i32,
    pub source_type: String,
    pub associated_test: Option<String>,
    pub manual_override: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TestResponse {
    pub id: i32,
    pub report_id: Option<i32>,
    pub fg_id: i32,
    pub test_type: String,
    pub frequency: Option<f64>,
    pub voltage: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub uo_m: String,
    pub primary_pins: Option<String>,
    pub secondary_pins: Option<String>,
    pub shorted_pins: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub order: i32,
    pub source_type: String,
    pub associated_test: Option<String>,
    pub manual_override: Option<bool>,
}

// ============================================================================
// CRUD Operations
// ============================================================================

#[tauri::command]
pub async fn create_test(state: State<'_, AppState>, test_data: TestData) -> Result<TestResponse, String> {
    let db = &*state.core_db;

    let test_model = test::ActiveModel {
        report_id: Set(test_data.report_id),
        fg_id: Set(test_data.fg_id),
        test_type: Set(test_data.test_type),
        frequency: Set(test_data.frequency),
        voltage: Set(test_data.voltage),
        minimum: Set(test_data.minimum),
        maximum: Set(test_data.maximum),
        uo_m: Set(test_data.uo_m),
        primary_pins: Set(test_data.primary_pins),
        secondary_pins: Set(test_data.secondary_pins),
        shorted_pins: Set(test_data.shorted_pins),
        description: Set(test_data.description),
        added_by: Set(test_data.added_by),
        order: Set(test_data.order),
        source_type: Set(test_data.source_type),
        associated_test: Set(test_data.associated_test),
        manual_override: Set(test_data.manual_override),
        ..Default::default()
    };

    let test_model: test::Model = test_model
        .insert(db)
        .await
        .map_err(|e| format!("Failed to create test: {}", e))?;

    Ok(TestResponse {
        id: test_model.id,
        report_id: test_model.report_id,
        fg_id: test_model.fg_id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        primary_pins: test_model.primary_pins,
        secondary_pins: test_model.secondary_pins,
        shorted_pins: test_model.shorted_pins,
        description: test_model.description,
        created_at: test_model.created_at.to_string(),
        updated_at: test_model.updated_at.to_string(),
        order: test_model.order,
        source_type: test_model.source_type,
        associated_test: test_model.associated_test,
        manual_override: test_model.manual_override,
    })
}

#[tauri::command]
pub async fn get_test(state: State<'_, AppState>, id: i32) -> Result<TestResponse, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    Ok(TestResponse {
        id: test_model.id,
        report_id: test_model.report_id,
        fg_id: test_model.fg_id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        primary_pins: test_model.primary_pins,
        secondary_pins: test_model.secondary_pins,
        shorted_pins: test_model.shorted_pins,
        description: test_model.description,
        created_at: test_model.created_at.to_string(),
        updated_at: test_model.updated_at.to_string(),
        order: test_model.order,
        source_type: test_model.source_type,
        associated_test: test_model.associated_test,
        manual_override: test_model.manual_override,
    })
}

#[tauri::command]
pub async fn get_all_tests(state: State<'_, AppState>) -> Result<Vec<TestResponse>, String> {
    let db = &*state.core_db;

    let tests = test::Entity::find()
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch tests: {}", e))?;

    Ok(tests
        .into_iter()
        .map(|test_model| TestResponse {
            id: test_model.id,
            report_id: test_model.report_id,
            fg_id: test_model.fg_id,
            test_type: test_model.test_type,
            frequency: test_model.frequency,
            voltage: test_model.voltage,
            minimum: test_model.minimum,
            maximum: test_model.maximum,
            uo_m: test_model.uo_m,
            primary_pins: test_model.primary_pins,
            secondary_pins: test_model.secondary_pins,
            shorted_pins: test_model.shorted_pins,
            description: test_model.description,
            created_at: test_model.created_at.to_string(),
            updated_at: test_model.updated_at.to_string(),
            order: test_model.order,
            source_type: test_model.source_type,
            associated_test: test_model.associated_test,
            manual_override: test_model.manual_override,
        })
        .collect())
}

#[tauri::command]
pub async fn update_test(state: State<'_, AppState>, id: i32, test_data: TestData) -> Result<TestResponse, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let mut test_model: test::ActiveModel = test_model.into();
    test_model.report_id = Set(test_data.report_id);
    test_model.fg_id = Set(test_data.fg_id);
    test_model.test_type = Set(test_data.test_type);
    test_model.frequency = Set(test_data.frequency);
    test_model.voltage = Set(test_data.voltage);
    test_model.minimum = Set(test_data.minimum);
    test_model.maximum = Set(test_data.maximum);
    test_model.uo_m = Set(test_data.uo_m);
    test_model.primary_pins = Set(test_data.primary_pins);
    test_model.secondary_pins = Set(test_data.secondary_pins);
    test_model.shorted_pins = Set(test_data.shorted_pins);
    test_model.description = Set(test_data.description);
    test_model.added_by = Set(test_data.added_by);
    test_model.order = Set(test_data.order);
    test_model.source_type = Set(test_data.source_type);
    test_model.associated_test = Set(test_data.associated_test);
    test_model.manual_override = Set(test_data.manual_override);

    let test_model: test::Model = test_model
        .update(db)
        .await
        .map_err(|e| format!("Failed to update test: {}", e))?;

    Ok(TestResponse {
        id: test_model.id,
        report_id: test_model.report_id,
        fg_id: test_model.fg_id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        primary_pins: test_model.primary_pins,
        secondary_pins: test_model.secondary_pins,
        shorted_pins: test_model.shorted_pins,
        description: test_model.description,
        created_at: test_model.created_at.to_string(),
        updated_at: test_model.updated_at.to_string(),
        order: test_model.order,
        source_type: test_model.source_type,
        associated_test: test_model.associated_test,
        manual_override: test_model.manual_override,
    })
}

#[tauri::command]
pub async fn delete_test(state: State<'_, AppState>, id: i32) -> Result<String, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let test_model: test::ActiveModel = test_model.into();
    test_model
        .delete(db)
        .await
        .map_err(|e| format!("Failed to delete test: {}", e))?;

    Ok(format!("Test {} deleted successfully", id))
}

#[tauri::command]
pub async fn assign_test_to_report(state: State<'_, AppState>, test_id: i32, report_id: i32) -> Result<TestResponse, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(test_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let mut test_model: test::ActiveModel = test_model.into();
    test_model.report_id = Set(Some(report_id));

    let test_model: test::Model = test_model
        .update(db)
        .await
        .map_err(|e| format!("Failed to assign test to report: {}", e))?;

    Ok(TestResponse {
        id: test_model.id,
        report_id: test_model.report_id,
        fg_id: test_model.fg_id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        primary_pins: test_model.primary_pins,
        secondary_pins: test_model.secondary_pins,
        shorted_pins: test_model.shorted_pins,
        description: test_model.description,
        created_at: test_model.created_at.to_string(),
        updated_at: test_model.updated_at.to_string(),
        order: test_model.order,
        source_type: test_model.source_type,
        associated_test: test_model.associated_test,
        manual_override: test_model.manual_override,
    })
}

#[tauri::command]
pub async fn unassign_test_from_report(state: State<'_, AppState>, test_id: i32) -> Result<TestResponse, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(test_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let mut test_model: test::ActiveModel = test_model.into();
    test_model.report_id = Set(None);

    let test_model: test::Model = test_model
        .update(db)
        .await
        .map_err(|e| format!("Failed to unassign test from report: {}", e))?;

    Ok(TestResponse {
        id: test_model.id,
        report_id: test_model.report_id,
        fg_id: test_model.fg_id,
        test_type: test_model.test_type,
        frequency: test_model.frequency,
        voltage: test_model.voltage,
        minimum: test_model.minimum,
        maximum: test_model.maximum,
        uo_m: test_model.uo_m,
        primary_pins: test_model.primary_pins,
        secondary_pins: test_model.secondary_pins,
        shorted_pins: test_model.shorted_pins,
        description: test_model.description,
        created_at: test_model.created_at.to_string(),
        updated_at: test_model.updated_at.to_string(),
        order: test_model.order,
        source_type: test_model.source_type,
        associated_test: test_model.associated_test,
        manual_override: test_model.manual_override,
    })
}

// Update test order for bulk reordering
#[tauri::command]
pub async fn update_test_order(state: State<'_, AppState>, test_id: i32, new_order: i32) -> Result<String, String> {
    let db = &*state.core_db;

    let test_model = test::Entity::find_by_id(test_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch test: {}", e))?
        .ok_or_else(|| "Test not found".to_string())?;

    let mut test_model: test::ActiveModel = test_model.into();
    test_model.order = Set(new_order);

    test_model
        .update(db)
        .await
        .map_err(|e| format!("Failed to update test order: {}", e))?;

    Ok(format!("Test {} order updated to {}", test_id, new_order))
}
