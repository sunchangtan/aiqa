use serde::Serialize;
use utoipa::ToSchema;

/// 空载荷，用于描述无数据的统一响应。
#[derive(Debug, Serialize, ToSchema)]
pub struct EmptyPayload {}
