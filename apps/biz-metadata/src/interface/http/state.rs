use std::sync::Arc;

use crate::application::service::biz_metadata::BizMetadataService;
use crate::application::service::biz_metadata_alias::BizMetadataAliasService;
use crate::infrastructure::persistence::repository::biz_metadata_alias_repository_impl::BizMetadataAliasRepositoryImpl;
use crate::infrastructure::persistence::repository::biz_metadata_repository_impl::BizMetadataRepositoryImpl;

/// Axum 共享状态，持有应用服务。
#[derive(Clone)]
pub struct AppState {
    pub biz_metadata_service: Arc<BizMetadataService<BizMetadataRepositoryImpl>>,
    pub biz_metadata_alias_service: Arc<BizMetadataAliasService<BizMetadataAliasRepositoryImpl>>,
}
