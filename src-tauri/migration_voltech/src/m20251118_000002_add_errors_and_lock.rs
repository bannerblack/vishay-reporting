use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create parse_errors table
        manager
            .create_table(
                Table::create()
                    .table(ParseErrors::Table)
                    .if_not_exists()
                    .col(pk_auto(ParseErrors::Id))
                    .col(string(ParseErrors::FilePath).not_null())
                    .col(text(ParseErrors::ErrorMessage).not_null())
                    .col(integer_null(ParseErrors::LineNumber))
                    .col(
                        timestamp_with_time_zone(ParseErrors::Timestamp)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(boolean(ParseErrors::Acknowledged).default(false).not_null())
                    .to_owned(),
            )
            .await?;

        // Create indexes for parse_errors
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_parse_errors_acknowledged")
                    .table(ParseErrors::Table)
                    .col(ParseErrors::Acknowledged)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_parse_errors_timestamp")
                    .table(ParseErrors::Table)
                    .col(ParseErrors::Timestamp)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_parse_errors_file_path")
                    .table(ParseErrors::Table)
                    .col(ParseErrors::FilePath)
                    .to_owned(),
            )
            .await?;

        // Create watcher_lock table (single row)
        manager
            .create_table(
                Table::create()
                    .table(WatcherLock::Table)
                    .if_not_exists()
                    .col(integer(WatcherLock::Id).primary_key().default(1).not_null())
                    .col(string(WatcherLock::HolderId).not_null())
                    .col(string(WatcherLock::HolderName).not_null())
                    .col(
                        timestamp_with_time_zone(WatcherLock::AcquiredAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp_with_time_zone(WatcherLock::LastHeartbeat)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(boolean(WatcherLock::IsActive).default(true).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(WatcherLock::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ParseErrors::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ParseErrors {
    Table,
    Id,
    FilePath,
    ErrorMessage,
    LineNumber,
    Timestamp,
    Acknowledged,
}

#[derive(DeriveIden)]
enum WatcherLock {
    Table,
    Id,
    HolderId,
    HolderName,
    AcquiredAt,
    LastHeartbeat,
    IsActive,
}
