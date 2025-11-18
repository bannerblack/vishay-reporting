// Database operations for voltech integration with SeaORM
use entity_voltech::{processed_files, settings, parse_errors, watcher_lock};
use sea_orm::{entity::*, query::*, DbConn, DbErr, Set, ActiveValue::NotSet};
use sea_orm::sea_query::{OnConflict, Expr};
use chrono::Utc;
use uuid::Uuid;

// ==================== File Processing Tracking ====================

/// Check if a file needs processing based on size/modified time
pub async fn needs_processing(
    db: &DbConn,
    file_path: &str,
    file_size: i32,
    file_modified: i32,
) -> Result<bool, DbErr> {
    let existing = processed_files::Entity::find()
        .filter(processed_files::Column::FilePath.eq(file_path))
        .one(db)
        .await?;

    match existing {
        Some(record) => {
            // Check if file has changed (compare timestamps as i32)
            Ok(record.file_size != file_size || record.file_modified.timestamp() as i32 != file_modified)
        }
        None => Ok(true), // New file, needs processing
    }
}

/// Mark a file as processed
pub async fn mark_file_processed(
    db: &DbConn,
    file_path: &str,
    file_size: i32,
    file_modified: i32,
    record_count: i32,
) -> Result<(), DbErr> {
    let now = Utc::now();
    let file_modified_dt = chrono::DateTime::from_timestamp(file_modified as i64, 0)
        .unwrap_or(now)
        .with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());
    
    let model = processed_files::ActiveModel {
        id: NotSet,
        file_path: Set(file_path.to_string()),
        file_size: Set(file_size),
        file_modified: Set(file_modified_dt),
        record_count: Set(record_count),
        processed_at: Set(now.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())),
    };

    // Insert or replace
    processed_files::Entity::insert(model)
        .on_conflict(
            OnConflict::column(processed_files::Column::FilePath)
                .update_columns([
                    processed_files::Column::FileSize,
                    processed_files::Column::FileModified,
                    processed_files::Column::RecordCount,
                    processed_files::Column::ProcessedAt,
                ])
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(())
}

/// Get all processed files
pub async fn get_all_processed_files(db: &DbConn) -> Result<Vec<processed_files::Model>, DbErr> {
    processed_files::Entity::find()
        .order_by_desc(processed_files::Column::ProcessedAt)
        .all(db)
        .await
}

// ==================== Settings Management ====================

/// Get a setting value by key
pub async fn get_setting(db: &DbConn, key: &str) -> Result<Option<String>, DbErr> {
    let result = settings::Entity::find()
        .filter(settings::Column::Key.eq(key))
        .one(db)
        .await?;

    Ok(result.map(|s| s.value))
}

/// Set a setting value
pub async fn set_setting(db: &DbConn, key: &str, value: &str) -> Result<(), DbErr> {
    let model = settings::ActiveModel {
        key: Set(key.to_string()),
        value: Set(value.to_string()),
        updated_at: Set(Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())),
    };

    settings::Entity::insert(model)
        .on_conflict(
            OnConflict::column(settings::Column::Key)
                .update_column(settings::Column::Value)
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(())
}

/// Get all settings
pub async fn get_all_settings(db: &DbConn) -> Result<Vec<settings::Model>, DbErr> {
    settings::Entity::find()
        .order_by_asc(settings::Column::Key)
        .all(db)
        .await
}

/// Delete a setting
pub async fn delete_setting(db: &DbConn, key: &str) -> Result<(), DbErr> {
    settings::Entity::delete_many()
        .filter(settings::Column::Key.eq(key))
        .exec(db)
        .await?;

    Ok(())
}

// ==================== Error Logging ====================

/// Log a parse error to the database
pub async fn log_parse_error(
    db: &DbConn,
    file_path: &str,
    error_message: &str,
    line_number: Option<i32>,
) -> Result<i32, DbErr> {
    let model = parse_errors::ActiveModel {
        id: NotSet,
        file_path: Set(file_path.to_string()),
        error_message: Set(error_message.to_string()),
        line_number: Set(line_number),
        timestamp: Set(Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())),
        acknowledged: Set(false),
    };

    let result = model.insert(db).await?;
    Ok(result.id)
}

/// Get all parse errors with optional filtering
pub async fn get_errors(
    db: &DbConn,
    acknowledged: Option<bool>,
    file_path: Option<String>,
) -> Result<Vec<parse_errors::Model>, DbErr> {
    let mut query = parse_errors::Entity::find();

    if let Some(ack) = acknowledged {
        query = query.filter(parse_errors::Column::Acknowledged.eq(ack));
    }

    if let Some(path) = file_path {
        query = query.filter(parse_errors::Column::FilePath.eq(path));
    }

    query
        .order_by_desc(parse_errors::Column::Timestamp)
        .all(db)
        .await
}

