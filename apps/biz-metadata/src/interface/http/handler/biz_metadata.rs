use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode},
};

use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::interface::http::{
    dto::{
        request::{
            BizMetadataListParams, CreateBizMetadataRequest, DeleteBizMetadataParams,
            UpdateBizMetadataRequest,
        },
        response::{BizMetadataResponse, EmptyPayload, PageResultResponse, ResultResponse},
    },
    error::{ApiError, from_domain_err, not_found, to_api_error},
    mapper::{BizMetadataDtoMapper, HttpError},
};

use crate::interface::http::state::AppState;

pub(crate) const BIZ_METADATA_CONTEXT: &str = "/biz_metadata";

#[utoipa::path(
    post,
    context_path = BIZ_METADATA_CONTEXT,
    path = "/",
    request_body = CreateBizMetadataRequest,
    responses(
        (status = 201, body = ResultResponse<BizMetadataResponse>, description = "Created, Location header set"),
        (status = 400, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata"
)]
/// 创建一条业务元数据定义（节点或特征），用于后续业务语义建模与检索。
pub async fn create_biz_metadata(
    State(state): State<AppState>,
    Json(payload): Json<CreateBizMetadataRequest>,
) -> Result<
    (
        StatusCode,
        [(axum::http::header::HeaderName, HeaderValue); 1],
        Json<ResultResponse<BizMetadataResponse>>,
    ),
    ApiError,
> {
    let service = &state.biz_metadata_service;
    let cmd = BizMetadataDtoMapper::map_to_create_command(payload).map_err(to_api_error)?;
    let created = service
        .create_biz_metadata(cmd)
        .await
        .map_err(from_domain_err)?;

    let location = format!("/biz_metadata/{}", created.id().value());
    let location_header = HeaderValue::from_str(&location)
        .map_err(|_| to_api_error(HttpError::bad_request("invalid Location header")))?;

    Ok((
        StatusCode::CREATED,
        [(axum::http::header::LOCATION, location_header)],
        Json(ResultResponse::ok(BizMetadataDtoMapper::map_to_response(
            created,
        ))),
    ))
}

#[utoipa::path(
    put,
    context_path = BIZ_METADATA_CONTEXT,
    path = "/{id}",
    request_body = UpdateBizMetadataRequest,
    params(
        ("id" = i64, Path, description = "BizMetadata ID")
    ),
    responses(
        (status = 200, body = ResultResponse<BizMetadataResponse>, description = "Updated"),
        (status = 400, body = ResultResponse<EmptyPayload>),
        (status = 404, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata"
)]
/// 基于版本号更新指定业务元数据的可变属性（乐观锁）。
pub async fn update_biz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateBizMetadataRequest>,
) -> Result<Json<ResultResponse<BizMetadataResponse>>, ApiError> {
    let service = &state.biz_metadata_service;
    let cmd = BizMetadataDtoMapper::map_to_update_command(id, payload).map_err(to_api_error)?;
    let updated = service
        .update_biz_metadata(cmd)
        .await
        .map_err(from_domain_err)?;

    Ok(Json(ResultResponse::ok(
        BizMetadataDtoMapper::map_to_response(updated),
    )))
}

#[utoipa::path(
    get,
    context_path = BIZ_METADATA_CONTEXT,
    path = "/{id}",
    params(
        ("id" = i64, Path, description = "BizMetadata ID")
    ),
    responses(
        (status = 200, body = ResultResponse<BizMetadataResponse>),
        (status = 404, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata"
)]
/// 按 ID 查询单条业务元数据定义。
pub async fn get_biz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ResultResponse<BizMetadataResponse>>, ApiError> {
    let service = &state.biz_metadata_service;
    let found = service
        .find_biz_metadata_by_id(BizMetadataId::new(id))
        .await
        .map_err(from_domain_err)?
        .ok_or_else(|| not_found("biz_metadata not found"))?;
    Ok(Json(ResultResponse::ok(
        BizMetadataDtoMapper::map_to_response(found),
    )))
}

#[utoipa::path(
    delete,
    context_path = BIZ_METADATA_CONTEXT,
    path = "/{id}",
    params(
        ("id" = i64, Path, description = "BizMetadata ID"),
        DeleteBizMetadataParams
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata"
)]
/// 基于版本号删除（软删）指定业务元数据定义。
pub async fn delete_biz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<DeleteBizMetadataParams>,
) -> Result<StatusCode, ApiError> {
    let version = crate::domain::biz_metadata::value_object::Version::new(params.version)
        .map_err(|e| to_api_error(HttpError::bad_request(e.to_string())))?;
    state
        .biz_metadata_service
        .delete_biz_metadata(BizMetadataId::new(id), version)
        .await
        .map_err(from_domain_err)?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    context_path = BIZ_METADATA_CONTEXT,
    path = "/",
    params(
        BizMetadataListParams
    ),
    responses(
        (status = 200, body = ResultResponse<PageResultResponse<BizMetadataResponse>>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata"
)]
/// 分页查询业务元数据定义列表。
pub async fn list_biz_metadata(
    State(state): State<AppState>,
    Query(params): Query<BizMetadataListParams>,
) -> Result<Json<ResultResponse<PageResultResponse<BizMetadataResponse>>>, ApiError> {
    let query = BizMetadataDtoMapper::map_to_query_request(params);

    let page = state
        .biz_metadata_service
        .query_biz_metadata(query)
        .await
        .map_err(from_domain_err)?;

    let resp_page = BizMetadataDtoMapper::map_to_page_response(page);
    Ok(Json(ResultResponse::ok(resp_page)))
}
