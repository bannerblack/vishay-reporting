use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create test_results table
        manager
            .create_table(
                Table::create()
                    .table(TestResults::Table)
                    .if_not_exists()
                    .col(pk_auto(TestResults::Id))
                    .col(string(TestResults::Part).not_null())
                    .col(string(TestResults::Operator).not_null())
                    .col(string(TestResults::Batch).not_null())
                    .col(string(TestResults::Date).not_null())
                    .col(string(TestResults::SerialNum).not_null())
                    .col(integer(TestResults::ResultNum).not_null())
                    .col(string(TestResults::PassFail).not_null())
                    .col(string_null(TestResults::Time))
                    .col(string_null(TestResults::Retries))
                    .col(string(TestResults::FilePath).not_null())
                    .col(text(TestResults::Measurements).not_null())
                    .col(
                        timestamp_with_time_zone(TestResults::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_unique_file_result")
                            .col(TestResults::FilePath)
                            .col(TestResults::ResultNum),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for test_results
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_part")
                    .table(TestResults::Table)
                    .col(TestResults::Part)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_date")
                    .table(TestResults::Table)
                    .col(TestResults::Date)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_batch")
                    .table(TestResults::Table)
                    .col(TestResults::Batch)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_pass_fail")
                    .table(TestResults::Table)
                    .col(TestResults::PassFail)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_file_path")
                    .table(TestResults::Table)
                    .col(TestResults::FilePath)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_created_at")
                    .table(TestResults::Table)
                    .col(TestResults::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create processed_files table
        manager
            .create_table(
                Table::create()
                    .table(ProcessedFiles::Table)
                    .if_not_exists()
                    .col(pk_auto(ProcessedFiles::Id))
                    .col(string(ProcessedFiles::FilePath).not_null().unique_key())
                    .col(big_integer(ProcessedFiles::FileSize).not_null())
                    .col(timestamp_with_time_zone(ProcessedFiles::FileModified).not_null())
                    .col(
                        timestamp_with_time_zone(ProcessedFiles::ProcessedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(integer(ProcessedFiles::RecordCount).not_null())
                    .to_owned(),
            )
            .await?;

        // Create settings table
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(string(Settings::Key).primary_key())
                    .col(text(Settings::Value).not_null())
                    .col(
                        timestamp_with_time_zone(Settings::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ProcessedFiles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TestResults::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TestResults {
    Table,
    Id,
    Part,
    Operator,
    Batch,
    Date,
    SerialNum,
    ResultNum,
    PassFail,
    Time,
    Retries,
    FilePath,
    Measurements,
    CreatedAt,
}

#[derive(DeriveIden)]
enum ProcessedFiles {
    Table,
    Id,
    FilePath,
    FileSize,
    FileModified,
    ProcessedAt,
    RecordCount,
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Key,
    Value,
    UpdatedAt,
}
