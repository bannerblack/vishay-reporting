pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241115_000002_add_test_order;
mod m20251115_150148_events_and_fixes;
mod m20251119_000001_add_test_source_fields;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241115_000002_add_test_order::Migration),
            Box::new(m20251115_150148_events_and_fixes::Migration),
            Box::new(m20251119_000001_add_test_source_fields::Migration),
        ]
    }
}
