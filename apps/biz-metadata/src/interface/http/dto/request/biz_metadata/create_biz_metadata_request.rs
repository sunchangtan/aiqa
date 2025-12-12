use serde::Deserialize;
use utoipa::ToSchema;

/// 创建 BizMetadata 的请求体。
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBizMetadataRequest {
    pub code: String,
    pub name: String,
    /// entity / event / field / relation
    pub meta_type: String,
    /// metric / dimension / text / group
    pub data_class: String,
    pub value_type: String,
    pub description: Option<String>,
    pub owner_id: Option<i64>,
    pub unit: Option<String>,
    pub is_identifier: bool,
    /// active / deprecated / draft
    pub status: Option<String>,
}
