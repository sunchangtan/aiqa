use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::domain::biz_metadata_alias::value_object::BizMetadataAliasId;
use crate::interface::http::dto::{
    request::{
        BizMetadataAliasListParams, CreateBizMetadataAliasRequest, UpdateBizMetadataAliasRequest,
    },
    response::{
        BizMetadataAliasPageResponseBody, BizMetadataAliasResponse, BizMetadataAliasResponseBody,
        EmptyPayload, PageResultResponse, ResultResponse,
    },
};
use crate::interface::http::error::{ApiError, from_domain_err, not_found, to_api_error};
use crate::interface::http::mapper::BizMetadataAliasDtoMapper;
use crate::interface::http::state::AppState;

#[utoipa::path(
    post,
    path = "/biz_metadata_alias",
    request_body = CreateBizMetadataAliasRequest,
    responses(
        (status = 201, body = ResultResponse<BizMetadataAliasResponse>, description = "Created"),
        (status = 400, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata_alias"
)]
pub async fn create_biz_metadata_alias(
    State(state): State<AppState>,
    Json(payload): Json<CreateBizMetadataAliasRequest>,
) -> Result<(StatusCode, Json<BizMetadataAliasResponseBody>), ApiError> {
    let cmd = BizMetadataAliasDtoMapper::map_to_create_command(payload).map_err(to_api_error)?;
    let created = state
        .biz_metadata_alias_service
        .create_alias(cmd)
        .await
        .map_err(from_domain_err)?;
    Ok((
        StatusCode::CREATED,
        Json(ResultResponse::ok(
            BizMetadataAliasDtoMapper::map_to_response(created),
        )),
    ))
}

#[utoipa::path(
    put,
    path = "/biz_metadata_alias/{id}",
    request_body = UpdateBizMetadataAliasRequest,
    params(
        ("id" = i64, Path, description = "BizMetadataAlias ID")
    ),
    responses(
        (status = 200, body = ResultResponse<BizMetadataAliasResponse>, description = "Updated"),
        (status = 400, body = ResultResponse<EmptyPayload>),
        (status = 404, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata_alias"
)]
pub async fn update_biz_metadata_alias(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateBizMetadataAliasRequest>,
) -> Result<Json<BizMetadataAliasResponseBody>, ApiError> {
    let cmd =
        BizMetadataAliasDtoMapper::map_to_update_command(id, payload).map_err(to_api_error)?;
    let updated = state
        .biz_metadata_alias_service
        .update_alias(cmd)
        .await
        .map_err(from_domain_err)?;
    Ok(Json(ResultResponse::ok(
        BizMetadataAliasDtoMapper::map_to_response(updated),
    )))
}

#[utoipa::path(
    get,
    path = "/biz_metadata_alias/{id}",
    params(
        ("id" = i64, Path, description = "BizMetadataAlias ID")
    ),
    responses(
        (status = 200, body = ResultResponse<BizMetadataAliasResponse>),
        (status = 404, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata_alias"
)]
pub async fn get_biz_metadata_alias(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<BizMetadataAliasResponseBody>, ApiError> {
    let found = state
        .biz_metadata_alias_service
        .find_by_id(BizMetadataAliasId::new(id))
        .await
        .map_err(from_domain_err)?
        .ok_or_else(|| not_found("biz_metadata_alias not found"))?;
    Ok(Json(ResultResponse::ok(
        BizMetadataAliasDtoMapper::map_to_response(found),
    )))
}

#[utoipa::path(
    delete,
    path = "/biz_metadata_alias/{id}",
    params(
        ("id" = i64, Path, description = "BizMetadataAlias ID")
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, body = ResultResponse<EmptyPayload>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata_alias"
)]
pub async fn delete_biz_metadata_alias(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    state
        .biz_metadata_alias_service
        .delete_alias(BizMetadataAliasId::new(id))
        .await
        .map_err(from_domain_err)?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/biz_metadata_alias",
    params(
        BizMetadataAliasListParams
    ),
    responses(
        (status = 200, body = ResultResponse<PageResultResponse<BizMetadataAliasResponse>>),
        (status = 500, body = ResultResponse<EmptyPayload>)
    ),
    tag = "biz_metadata_alias"
)]
pub async fn list_biz_metadata_alias(
    State(state): State<AppState>,
    Query(params): Query<BizMetadataAliasListParams>,
) -> Result<Json<BizMetadataAliasPageResponseBody>, ApiError> {
    let query = BizMetadataAliasDtoMapper::map_to_query_request(params);
    let page = state
        .biz_metadata_alias_service
        .query_alias(query)
        .await
        .map_err(from_domain_err)?;
    let resp_page = BizMetadataAliasDtoMapper::map_to_page_response(page);
    Ok(Json(ResultResponse::ok(resp_page)))
}
