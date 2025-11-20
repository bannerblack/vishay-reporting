use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add permissions column to User table
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::Permissions)
                            .string()
                            .not_null()
                            .default("[]"),
                    )
                    .to_owned(),
            )
            .await?;

        // Create Event table
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::OriginatorId).integer())
                    .col(ColumnDef::new(Event::TargetId).integer())
                    .col(ColumnDef::new(Event::ReportId).integer().not_null())
                    .col(ColumnDef::new(Event::Comment).string().not_null())
                    .col(
                        ColumnDef::new(Event::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Event::Complete)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Event::CompletedDate).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_originator")
                            .from(Event::Table, Event::OriginatorId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_target")
                            .from(Event::Table, Event::TargetId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_report")
                            .from(Event::Table, Event::ReportId)
                            .to(Report::Table, Report::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for performance
        manager
            .create_index(
                Index::create()
                    .name("idx_user_username")
                    .table(User::Table)
                    .col(User::Username)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_report_fg_id")
                    .table(Report::Table)
                    .col(Report::FgId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_test_report_id")
                    .table(Test::Table)
                    .col(Test::ReportId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_event_target_id")
                    .table(Event::Table)
                    .col(Event::TargetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_event_complete")
                    .table(Event::Table)
                    .col(Event::Complete)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(Index::drop().name("idx_event_complete").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_event_target_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_test_report_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_report_fg_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_user_username").to_owned())
            .await?;

        // Drop Event table
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await?;

        // Drop permissions column from User table
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Permissions)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Permissions,
}

#[derive(DeriveIden)]
enum Report {
    Table,
    Id,
    FgId,
}

#[derive(DeriveIden)]
enum Test {
    Table,
    ReportId,
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
    OriginatorId,
    TargetId,
    ReportId,
    Comment,
    CreatedAt,
    Complete,
    CompletedDate,
}
