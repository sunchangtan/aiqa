use serde::Deserialize;
use utoipa::ToSchema;

/// 更新 BizMetadata 的请求体，未赋值的字段保持不变。
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBizMetadataRequest {
    /// 版本号（乐观锁），必须与服务端当前版本一致。
    pub version: i32,
    /// 可选名称。
    pub name: Option<String>,
    /// 可选描述。
    pub description: Option<Option<String>>,
    /// `object_type=feature` 时可更新：attribute/metric/text/object/array/identifier。
    pub data_class: Option<String>,
    /// 可选值类型。
    pub value_type: Option<String>,
    /// 可选单位。
    pub unit: Option<Option<String>>,
    /// 可选父节点。
    pub parent_id: Option<Option<i64>>,
    /// 可选状态：active/deprecated。
    pub status: Option<String>,
    /// 可选来源：manual/auto_mine/api_sync。
    pub source: Option<String>,
}
