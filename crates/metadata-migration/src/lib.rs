pub use sea_orm_migration::prelude::*;

mod m20251128_171016_create_table_metadata;
mod m20251128_171023_create_table_metadata_relation;
mod metadata_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251128_171016_create_table_metadata::Migration),
            Box::new(m20251128_171023_create_table_metadata_relation::Migration),
        ]
    }
}
