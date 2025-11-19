use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add relative_path column to processed_files table
        manager
            .alter_table(
                Table::alter()
                    .table(ProcessedFiles::Table)
                    .add_column(string_null(ProcessedFiles::RelativePath))
                    .to_owned(),
            )
            .await?;

        // Create index on relative_path for faster lookups
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_relative_path")
                    .table(ProcessedFiles::Table)
                    .col(ProcessedFiles::RelativePath)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop index first
        manager
            .drop_index(
                Index::drop()
                    .name("idx_relative_path")
                    .table(ProcessedFiles::Table)
                    .to_owned(),
            )
            .await?;

        // Drop relative_path column
        manager
            .alter_table(
                Table::alter()
                    .table(ProcessedFiles::Table)
                    .drop_column(ProcessedFiles::RelativePath)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProcessedFiles {
    Table,
    RelativePath,
}