/// Acknowledge specific errors
pub async fn acknowledge_errors(db: &DbConn, error_ids: Vec<i32>) -> Result<u64, DbErr> {
    let result = parse_errors::Entity::update_many()
        .col_expr(parse_errors::Column::Acknowledged, Expr::value(true))
        .filter(parse_errors::Column::Id.is_in(error_ids))
        .exec(db)
        .await?;

    Ok(result.rows_affected)
}

/// Acknowledge all errors for a file
pub async fn acknowledge_file_errors(db: &DbConn, file_path: &str) -> Result<u64, DbErr> {
    let result = parse_errors::Entity::update_many()
        .col_expr(parse_errors::Column::Acknowledged, Expr::value(true))
        .filter(parse_errors::Column::FilePath.eq(file_path))
        .exec(db)
        .await?;

    Ok(result.rows_affected)
}

/// Delete acknowledged errors older than specified days
pub async fn cleanup_old_errors(db: &DbConn, days: i64) -> Result<u64, DbErr> {
    let cutoff = Utc::now() - chrono::Duration::days(days);

    let result = parse_errors::Entity::delete_many()
        .filter(parse_errors::Column::Acknowledged.eq(true))
        .filter(parse_errors::Column::Timestamp.lt(cutoff))
        .exec(db)
        .await?;

    Ok(result.rows_affected)
}

// ==================== Lock Management (Master/Follower) ====================

/// Acquire the watcher lock (master role)
pub async fn acquire_lock(
    db: &DbConn,
    holder_name: &str,
) -> Result<String, DbErr> {
    let instance_id = Uuid::new_v4().to_string();
    let now = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

    let model = watcher_lock::ActiveModel {
        id: Set(1), // Always ID 1 (single-row table)
        holder_id: Set(instance_id.clone()),
        holder_name: Set(holder_name.to_string()),
        acquired_at: Set(now),
        last_heartbeat: Set(now),
        is_active: Set(true),
    };

    // Try to insert or update if lock is stale
    watcher_lock::Entity::insert(model)
        .on_conflict(
            OnConflict::column(watcher_lock::Column::Id)
                .update_columns([
                    watcher_lock::Column::HolderId,
                    watcher_lock::Column::HolderName,
                    watcher_lock::Column::AcquiredAt,
                    watcher_lock::Column::LastHeartbeat,
                    watcher_lock::Column::IsActive,
                ])
                .to_owned()
        )
        .exec(db)
        .await?;

    Ok(instance_id)
}

/// Update heartbeat for active lock holder
pub async fn update_heartbeat(
    db: &DbConn,
    instance_id: &str,
) -> Result<bool, DbErr> {
    let now = Utc::now();

    let result = watcher_lock::Entity::update_many()
        .col_expr(watcher_lock::Column::LastHeartbeat, Expr::value(now))
        .filter(watcher_lock::Column::HolderId.eq(instance_id))
        .filter(watcher_lock::Column::IsActive.eq(true))
        .exec(db)
        .await?;

    Ok(result.rows_affected > 0)
}

/// Release the lock
pub async fn release_lock(
    db: &DbConn,
    instance_id: &str,
) -> Result<(), DbErr> {
    watcher_lock::Entity::update_many()
        .col_expr(watcher_lock::Column::IsActive, Expr::value(false))
        .filter(watcher_lock::Column::HolderId.eq(instance_id))
        .exec(db)
        .await?;

    Ok(())
}

/// Check if lock is stale (no heartbeat for > 2 minutes)
pub async fn check_stale_lock(db: &DbConn) -> Result<bool, DbErr> {
    let lock = watcher_lock::Entity::find()
        .filter(watcher_lock::Column::Id.eq(1))
        .filter(watcher_lock::Column::IsActive.eq(true))
        .one(db)
        .await?;

    match lock {
        Some(record) => {
            let elapsed = Utc::now().signed_duration_since(record.last_heartbeat);
            Ok(elapsed.num_seconds() > 120) // 2 minutes
        }
        None => Ok(false), // No active lock
    }
}

/// Get current lock holder info
pub async fn get_lock_info(db: &DbConn) -> Result<Option<watcher_lock::Model>, DbErr> {
    watcher_lock::Entity::find()
        .filter(watcher_lock::Column::Id.eq(1))
        .one(db)
        .await
}

/// Force release any lock (admin operation)
pub async fn force_release_lock(db: &DbConn) -> Result<(), DbErr> {
    watcher_lock::Entity::update_many()
        .col_expr(watcher_lock::Column::IsActive, Expr::value(false))
        .exec(db)
        .await?;

    Ok(())
}
