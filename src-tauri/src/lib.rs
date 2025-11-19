#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod sheets;
mod auth;
mod user;
mod fg;
mod report;
mod joins;
mod test;
mod voltech;

use tauri::Manager;

use migration::{DbErr as CoreDbErr, Migrator as CoreMigrator, MigratorTrait as CoreMigratorTrait};
// use migration_event::{DbErr as EventDbErr, Migrator as EventMigrator, MigratorTrait as EventMigratorTrait};
use migration_voltech::{DbErr as VoltechDbErr, Migrator as VoltechMigrator, MigratorTrait as VoltechMigratorTrait};
use sea_orm::{Database, DbConn, ConnectOptions};
use std::sync::Arc;
use tokio::sync::Mutex;

// State management for database connections
// pub struct AppState {
//     pub core_db: Arc<DbConn>,
//     pub event_db: Arc<DbConn>,
//     pub voltech_db: Arc<DbConn>,
// }

pub struct AppState {
    pub core_db: Arc<DbConn>,
    pub voltech_db: Arc<DbConn>,
    pub voltech_watcher_state: Arc<Mutex<voltech::WatcherState>>,
    pub instance_id: String,
}

pub async fn establish_core_connection() -> Result<DbConn, CoreDbErr> {
    // Try DATABASE_URL first (for tests), otherwise use default path
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            // Use a default path in the app data directory
            let app_data = dirs::data_dir()
                .expect("Failed to get app data directory")
                .join("vishay_testing");
            std::fs::create_dir_all(&app_data)
                .expect("Failed to create app data directory");
            let db_path = app_data.join("vishay.db");
            format!("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/testing.sqlite?mode=rwc")
        });
    
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");
    CoreMigrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    
    Ok(db)
}

// pub async fn establish_event_connection() -> Result<DbConn, EventDbErr> {
//     // Try EVENT_DATABASE_URL first (for tests), otherwise use default path
//     let database_url = std::env::var("EVENT_DATABASE_URL")
//         .unwrap_or_else(|_| {
//             // Use a default path in the app data directory
//             let app_data = dirs::data_dir()
//                 .expect("Failed to get app data directory")
//                 .join("vishay_testing");
//             std::fs::create_dir_all(&app_data)
//                 .expect("Failed to create app data directory");
//             let db_path = app_data.join("events.db");
//             format!("sqlite://{}?mode=rwc", db_path.display())
//         });
    
//     let db = Database::connect(&database_url)
//         .await
//         .expect("Failed to setup the event database");
//     EventMigrator::up(&db, None)
//         .await
//         .expect("Failed to run event migrations");
    
//     Ok(db)
// }

pub async fn establish_voltech_connection() -> Result<DbConn, VoltechDbErr> {
    // Try VOLTECH_DATABASE_URL first (for tests), otherwise use default path
    let database_url = std::env::var("VOLTECH_DATABASE_URL")
        .unwrap_or_else(|_| {
            // Use a default path in the app data directory
            let app_data = dirs::data_dir()
                .expect("Failed to get app data directory")
                .join("vishay_testing");
            std::fs::create_dir_all(&app_data)
                .expect("Failed to create app data directory");
            let db_path = app_data.join("voltech.db");
            format!("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/voltech.sqlite?mode=rwc")
        });
    
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(20).min_connections(5);
    
    let db = Database::connect(opt)
        .await
        .expect("Failed to setup the voltech database");
    VoltechMigrator::up(&db, None)
        .await
        .expect("Failed to run voltech migrations");
    
    Ok(db)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
          sheets::generate_spreadsheet, 

          // Authentication
            auth::get_system_user,
            auth::get_user_roles,
            auth::authenticate_user,
            auth::needs_initial_setup,
            auth::validate_admin_password,
            auth::create_initial_admin,
            auth::admin_create_user,
            auth::user_has_permission,
            
            // User CRUD
            user::create_user,
            user::get_user,
            user::get_user_by_username,
            user::get_all_users,
            user::update_user,
            user::delete_user,
            
            // FG CRUD
            fg::create_fg,
            fg::get_fg,
            fg::get_fg_by_number,
            fg::get_all_fgs,
            fg::update_fg,
            fg::delete_fg,
            
            // Report CRUD
            report::create_report,
            report::get_report,
            report::get_all_reports,
            report::update_report,
            report::delete_report,
            
            // Test CRUD
            test::create_test,
            test::get_test,
            test::get_all_tests,
            test::update_test,
            test::delete_test,
            test::assign_test_to_report,
            test::unassign_test_from_report,
            test::update_test_order,
            
            // Join Operations
            joins::get_fg_with_reports,
            joins::get_fg_with_tests,
            joins::get_fg_complete,
            joins::get_report_with_fg,
            joins::get_report_with_tests,
            joins::get_report_complete,
            joins::get_test_with_fg,
            joins::get_test_with_report,
            joins::get_test_complete,
            joins::get_all_reports_with_fg,
            joins::get_all_tests_by_fg,
            joins::get_all_tests_by_report,
            joins::get_available_tests_for_report,
            
            // Voltech File Processing
            voltech::start_voltech_watcher,
            voltech::stop_voltech_watcher,
            voltech::pause_voltech_watcher,
            voltech::resume_voltech_watcher,
            voltech::get_voltech_watcher_status,
            voltech::import_voltech_files,
            voltech::force_acquire_voltech_master,
            
            // Voltech Settings
            voltech::get_voltech_settings,
            voltech::set_voltech_setting,
            voltech::get_all_voltech_settings,
            voltech::delete_voltech_setting,
            
            // Voltech Errors
            voltech::get_voltech_errors,
            voltech::acknowledge_voltech_errors,
            voltech::acknowledge_file_errors,
            voltech::cleanup_old_voltech_errors,
            
            // Voltech Lock Management
            voltech::get_voltech_lock_status,
            voltech::force_release_voltech_lock,
            
            // Voltech Queries - Batch
            voltech::get_recent_batches_for_part,
            voltech::get_batch_details,
            voltech::search_batches,
            voltech::get_batch_tests,
            voltech::get_batches_for_part,
            
            // Voltech Queries - Part
            voltech::get_all_parts,
            voltech::get_part_summary,
            voltech::search_parts,
            
            // Voltech Queries - Test
            voltech::search_tests,
            voltech::get_failed_tests,
            voltech::get_test_by_serial,
            
            // Voltech Queries - Stats
            voltech::get_daily_stats,
            voltech::get_operator_stats,
            voltech::get_overall_stats,
            voltech::get_part_stats,
            
            // Voltech Full Import
            voltech::reset_voltech_database,
            voltech::full_import_voltech_files,
            voltech::update_server_path_setting
            ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            
            // Initialize database connections
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let core_db = runtime.block_on(establish_core_connection()).unwrap();
            let voltech_db = runtime.block_on(establish_voltech_connection()).unwrap();
            
            // Generate unique instance ID for this app instance
            let instance_id = uuid::Uuid::new_v4().to_string();
            
            // Initialize watcher state
            let watcher_state = Arc::new(Mutex::new(voltech::WatcherState::new(instance_id.clone())));
            
            // Store connections in app state
            app.manage(AppState {
                core_db: Arc::new(core_db),
                voltech_db: Arc::new(voltech_db),
                voltech_watcher_state: watcher_state,
                instance_id,
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
