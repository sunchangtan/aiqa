use serde::Serialize;
use utoipa::ToSchema;

/// HTTP 层统一响应包装。
#[derive(Debug, Serialize, ToSchema)]
pub struct ResultResponse<T>
where
    T: Serialize + utoipa::ToSchema,
{
    /// 业务状态码，0 表示成功，非 0 表示失败。
    pub code: i32,
    /// 额外的提示信息。
    pub msg: Option<String>,
    /// 具体数据载荷。
    pub data: Option<T>,
}

impl<T> ResultResponse<T>
where
    T: Serialize + utoipa::ToSchema,
{
    /// 构建成功响应。
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            msg: Some("ok".to_string()),
            data: Some(data),
        }
    }

    /// 构建无需数据的成功响应。
    pub fn ok_without_data() -> Self {
        Self {
            code: 0,
            msg: Some("ok".to_string()),
            data: None,
        }
    }

    /// 构建错误响应。
    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: Some(msg.into()),
            data: None,
        }
    }
}
