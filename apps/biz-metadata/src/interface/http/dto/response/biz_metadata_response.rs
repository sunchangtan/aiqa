use crate::domain::biz_metadata::BizMetadata;
use serde::Serialize;
use utoipa::ToSchema;

/// 单条 BizMetadata 的响应体。
#[derive(Debug, Serialize, ToSchema)]
pub struct BizMetadataResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub meta_type: String,
    pub owner_id: Option<i64>,
    pub data_class: String,
    pub value_type: String,
    pub unit: Option<String>,
    pub is_identifier: bool,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<BizMetadata> for BizMetadataResponse {
    fn from(src: BizMetadata) -> Self {
        Self {
            id: src.id().value(),
            code: src.code().as_str().to_string(),
            name: src.name().as_str().to_string(),
            description: src.description().map(|d| d.to_string()),
            meta_type: src.metadata_type().as_str().to_string(),
            owner_id: src.owner_id().map(|v| v.value()),
            data_class: src.data_class().as_str().to_string(),
            value_type: src.value_type().as_str().to_string(),
            unit: src.unit().map(|u| u.as_str().to_string()),
            is_identifier: src.is_identifier(),
            status: src.status().as_str().to_string(),
            created_at: src.created_at().to_rfc3339(),
            updated_at: src.updated_at().to_rfc3339(),
            deleted_at: src.delete_at().map(|d| d.to_rfc3339()),
        }
    }
}
