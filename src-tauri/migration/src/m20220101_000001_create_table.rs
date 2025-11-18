use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(
                        ColumnDef::new(User::Preferences)
                            .string()
                            .not_null()
                            .default("{'theme': 'light'}"),
                    )
                    .col(ColumnDef::new(User::AddedBy).integer())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FG::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FG::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FG::FG)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(FG::Rev)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(FG::Customer)
                            .string()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Report::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Report::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Report::FGId).integer().not_null())
                    .col(
                        ColumnDef::new(Report::Attributes)
                            .string()
                            .not_null()
                            .default("['default']"),
                    )
                    .col(ColumnDef::new(Report::AddedBy).integer())
                    .col(
                        ColumnDef::new(Report::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Report::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fg_fk")
                            .from(Report::Table, Report::FGId)
                            .to(FG::Table, FG::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Test::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Test::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Test::ReportId)
                            .integer()
                            .default(Expr::val(1)),
                    )
                    .col(ColumnDef::new(Test::FGId).integer().not_null())
                    .col(ColumnDef::new(Test::TestType).string().not_null())
                    .col(ColumnDef::new(Test::Frequency).double())
                    .col(ColumnDef::new(Test::Voltage).double())
                    .col(ColumnDef::new(Test::Minimum).double())
                    .col(ColumnDef::new(Test::Maximum).double())
                    .col(ColumnDef::new(Test::UoM).string().not_null())
                    .col(ColumnDef::new(Test::PrimaryPins).string())
                    .col(ColumnDef::new(Test::SecondaryPins).string())
                    .col(ColumnDef::new(Test::ShortedPins).string())
                    .col(ColumnDef::new(Test::Description).string())
                    .col(ColumnDef::new(Test::AddedBy).integer())
                    .col(
                        ColumnDef::new(Test::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Test::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fg_fk")
                            .from(Test::Table, Test::FGId)
                            .to(FG::Table, FG::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("report_fk")
                            .from(Test::Table, Test::ReportId)
                            .to(Report::Table, Report::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FG::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Report::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Test::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Username,
    Preferences,
    AddedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum FG {
    Table,
    Id,
    FG,
    Rev,
    Customer
}

#[derive(DeriveIden)]
enum Report {
    Table,
    Id,
    FGId,
    Attributes,
    AddedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Test {
    Table,
    Id,
    FGId,
    ReportId,
    TestType,
    Frequency,
    Voltage,
    Minimum,
    Maximum,
    UoM,
    PrimaryPins,
    SecondaryPins,
    ShortedPins,
    Description,
    AddedBy,
    CreatedAt,
    UpdatedAt,
}
