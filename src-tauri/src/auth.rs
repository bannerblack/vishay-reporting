use whoami;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};
use entity::user;
use tauri::State;
use crate::AppState;

// Hardcoded admin password for initial setup
const ADMIN_SETUP_PASSWORD: &str = "vishay_admin_2025";

// ============================================================================
// DTOs
// ============================================================================

#[derive(Debug, Serialize)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub preferences: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminSetupRequest {
    pub password: String,
    pub user_data: crate::user::UserData,
}

// ============================================================================
// Authentication Commands
// ============================================================================

/// Get the current Windows/AD user credentials
#[tauri::command]
pub fn get_system_user() -> (String, String) {
    user_credentials()
}

fn user_credentials() -> (String, String) {
    let username = whoami::username();
    let name = whoami::realname();
    (username, name)
}

/// Authenticate the current system user and return their profile from database
#[tauri::command]
pub async fn authenticate_user(state: State<'_, AppState>) -> Result<AuthenticatedUser, String> {
    let (username, _) = user_credentials();
    let db = &*state.core_db;

    // Try to find user in database
    match user::Entity::find()
        .filter(user::Column::Username.eq(&username))
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
    {
        Some(user_model) => {
            // Parse permissions from JSON string
            let permissions: Vec<String> = serde_json::from_str(&user_model.permissions)
                .unwrap_or_else(|_| vec![]);

            Ok(AuthenticatedUser {
                id: user_model.id,
                username: user_model.username,
                name: user_model.name,
                permissions,
                preferences: user_model.preferences,
            })
        }
        None => {
            // User not in database - check if this is initial setup
            let user_count = user::Entity::find()
                .count(db)
                .await
                .map_err(|e| format!("Failed to count users: {}", e))?;

            if user_count == 0 {
                // No users exist - this is initial setup
                Err("INITIAL_SETUP".to_string())
            } else {
                // Users exist but this user is not registered
                Err("USER_NOT_REGISTERED".to_string())
            }
        }
    }
}

/// Check if the database has any users (initial setup needed)
#[tauri::command]
pub async fn needs_initial_setup(state: State<'_, AppState>) -> Result<bool, String> {
    let db = &*state.core_db;

    let user_count = user::Entity::find()
        .count(db)
        .await
        .map_err(|e| format!("Failed to count users: {}", e))?;

    Ok(user_count == 0)
}

/// Validate admin password for initial setup
#[tauri::command]
pub fn validate_admin_password(password: String) -> Result<bool, String> {
    Ok(password == ADMIN_SETUP_PASSWORD)
}

/// Create first admin user during initial setup (requires admin password)
#[tauri::command]
pub async fn create_initial_admin(state: State<'_, AppState>, password: String, user_data: crate::user::UserData) -> Result<crate::user::UserResponse, String> {
    // Validate admin password
    if password != ADMIN_SETUP_PASSWORD {
        return Err("Invalid admin password".to_string());
    }

    // Check if users already exist
    let db = &*state.core_db;

    let user_count = user::Entity::find()
        .count(db)
        .await
        .map_err(|e| format!("Failed to count users: {}", e))?;

    if user_count > 0 {
        return Err("Initial setup already completed".to_string());
    }

    // Create the admin user
    crate::user::create_user_logic(&db, user_data).await
}

/// Admin-only: Create a new user (requires admin permission check on frontend)
#[tauri::command]
pub async fn admin_create_user(
    state: State<'_, AppState>,
    admin_username: String,
    user_data: crate::user::UserData
) -> Result<crate::user::UserResponse, String> {
    // Verify the requester is an admin
    let db = &*state.core_db;

    let admin = user::Entity::find()
        .filter(user::Column::Username.eq(&admin_username))
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch admin user: {}", e))?
        .ok_or_else(|| "Admin user not found".to_string())?;

    // Check if admin has admin permission
    let permissions: Vec<String> = serde_json::from_str(&admin.permissions)
        .unwrap_or_else(|_| vec![]);

    if !permissions.contains(&"admin".to_string()) {
        return Err("Unauthorized: Admin permission required".to_string());
    }

    // Create the new user
    crate::user::create_user_logic(&db, user_data).await
}

/// Check if a user has a specific permission
#[tauri::command]
pub async fn user_has_permission(state: State<'_, AppState>, username: String, permission: String) -> Result<bool, String> {
    let db = &*state.core_db;

    let user_model = user::Entity::find()
        .filter(user::Column::Username.eq(&username))
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch user: {}", e))?
        .ok_or_else(|| "User not found".to_string())?;

    let permissions: Vec<String> = serde_json::from_str(&user_model.permissions)
        .unwrap_or_else(|_| vec![]);

    Ok(permissions.contains(&permission))
}

// Legacy function kept for backwards compatibility
#[tauri::command]
pub fn get_user_roles() -> Vec<String> {
    vec![]
}