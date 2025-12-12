use serde::Deserialize;
use utoipa::ToSchema;

/// 更新 BizMetadataAlias 的请求体。
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBizMetadataAliasRequest {
    /// 可选元数据 ID。
    pub metadata_id: Option<i64>,
    /// 可选别名文本。
    pub alias: Option<String>,
    /// 可选来源：manual/auto_mine/log/embedding。
    pub source: Option<String>,
    /// 可选匹配权重。
    pub weight: Option<i32>,
    /// 可选首选标记。
    pub is_primary: Option<bool>,
    /// 可选语言编码。
    pub language: Option<String>,
}
