use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create manual_test_results table
        manager
            .create_table(
                Table::create()
                    .table(ManualTestResults::Table)
                    .if_not_exists()
                    .col(pk_auto(ManualTestResults::Id))
                    .col(integer(ManualTestResults::Result))
                    .col(string(ManualTestResults::Test))
                    .col(string(ManualTestResults::Fg))
                    .col(string(ManualTestResults::Rev))
                    .col(string(ManualTestResults::Batch))
                    .col(string(ManualTestResults::Operator))
                    .col(string(ManualTestResults::Date))
                    .col(string(ManualTestResults::Time))
                    .col(string(ManualTestResults::Sn))
                    .col(string(ManualTestResults::Passfail))
                    .col(double(ManualTestResults::Minimum))
                    .col(double(ManualTestResults::Reading))
                    .col(double(ManualTestResults::Maximum))
                    .col(string(ManualTestResults::Uom))
                    .col(string(ManualTestResults::FilePath))
                    .col(timestamp_with_time_zone(ManualTestResults::CreatedAt))
                    .col(date(ManualTestResults::NormalizedDate))
                    .to_owned(),
            )
            .await?;

        // Create indexes for manual_test_results
        manager
            .create_index(
                Index::create()
                    .name("idx_manual_fg")
                    .table(ManualTestResults::Table)
                    .col(ManualTestResults::Fg)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_manual_batch")
                    .table(ManualTestResults::Table)
                    .col(ManualTestResults::Batch)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_manual_test")
                    .table(ManualTestResults::Table)
                    .col(ManualTestResults::Test)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_manual_normalized_date")
                    .table(ManualTestResults::Table)
                    .col(ManualTestResults::NormalizedDate)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_manual_passfail")
                    .table(ManualTestResults::Table)
                    .col(ManualTestResults::Passfail)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_manual_file_path")
                    .table(ManualTestResults::Table)
                    .col(ManualTestResults::FilePath)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_manual_created_at")
                    .table(ManualTestResults::Table)
                    .col(ManualTestResults::CreatedAt)
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
                    .col(string(ProcessedFiles::FileName))
                    .col(string_uniq(ProcessedFiles::FilePath))
                    .col(timestamp_with_time_zone(ProcessedFiles::ProcessedAt))
                    .col(integer(ProcessedFiles::RecordCount))
                    .to_owned(),
            )
            .await?;

        // Create settings table
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(pk_auto(Settings::Id))
                    .col(string_uniq(Settings::Key))
                    .col(string(Settings::Value))
                    .col(timestamp_with_time_zone(Settings::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        // Insert default base_path setting
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Settings::Table)
                    .columns([Settings::Key, Settings::Value, Settings::UpdatedAt])
                    .values_panic([
                        "base_path".into(),
                        r"\\wsdv03\DV_Specific\Departments\Voltech\LabView\LabView Results\".into(),
                        chrono::Utc::now().into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ProcessedFiles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ManualTestResults::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ManualTestResults {
    Table,
    Id,
    Result,
    Test,
    Fg,
    Rev,
    Batch,
    Operator,
    Date,
    Time,
    Sn,
    Passfail,
    Minimum,
    Reading,
    Maximum,
    Uom,
    FilePath,
    CreatedAt,
    NormalizedDate,
}

#[derive(DeriveIden)]
enum ProcessedFiles {
    Table,
    Id,
    FileName,
    FilePath,
    ProcessedAt,
    RecordCount,
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Id,
    Key,
    Value,
    UpdatedAt,
}
