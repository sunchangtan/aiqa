use serde::Deserialize;
use utoipa::IntoParams;

/// BizMetadataAlias 的分页查询参数。
#[derive(Debug, Deserialize, IntoParams, utoipa::ToSchema)]
pub struct BizMetadataAliasListParams {
    /// 每页数量，默认 20。
    pub limit: Option<u64>,
    /// 偏移量，从 0 开始。
    pub offset: Option<u64>,
    /// 按元数据 ID 过滤。
    pub metadata_id: Option<i64>,
    /// 按别名模糊过滤。
    pub alias: Option<String>,
    /// 按语言过滤。
    pub language: Option<String>,
}
