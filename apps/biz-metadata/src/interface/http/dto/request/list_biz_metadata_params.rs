use serde::Deserialize;
use utoipa::IntoParams;

/// BizMetadata 列表查询的分页与过滤参数。
#[derive(Debug, Deserialize, IntoParams, utoipa::ToSchema)]
pub struct BizMetadataListParams {
    /// 每页数量，默认 20。
    pub limit: Option<u64>,
    /// 偏移量，从 0 开始。
    pub offset: Option<u64>,
    /// 可选 code 过滤。
    pub code: Option<String>,
    /// 可选 name 过滤。
    pub name: Option<String>,
}
