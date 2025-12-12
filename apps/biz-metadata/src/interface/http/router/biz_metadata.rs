use axum::Router;
use utoipa::Modify;

use crate::application::service::biz_metadata::BizMetadataService;
use crate::infrastructure::persistence::repository::biz_metadata_repository_impl::BizMetadataRepositoryImpl;

/// BizMetadata 路由的统一前缀。
pub const BIZ_METADATA_BASE: &str = "/biz_metadata";

/// 为 OpenAPI 文档追加路径前缀，保证文档与实际路由一致。
struct BizMetadataPathPrefix;

impl Modify for BizMetadataPathPrefix {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut new_paths = utoipa::openapi::path::Paths::new();
        for (path, item) in std::mem::take(&mut openapi.paths.paths) {
            new_paths
                .paths
                .insert(format!("{BIZ_METADATA_BASE}{path}"), item);
        }
        openapi.paths = new_paths;
        openapi.servers = Some(vec![
            utoipa::openapi::server::ServerBuilder::new()
                .url(BIZ_METADATA_BASE)
                .build(),
        ]);
    }
}

// build.rs 生成的 OpenAPI 与路由定义。
include!(concat!(env!("OUT_DIR"), "/api_doc.rs"));

/// 构建带 Swagger UI 的 BizMetadata 路由。
pub fn build_router(service: BizMetadataService<BizMetadataRepositoryImpl>) -> Router {
    use crate::interface::http::state::AppState;
    use std::sync::Arc;
    use utoipa_swagger_ui::SwaggerUi;

    let state = AppState {
        service: Arc::new(service),
    };

    let api = build_generated_router(state);
    let openapi = ApiDoc::openapi();

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", openapi))
        .nest(BIZ_METADATA_BASE, api)
}
