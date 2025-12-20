mod auth;
mod event;
mod fg;
mod joins;
mod manual;
mod report;
mod reports;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod sheets;
mod test;
mod test_types;
mod user;
mod voltech;

use tauri::Manager;

use migration::{DbErr as CoreDbErr, Migrator as CoreMigrator, MigratorTrait as CoreMigratorTrait};
// use migration_event::{DbErr as EventDbErr, Migrator as EventMigrator, MigratorTrait as EventMigratorTrait};
use migration_manual::{
    DbErr as ManualDbErr, Migrator as ManualMigrator, MigratorTrait as ManualMigratorTrait,
};
use migration_voltech::{
    DbErr as VoltechDbErr, Migrator as VoltechMigrator, MigratorTrait as VoltechMigratorTrait,
};
use sea_orm::{ConnectOptions, Database, DbConn};
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
    pub manual_db: Arc<DbConn>,
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

pub async fn establish_manual_connection() -> Result<DbConn, ManualDbErr> {
    // Try MANUAL_DATABASE_URL first (for tests), otherwise use default path
    let database_url = std::env::var("MANUAL_DATABASE_URL").unwrap_or_else(|_| {
        // Use a default path in the app data directory
        let app_data = dirs::data_dir()
            .expect("Failed to get app data directory")
            .join("vishay_testing");
        std::fs::create_dir_all(&app_data).expect("Failed to create app data directory");
        let db_path = app_data.join("manual.db");
        format!(
            "sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/manual.sqlite?mode=rwc"
        )
    });

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(20).min_connections(5);

    let db = Database::connect(opt)
        .await
        .expect("Failed to setup the manual database");
    ManualMigrator::up(&db, None)
        .await
        .expect("Failed to run manual migrations");

    Ok(db)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
            user::get_user_preferences,
            user::update_user_preferences,
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
            reports::serialized::get_serialized,
            // Event CRUD
            event::create_event,
            event::get_event,
            event::get_all_events,
            event::update_event,
            event::delete_event,
            event::complete_event,
            event::get_events_by_user,
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
            voltech::update_server_path_setting,
            // Manual Test Import & Operations
            manual::commands::import_manual_file,
            manual::commands::import_manual_fg_folder,
            manual::commands::get_manual_test_names,
            manual::commands::get_manual_tests,
            manual::commands::get_manual_summary,
            manual::commands::get_manual_base_path,
            manual::commands::set_manual_base_path,
            // Test Type Mapping
            test_types::get_test_types,
            test_types::find_tests_for_type,
            // Report Validation & Generation
            reports::validator::validate_report,
            reports::collector::collect_report,
            reports::generate_excel_report,
            reports::save_excel_report,
            reports::debug::debug_voltech_query,
            reports::excel::create_ba_report
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .format(|out, message, record| {
                            let target = record.target();
                            
                            // Custom formatting for SQL queries
                            if target.starts_with("sqlx::query") {
                                let msg_str = message.to_string();
                                
                                // Debug: Print first 200 chars to see actual format
                                // eprintln!("DEBUG LOG: {}", &msg_str[..msg_str.len().min(200)]);
                                
                                // The log format appears to be: summary="..." db.statement="..." rows_affected=X rows_returned=Y elapsed=Xms elapsed_secs=X.X
                                // We need to parse db.statement field for the SQL
                                
                                let table = if let Some(stmt_start) = msg_str.find("db.statement=\"") {
                                    let sql_start = stmt_start + 14;
                                    let sql_section = &msg_str[sql_start..];
                                    
                                    // Find the closing quote, handling escaped quotes
                                    let mut quote_end = 0;
                                    let chars: Vec<char> = sql_section.chars().collect();
                                    let mut i = 0;
                                    while i < chars.len() {
                                        if chars[i] == '\\' && i + 1 < chars.len() {
                                            i += 2; // Skip escaped character
                                        } else if chars[i] == '"' {
                                            quote_end = i;
                                            break;
                                        } else {
                                            i += 1;
                                        }
                                    }
                                    
                                    if quote_end > 0 {
                                        let sql = &sql_section[..quote_end];
                                        
                                        // Parse table name from SQL
                                        if let Some(from_idx) = sql.find(" FROM ") {
                                            let after_from = &sql[from_idx + 6..];
                                            // Remove quotes and whitespace
                                            let table_name = after_from
                                                .trim_start()
                                                .trim_start_matches('"')
                                                .split(|c: char| c == '"' || c == ' ' || c == '\n')
                                                .next()
                                                .unwrap_or("unknown");
                                            table_name
                                        } else if let Some(into_idx) = sql.find(" INTO ") {
                                            let after_into = &sql[into_idx + 6..];
                                            let table_name = after_into
                                                .trim_start()
                                                .trim_start_matches('"')
                                                .split(|c: char| c == '"' || c == ' ' || c == '\n')
                                                .next()
                                                .unwrap_or("unknown");
                                            table_name
                                        } else if let Some(update_idx) = sql.find("UPDATE ") {
                                            let after_update = &sql[update_idx + 7..];
                                            let table_name = after_update
                                                .trim_start()
                                                .trim_start_matches('"')
                                                .split(|c: char| c == '"' || c == ' ' || c == '\n')
                                                .next()
                                                .unwrap_or("unknown");
                                            table_name
                                        } else {
                                            "unknown"
                                        }
                                    } else {
                                        "unknown"
                                    }
                                } else {
                                    "unknown"
                                };
                                
                                // Extract operation type
                                let operation = if msg_str.contains("SELECT") {
                                    "SELECT"
                                } else if msg_str.contains("INSERT") {
                                    "INSERT"
                                } else if msg_str.contains("UPDATE") {
                                    "UPDATE"
                                } else if msg_str.contains("DELETE") {
                                    "DELETE"
                                } else {
                                    "QUERY"
                                };
                                
                                // Extract timing from elapsed_secs field
                                let elapsed = if let Some(secs_idx) = msg_str.find("elapsed_secs=") {
                                    let start = secs_idx + 13;
                                    // Find the end of the number (space, newline, or end of string)
                                    let remaining = &msg_str[start..];
                                    let end_pos = remaining
                                        .find(|c: char| !c.is_numeric() && c != '.')
                                        .unwrap_or(remaining.len());
                                    
                                    if let Ok(secs) = remaining[..end_pos].parse::<f64>() {
                                        let ms = secs * 1000.0;
                                        if ms < 1.0 {
                                            format!("{:.2}ms", ms)
                                        } else if ms < 10.0 {
                                            format!("{:.1}ms", ms)
                                        } else {
                                            format!("{:.0}ms", ms)
                                        }
                                    } else {
                                        "?".to_string()
                                    }
                                } else {
                                    "?".to_string()
                                };
                                
                                // Extract row information
                                let rows_info = if let Some(returned_idx) = msg_str.find("rows_returned=") {
                                    let start = returned_idx + 14;
                                    let remaining = &msg_str[start..];
                                    let end_pos = remaining
                                        .find(|c: char| !c.is_numeric())
                                        .unwrap_or(remaining.len());
                                    format!("→ {} rows", &remaining[..end_pos])
                                } else if let Some(affected_idx) = msg_str.find("rows_affected=") {
                                    let start = affected_idx + 14;
                                    let remaining = &msg_str[start..];
                                    let end_pos = remaining
                                        .find(|c: char| !c.is_numeric())
                                        .unwrap_or(remaining.len());
                                    let affected = &remaining[..end_pos];
                                    if affected != "0" {
                                        format!("✓ {} affected", affected)
                                    } else {
                                        String::new()
                                    }
                                } else {
                                    String::new()
                                };
                                
                                out.finish(format_args!(
                                    "[{}] 🗄️  {} {} | {} {}",
                                    record.level(),
                                    operation,
                                    table,
                                    elapsed,
                                    rows_info
                                ))
                            } else {
                                // Standard formatting for non-SQL logs
                                out.finish(format_args!(
                                    "[{}] [{}] {}",
                                    record.level(),
                                    target,
                                    message
                                ))
                            }
                        })
                        .build(),
                )?;
            }

            // Initialize database connections
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let core_db = runtime.block_on(establish_core_connection()).unwrap();
            let voltech_db = runtime.block_on(establish_voltech_connection()).unwrap();
            let manual_db = runtime.block_on(establish_manual_connection()).unwrap();

            // Generate unique instance ID for this app instance
            let instance_id = uuid::Uuid::new_v4().to_string();

            // Initialize watcher state
            let watcher_state =
                Arc::new(Mutex::new(voltech::WatcherState::new(instance_id.clone())));

            // Store connections in app state
            app.manage(AppState {
                core_db: Arc::new(core_db),
                voltech_db: Arc::new(voltech_db),
                manual_db: Arc::new(manual_db),
                voltech_watcher_state: watcher_state,
                instance_id,
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
