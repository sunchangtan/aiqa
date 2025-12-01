use sea_orm::{Database, DatabaseConnection};

pub use application::service::metadata::{
    CreateMetadataCommand, ExtraUpdate, MetadataQueryRequest, MetadataService,
    UpdateMetadataCommand,
};

use infrastructure::repository::metadata_repository_impl::MetadataRepositoryImpl;

mod application;
mod domain;
mod infrastructure;

/// 根据数据库连接字符串构建 `MetadataService`，内部会创建 SeaORM 连接。
pub async fn metadata_service_from_url(
    url: &str,
) -> Result<MetadataService<MetadataRepositoryImpl>, sea_orm::DbErr> {
    let db = Database::connect(url).await?;
    Ok(metadata_service_from_connection(db))
}

/// 使用已有的 `DatabaseConnection` 构建 `MetadataService`，便于复用连接池。
pub fn metadata_service_from_connection(
    db: DatabaseConnection,
) -> MetadataService<MetadataRepositoryImpl> {
    let repository = MetadataRepositoryImpl::new(db);
    MetadataService::new(repository)
}
