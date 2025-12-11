use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(biz_metadata_migration::Migrator).await;
}
