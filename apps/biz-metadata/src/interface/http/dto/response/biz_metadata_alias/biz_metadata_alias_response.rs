use crate::domain::biz_metadata_alias::BizMetadataAlias;
use serde::Serialize;
use utoipa::ToSchema;

/// 单条 BizMetadataAlias 的响应体。
#[derive(Debug, Serialize, ToSchema)]
pub struct BizMetadataAliasResponse {
    pub id: i64,
    pub metadata_id: i64,
    pub alias: String,
    pub source: String,
    pub weight: i32,
    pub is_primary: bool,
    pub language: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<BizMetadataAlias> for BizMetadataAliasResponse {
    fn from(src: BizMetadataAlias) -> Self {
        Self {
            id: src.id().value(),
            metadata_id: src.metadata_id().value(),
            alias: src.alias().as_str().to_string(),
            source: src.source().as_str().to_string(),
            weight: src.weight().value(),
            is_primary: src.is_primary(),
            language: src.language().as_str().to_string(),
            created_at: src.created_at().to_rfc3339(),
            updated_at: src.updated_at().to_rfc3339(),
            deleted_at: src.delete_at().map(|d| d.to_rfc3339()),
        }
    }
}
