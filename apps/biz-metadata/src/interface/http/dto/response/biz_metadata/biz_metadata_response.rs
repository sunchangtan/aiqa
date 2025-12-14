use crate::domain::biz_metadata::BizMetadata;
use serde::Serialize;
use utoipa::ToSchema;

/// 单条 BizMetadata 的响应体。
#[derive(Debug, Serialize, ToSchema)]
pub struct BizMetadataResponse {
    pub id: i64,
    /// 版本号（乐观锁）。
    pub version: i32,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    /// 语义对象类型（entity/event/relation/document/feature）。
    pub object_type: String,
    /// 可选父节点 ID。
    pub parent_id: Option<i64>,
    /// `object_type=feature` 时返回数据分类。
    pub data_class: Option<String>,
    /// `object_type=feature` 时返回值类型。
    pub value_type: Option<String>,
    pub unit: Option<String>,
    pub status: String,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<BizMetadata> for BizMetadataResponse {
    fn from(src: BizMetadata) -> Self {
        Self {
            id: src.id().value(),
            version: src.version().value(),
            code: src.code().as_str().to_string(),
            name: src.name().as_str().to_string(),
            description: src.description().map(|d| d.to_string()),
            object_type: src.object_type().as_str().to_string(),
            parent_id: src.parent_id().map(|v| v.value()),
            data_class: src.data_class().map(|v| v.as_str().to_string()),
            value_type: src.value_type().map(|v| v.as_str().to_string()),
            unit: src.unit().map(|u| u.as_str().to_string()),
            status: src.status().as_str().to_string(),
            source: src.source().as_str().to_string(),
            created_at: src.created_at().to_rfc3339(),
            updated_at: src.updated_at().to_rfc3339(),
            deleted_at: src.delete_at().map(|d| d.to_rfc3339()),
        }
    }
}
