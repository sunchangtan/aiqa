use serde::Deserialize;
use utoipa::ToSchema;

/// 更新 BizMetadata 的请求体，未赋值的字段保持不变。
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBizMetadataRequest {
    /// 可选名称。
    pub name: Option<String>,
    /// 可选描述。
    pub description: Option<Option<String>>,
    /// 可选元类型：entity/event/field/relation。
    pub meta_type: Option<String>,
    /// 可选数据分类：metric/dimension/text/group。
    pub data_class: Option<String>,
    /// 可选值类型。
    pub value_type: Option<String>,
    /// 可选单位。
    pub unit: Option<Option<String>>,
    /// 可选归属父节点。
    pub owner_id: Option<Option<i64>>,
    /// 可选标识符标记。
    pub is_identifier: Option<bool>,
    /// 可选状态：active/deprecated/draft。
    pub status: Option<String>,
}
