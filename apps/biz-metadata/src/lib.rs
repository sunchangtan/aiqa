use sea_orm::{Database, DatabaseConnection};

pub use application::service::biz_metadata::{
    BizMetadataQueryRequest, BizMetadataService, CreateBizMetadataCommand, FieldUpdate,
    UpdateBizMetadataCommand,
};
pub use application::service::biz_metadata_alias::{
    AliasFieldUpdate, BizMetadataAliasQueryRequest, BizMetadataAliasService,
    CreateBizMetadataAliasCommand, UpdateBizMetadataAliasCommand,
};
pub use domain::biz_metadata::BizMetadata;
pub use domain::biz_metadata::repository::BizMetadataRepository;
pub use domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataStatus, BizMetadataType, DataClass, ValueType,
};
pub use domain::biz_metadata_alias::{
    AliasSource, AliasText, AliasWeight, BizMetadataAlias, BizMetadataAliasId,
    BizMetadataAliasRepository, BizMetadataAliasSnapshot, LanguageCode,
};
pub use domain_core::prelude::Audit;

use infrastructure::persistence::repository::{
    biz_metadata_alias_repository_impl::BizMetadataAliasRepositoryImpl,
    biz_metadata_repository_impl::BizMetadataRepositoryImpl,
};

mod application;
mod domain;
pub mod infrastructure;
pub mod interface;

/// 根据数据库连接字符串构建 `MetadataService`，内部会创建 SeaORM 连接。
pub async fn metadata_service_from_url(
    url: &str,
) -> Result<BizMetadataService<BizMetadataRepositoryImpl>, sea_orm::DbErr> {
    let db = Database::connect(url).await?;
    Ok(metadata_service_from_connection(db))
}

/// 使用已有的 `DatabaseConnection` 构建 `MetadataService`，便于复用连接池。
pub fn metadata_service_from_connection(
    db: DatabaseConnection,
) -> BizMetadataService<BizMetadataRepositoryImpl> {
    let repository = BizMetadataRepositoryImpl::new(db);
    BizMetadataService::new(repository)
}

/// 根据数据库连接构建 BizMetadataService，供 HTTP 适配层使用。
pub fn build_service(db: DatabaseConnection) -> BizMetadataService<BizMetadataRepositoryImpl> {
    let repository = BizMetadataRepositoryImpl::new(db);
    BizMetadataService::new(repository)
}

/// 根据数据库连接构建 BizMetadataAliasService。
pub fn build_alias_service(
    db: DatabaseConnection,
) -> BizMetadataAliasService<BizMetadataAliasRepositoryImpl> {
    let repository = BizMetadataAliasRepositoryImpl::new(db);
    BizMetadataAliasService::new(repository)
}
