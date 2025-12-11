use std::sync::Arc;

use crate::application::service::biz_metadata::BizMetadataService;
use crate::infrastructure::persistence::repository::biz_metadata_repository_impl::BizMetadataRepositoryImpl;

/// Axum 共享状态，持有应用服务。
#[derive(Clone)]
pub struct AppState {
    pub service: Arc<BizMetadataService<BizMetadataRepositoryImpl>>,
}
