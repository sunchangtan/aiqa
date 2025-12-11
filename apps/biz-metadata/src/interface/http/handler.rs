//! AI 元数据 HTTP 入口的处理器集合，负责将 REST 请求转为应用服务调用。
//! 使用统一的 `ResultResponse<T>` 包装，便于 AI/前端消费和 OpenAPI 生成。
use axum::{
    Json,
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode},
};

use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::interface::http::{
    dto::{
        request::{BizMetadataListParams, CreateBizMetadataRequest, UpdateBizMetadataRequest},
        response::{BizMetadataResponse, EmptyPayload, PageResultResponse, ResultResponse},
    },
    mapper::{BizMetadataDtoMapper, HttpError, map_domain_error},
};

use super::state::AppState;

#[utoipa::path(
    post,
    path = "/",
    request_body = CreateBizMetadataRequest,
    responses(
        (status = 201, body = ResultResponse<BizMetadataResponse>, description = "Created, Location header set"),
        (status = 400, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata"
)]
/// 创建元数据，返回创建结果并在 `Location` 头中指向新资源。
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
    let service = &state.service;
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
/// 更新指定 ID 的元数据，仅应用被赋值的字段，并返回最新状态。
pub async fn update_biz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateBizMetadataRequest>,
) -> Result<Json<ResultResponse<BizMetadataResponse>>, ApiError> {
    let service = &state.service;
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
/// 查询指定 ID 的元数据，不存在时返回 404。
pub async fn get_biz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ResultResponse<BizMetadataResponse>>, ApiError> {
    let service = &state.service;
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
    path = "/{id}",
    params(
        ("id" = i64, Path, description = "BizMetadata ID")
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata"
)]
/// 删除指定 ID 的元数据，成功时返回 204 无内容。
pub async fn delete_biz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .service
        .delete_biz_metadata(BizMetadataId::new(id))
        .await
        .map_err(from_domain_err)?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
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
/// 按分页与可选过滤条件查询元数据列表，返回分页结果。
pub async fn list_biz_metadata(
    State(state): State<AppState>,
    Query(params): Query<BizMetadataListParams>,
) -> Result<Json<ResultResponse<PageResultResponse<BizMetadataResponse>>>, ApiError> {
    let query = BizMetadataDtoMapper::map_to_query_request(params);

    let page = state
        .service
        .query_biz_metadata(query)
        .await
        .map_err(from_domain_err)?;

    let resp_page = BizMetadataDtoMapper::map_to_page_response(page);
    Ok(Json(ResultResponse::ok(resp_page)))
}

type ApiError = (StatusCode, Json<ResultResponse<EmptyPayload>>);

fn to_api_error(err: HttpError) -> ApiError {
    err.into_response()
}

fn from_domain_err(err: domain_core::domain_error::DomainError) -> ApiError {
    map_domain_error(err).into_response()
}

fn not_found(message: impl Into<String>) -> ApiError {
    HttpError {
        status: StatusCode::NOT_FOUND,
        code: StatusCode::NOT_FOUND.as_u16() as i32,
        message: message.into(),
    }
    .into_response()
}
