pub use sea_orm_migration::prelude::*;

mod m20251128_171016_create_table_biz_metadata;
mod m20251128_171200_create_table_biz_metadata_alias;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251128_171016_create_table_biz_metadata::Migration),
            Box::new(m20251128_171200_create_table_biz_metadata_alias::Migration),
        ]
    }
}
