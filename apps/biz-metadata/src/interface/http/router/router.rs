use axum::Router;

use crate::application::service::biz_metadata::BizMetadataService;
use crate::application::service::biz_metadata_alias::BizMetadataAliasService;
use crate::infrastructure::persistence::repository::biz_metadata_alias_repository_impl::BizMetadataAliasRepositoryImpl;
use crate::infrastructure::persistence::repository::biz_metadata_repository_impl::BizMetadataRepositoryImpl;
use crate::interface::http::state::AppState;
use tower_http::normalize_path::NormalizePathLayer;

// build.rs 生成的 OpenAPI 定义。
include!(concat!(env!("OUT_DIR"), "/api_doc.rs"));

/// 构建带 Swagger UI 的路由，包含元数据与别名接口。
pub fn build_router(
    biz_metadata_service: BizMetadataService<BizMetadataRepositoryImpl>,
    biz_metadata_alias_service: BizMetadataAliasService<BizMetadataAliasRepositoryImpl>,
) -> Router<()> {
    use std::sync::Arc;
    use utoipa_swagger_ui::SwaggerUi;

    let state = AppState {
        biz_metadata_service: Arc::new(biz_metadata_service),
        biz_metadata_alias_service: Arc::new(biz_metadata_alias_service),
    };

    let openapi = ApiDoc::openapi();
    let swagger: Router<()> = Router::<AppState>::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", openapi))
        .with_state(state.clone());

    let api = generated_routes_biz_metadata(state.clone())
        .merge(generated_routes_biz_metadata_alias(state));

    swagger
        .merge(api)
        .layer(NormalizePathLayer::trim_trailing_slash())
}
