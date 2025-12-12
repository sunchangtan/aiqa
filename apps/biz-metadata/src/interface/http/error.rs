use axum::{Json, http::StatusCode};

use crate::interface::http::dto::response::{EmptyPayload, ResultResponse};
use crate::interface::http::mapper::{HttpError, map_domain_error};

/// 统一的 API 错误响应类型，包含状态码与统一包装的响应体。
pub type ApiError = (StatusCode, Json<ResultResponse<EmptyPayload>>);

/// 将 HTTP 层错误转换为标准 API 错误。
pub fn to_api_error(err: HttpError) -> ApiError {
    err.into_response()
}

/// 将领域错误映射为标准 API 错误。
pub fn from_domain_err(err: domain_core::domain_error::DomainError) -> ApiError {
    map_domain_error(err).into_response()
}

/// 404 错误辅助方法，附带自定义消息。
pub fn not_found(message: impl Into<String>) -> ApiError {
    HttpError {
        status: StatusCode::NOT_FOUND,
        code: StatusCode::NOT_FOUND.as_u16() as i32,
        message: message.into(),
    }
    .into_response()
}
