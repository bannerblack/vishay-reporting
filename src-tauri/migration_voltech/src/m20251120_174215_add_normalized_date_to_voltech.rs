use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add normalized_date column to test_results table
        manager
            .alter_table(
                Table::alter()
                    .table(TestResults::Table)
                    .add_column(date_null(TestResults::NormalizedDate))
                    .to_owned(),
            )
            .await?;

        // Backfill normalized_date from existing date strings
        // Date format in voltech is "DD-MM-YY" (e.g., "19-11-25")
        // Convert to ISO format: "20YY-MM-DD"
        let db = manager.get_connection();
        let sql = r#"
            UPDATE test_results 
            SET normalized_date = 
                CASE 
                    WHEN date LIKE '__-__-__' THEN
                        date('20' || substr(date, 7, 2) || '-' || substr(date, 4, 2) || '-' || substr(date, 1, 2))
                    ELSE NULL
                END
            WHERE normalized_date IS NULL
        "#;

        db.execute_unprepared(sql).await?;

        // Create index on normalized_date
        manager
            .create_index(
                Index::create()
                    .name("idx_test_results_normalized_date")
                    .table(TestResults::Table)
                    .col(TestResults::NormalizedDate)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_test_results_normalized_date")
                    .table(TestResults::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TestResults::Table)
                    .drop_column(TestResults::NormalizedDate)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TestResults {
    Table,
    NormalizedDate,
}
