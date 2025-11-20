use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Fg::Table)
                    .add_column(boolean(Fg::Serialized).not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Fg::Table)
                    .drop_column(Fg::Serialized)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Fg {
    Table,
    Serialized,
}
