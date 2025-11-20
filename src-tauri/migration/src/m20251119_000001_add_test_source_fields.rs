use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add source_type column (voltech, manual, or other)
        manager
            .alter_table(
                Table::alter()
                    .table(Test::Table)
                    .add_column(
                        ColumnDef::new(Test::SourceType)
                            .string()
                            .not_null()
                            .default("other")
                    )
                    .to_owned(),
            )
            .await?;

        // Add associated_test column (nullable - header name of voltech test or manual test name)
        manager
            .alter_table(
                Table::alter()
                    .table(Test::Table)
                    .add_column(
                        ColumnDef::new(Test::AssociatedTest)
                            .string()
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        // Add manual_override column (nullable boolean - for "other" type tests)
        manager
            .alter_table(
                Table::alter()
                    .table(Test::Table)
                    .add_column(
                        ColumnDef::new(Test::ManualOverride)
                            .boolean()
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on source_type for filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_test_source_type")
                    .table(Test::Table)
                    .col(Test::SourceType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop index
        manager
            .drop_index(Index::drop().name("idx_test_source_type").to_owned())
            .await?;

        // Drop columns
        manager
            .alter_table(
                Table::alter()
                    .table(Test::Table)
                    .drop_column(Test::ManualOverride)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Test::Table)
                    .drop_column(Test::AssociatedTest)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Test::Table)
                    .drop_column(Test::SourceType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Test {
    Table,
    SourceType,
    AssociatedTest,
    ManualOverride,
}
