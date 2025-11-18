use sea_orm::{Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};
use entity::fg;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

// ============================================================================
// DTOs
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct FGData {
    pub fg: String,
    pub rev: String,
    pub customer: String,
}

#[derive(Debug, Serialize)]
pub struct FGResponse {
    pub id: i32,
    pub fg: String,
    pub rev: String,
    pub customer: String,
}

// ============================================================================
// CRUD Operations
// ============================================================================

#[tauri::command]
pub async fn create_fg(state: State<'_, AppState>, fg_data: FGData) -> Result<FGResponse, String> {
    let db = &*state.core_db;

    // Check if FG already exists
    let existing = fg::Entity::find()
        .filter(fg::Column::Fg.eq(&fg_data.fg))
        .one(&db)
        .await
        .map_err(|e| format!("Failed to check FG existence: {}", e))?;

    if existing.is_some() {
        return Err(format!("FG {} already exists", fg_data.fg));
    }

    let fg_model = fg::ActiveModel {
        fg: Set(fg_data.fg),
        rev: Set(fg_data.rev),
        customer: Set(fg_data.customer),
        ..Default::default()
    };

    let fg_model: fg::Model = fg_model
        .insert(&db)
        .await
        .map_err(|e| format!("Failed to create FG: {}", e))?;

    Ok(FGResponse {
        id: fg_model.id,
        fg: fg_model.fg,
        rev: fg_model.rev,
        customer: fg_model.customer,
    })
}

#[tauri::command]
pub async fn get_fg(state: State<'_, AppState>, id: i32) -> Result<FGResponse, String> {
    let db = &*state.core_db;

    let fg_model = fg::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    Ok(FGResponse {
        id: fg_model.id,
        fg: fg_model.fg,
        rev: fg_model.rev,
        customer: fg_model.customer,
    })
}

#[tauri::command]
pub async fn get_fg_by_number(state: State<'_, AppState>, fg_number: String) -> Result<FGResponse, String> {
    let db = &*state.core_db;

    let fg_model = fg::Entity::find()
        .filter(fg::Column::Fg.eq(fg_number))
        .one(&db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    Ok(FGResponse {
        id: fg_model.id,
        fg: fg_model.fg,
        rev: fg_model.rev,
        customer: fg_model.customer,
    })
}

#[tauri::command]
pub async fn get_all_fgs(state: State<'_, AppState>) -> Result<Vec<FGResponse>, String> {
    let db = &*state.core_db;

    let fgs = fg::Entity::find()
        .all(&db)
        .await
        .map_err(|e| format!("Failed to fetch FGs: {}", e))?;

    Ok(fgs
        .into_iter()
        .map(|fg_model| FGResponse {
            id: fg_model.id,
            fg: fg_model.fg,
            rev: fg_model.rev,
            customer: fg_model.customer,
        })
        .collect())
}

#[tauri::command]
pub async fn update_fg(state: State<'_, AppState>, id: i32, fg_data: FGData) -> Result<FGResponse, String> {
    let db = &*state.core_db;

    let fg_model = fg::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    let mut fg_model: fg::ActiveModel = fg_model.into();
    fg_model.fg = Set(fg_data.fg);
    fg_model.rev = Set(fg_data.rev);
    fg_model.customer = Set(fg_data.customer);

    let fg_model: fg::Model = fg_model
        .update(&db)
        .await
        .map_err(|e| format!("Failed to update FG: {}", e))?;

    Ok(FGResponse {
        id: fg_model.id,
        fg: fg_model.fg,
        rev: fg_model.rev,
        customer: fg_model.customer,
    })
}

#[tauri::command]
pub async fn delete_fg(state: State<'_, AppState>, id: i32) -> Result<String, String> {
    let db = &*state.core_db;

    let fg_model = fg::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| format!("Failed to fetch FG: {}", e))?
        .ok_or_else(|| "FG not found".to_string())?;

    let fg_model: fg::ActiveModel = fg_model.into();
    fg_model
        .delete(&db)
        .await
        .map_err(|e| format!("Failed to delete FG: {}", e))?;

    Ok(format!("FG {} deleted successfully", id))
}
