use serde::Deserialize;
use utoipa::ToSchema;

/// 创建 BizMetadata 的请求体。
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBizMetadataRequest {
    /// 业务编码（点分层级）。
    pub code: String,
    /// 业务名称。
    pub name: String,
    /// 语义对象类型：entity/event/relation/document/feature。
    pub object_type: String,
    /// 可选描述。
    pub description: Option<String>,
    /// 可选父节点 ID。
    pub parent_id: Option<i64>,
    /// `object_type=feature` 时必填：attribute/metric/text/object/array/identifier。
    pub data_class: Option<String>,
    /// `object_type=feature` 时必填：值类型（例如 string/int/json<object:S>）。
    pub value_type: Option<String>,
    /// `data_class=metric` 时可填单位。
    pub unit: Option<String>,
    /// 可选状态：active/deprecated。
    pub status: Option<String>,
    /// 可选来源：manual/auto_mine/api_sync。
    pub source: Option<String>,
}
