use sea_orm::{Set, ActiveModelTrait, EntityTrait};
use entity::user;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

// ============================================================================
// DTOs
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub name: String,
    pub username: String,
    pub preferences: String,
    pub permissions: String,
    pub added_by: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub preferences: String,
    pub permissions: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPreferences {
    pub theme: String,        // "light", "dark", "blue", "green", "system"
    pub language: String,     // "en", "es", "de", etc.
    pub notifications: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            language: "en".to_string(),
            notifications: true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesData {
    pub preferences: UserPreferences,
}

// ============================================================================
// CRUD Operations
// ============================================================================

use sea_orm::DatabaseConnection;

// Logic functions (testable)
pub async fn create_user_logic(db: &DatabaseConnection, user_data: UserData) -> Result<UserResponse, String> {
    let user = user::ActiveModel {
        name: Set(user_data.name),
        username: Set(user_data.username),
        preferences: Set(user_data.preferences),
        permissions: Set(user_data.permissions),
        added_by: Set(user_data.added_by),
        ..Default::default()
    };

    let user: user::Model = user
        .insert(db)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

    Ok(UserResponse {
        id: user.id,
        name: user.name,
        username: user.username,
        preferences: user.preferences,
        permissions: user.permissions,
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    })
}

// Tauri command wrappers
#[tauri::command]
pub async fn create_user(state: State<'_, AppState>, user_data: UserData) -> Result<UserResponse, String> {
    let db = &*state.core_db;
    create_user_logic(db, user_data).await
}

#[tauri::command]
pub async fn get_user(state: State<'_, AppState>, id: i32) -> Result<UserResponse, String> {
    let db = &*state.core_db;

    let user = user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    Ok(UserResponse {
        id: user.id,
        name: user.name,
        username: user.username,
        preferences: user.preferences,
        permissions: user.permissions,
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    })
}

#[tauri::command]
pub async fn get_user_by_username(state: State<'_, AppState>, username: String) -> Result<UserResponse, String> {
    use sea_orm::QueryFilter;
    use sea_orm::ColumnTrait;
    
    let db = &*state.core_db;

    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    Ok(UserResponse {
        id: user.id,
        name: user.name,
        username: user.username,
        preferences: user.preferences,
        permissions: user.permissions,
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    })
}

#[tauri::command]
pub async fn get_all_users(state: State<'_, AppState>) -> Result<Vec<UserResponse>, String> {
    let db = &*state.core_db;

    let users = user::Entity::find()
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch users: {}", e))?;

    Ok(users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            name: user.name,
            username: user.username,
            preferences: user.preferences,
            permissions: user.permissions,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        })
        .collect())
}

#[tauri::command]
pub async fn update_user(state: State<'_, AppState>, id: i32, user_data: UserData) -> Result<UserResponse, String> {
    let db = &*state.core_db;

    let user = user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    let mut user: user::ActiveModel = user.into();
    user.name = Set(user_data.name);
    user.username = Set(user_data.username);
    user.preferences = Set(user_data.preferences);
    user.permissions = Set(user_data.permissions);
    user.added_by = Set(user_data.added_by);

    let user: user::Model = user
        .update(db)
        .await
        .map_err(|e| format!("Failed to update user: {}", e))?;

    Ok(UserResponse {
        id: user.id,
        name: user.name,
        username: user.username,
        preferences: user.preferences,
        permissions: user.permissions,
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    })
}

#[tauri::command]
pub async fn delete_user(state: State<'_, AppState>, id: i32) -> Result<String, String> {
    let db = &*state.core_db;

    let user = user::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    let user: user::ActiveModel = user.into();
    user
        .delete(db)
        .await
        .map_err(|e| format!("Failed to delete user: {}", e))?;

    Ok(format!("User {} deleted successfully", id))
}

// ============================================================================
// Preferences Operations
// ============================================================================

#[tauri::command]
pub async fn get_user_preferences(
    state: State<'_, AppState>,
    user_id: i32,
) -> Result<UserPreferences, String> {
    let db = &*state.core_db;
    
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;
    
    let prefs: UserPreferences = serde_json::from_str(&user.preferences)
        .unwrap_or_default();
    
    Ok(prefs)
}

#[tauri::command]
pub async fn update_user_preferences(
    state: State<'_, AppState>,
    user_id: i32,
    preferences_data: UpdatePreferencesData,
) -> Result<UserResponse, String> {
    let db = &*state.core_db;
    
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;
    
    let preferences_json = serde_json::to_string(&preferences_data.preferences)
        .map_err(|e| format!("Failed to serialize preferences: {}", e))?;
    
    let mut user: user::ActiveModel = user.into();
    user.preferences = Set(preferences_json);
    
    let updated_user = user.update(db)
        .await
        .map_err(|e| format!("Failed to update preferences: {}", e))?;
    
    Ok(UserResponse {
        id: updated_user.id,
        name: updated_user.name,
        username: updated_user.username,
        preferences: updated_user.preferences,
        permissions: updated_user.permissions,
        created_at: updated_user.created_at.to_string(),
        updated_at: updated_user.updated_at.to_string(),
    })
}
