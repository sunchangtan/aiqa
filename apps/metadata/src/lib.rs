use sea_orm::{Database, DatabaseConnection};

pub use application::service::metadata::{
    CreateMetadataCommand, ExtraUpdate, MetadataQueryRequest, MetadataService,
    UpdateMetadataCommand,
};
pub use application::service::metadata_relation::{
    CreateMetadataRelationCommand, MetadataRelationQueryRequest, MetadataRelationService,
    RelinkMetadataRelationCommand,
};
pub use domain::metadata::repository::MetadataRepository;
pub use domain::metadata::value_object::{
    MetadataCapabilities, MetadataId, MetadataType, ValueType,
};
pub use domain::metadata::{Metadata, MetadataReconstructParams};
pub use domain::metadata_relation::MetadataRelation;
pub use domain::metadata_relation::repository::MetadataRelationRepository;
pub use domain::metadata_relation::value_object::MetadataRelationId;
pub use domain::metadata_relation::value_object::validate_relation_id;
pub use domain_core::prelude::Audit;

pub use infrastructure::repository::metadata_relation_repository_impl::MetadataRelationRepositoryImpl;
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

/// 根据数据库连接字符串构建 `MetadataRelationService`。
pub async fn metadata_relation_service_from_url(
    url: &str,
) -> Result<MetadataRelationService<MetadataRelationRepositoryImpl>, sea_orm::DbErr> {
    let db = Database::connect(url).await?;
    Ok(metadata_relation_service_from_connection(db))
}

/// 使用已有的 `DatabaseConnection` 构建 `MetadataRelationService`。
pub fn metadata_relation_service_from_connection(
    db: DatabaseConnection,
) -> MetadataRelationService<MetadataRelationRepositoryImpl> {
    let repository = MetadataRelationRepositoryImpl::new(db);
    MetadataRelationService::new(repository)
}
