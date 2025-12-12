use serde::Deserialize;
use utoipa::ToSchema;

/// 创建 BizMetadataAlias 的请求体。
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBizMetadataAliasRequest {
    pub metadata_id: i64,
    pub alias: String,
    /// manual / auto_mine / log / embedding
    pub source: Option<String>,
    /// 匹配权重 0-100。
    pub weight: Option<i32>,
    /// 是否首选别名。
    pub is_primary: Option<bool>,
    /// 语言编码，默认 zh-CN。
    pub language: Option<String>,
}
