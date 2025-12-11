use axum::{Json, http::StatusCode};

use crate::interface::http::dto::response::{EmptyPayload, ResultResponse};
use domain_core::domain_error::DomainError;

/// HTTP 层标准化错误，便于转换为响应体。
#[derive(Debug)]
pub struct HttpError {
    pub status: StatusCode,
    pub code: i32,
    pub message: String,
}

impl HttpError {
    /// 400 错误。
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: StatusCode::BAD_REQUEST.as_u16() as i32,
            message: message.into(),
        }
    }

    /// 转换为统一响应格式。
    pub fn into_response(self) -> (StatusCode, Json<ResultResponse<EmptyPayload>>) {
        (
            self.status,
            Json(ResultResponse::error(self.code, self.message)),
        )
    }
}

/// 将领域错误映射为 HTTP 错误。
pub fn map_domain_error(err: DomainError) -> HttpError {
    match err {
        DomainError::Validation { message } | DomainError::InvariantViolation { message } => {
            HttpError::bad_request(message)
        }
        DomainError::Persistence { message } => HttpError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i32,
            message,
        },
    }
}
