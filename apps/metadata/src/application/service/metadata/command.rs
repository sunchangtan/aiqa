use serde_json::Value as JsonValue;

use crate::domain::metadata::value_object::{MetadataCapabilities, MetadataId, MetadataType};

/// 创建命令，封装创建所需字段。
pub struct CreateMetadataCommand {
    pub id: MetadataId,
    pub code: String,
    pub name: String,
    pub metadata_type: MetadataType,
    pub value_type: String,
    pub capabilities: Option<MetadataCapabilities>,
    pub extra: Option<JsonValue>,
}

/// 更新命令，包含可选的增量字段。
pub struct UpdateMetadataCommand {
    pub id: MetadataId,
    pub name: Option<String>,
    pub metadata_type: Option<MetadataType>,
    pub value_type: Option<String>,
    pub capabilities: Option<MetadataCapabilities>,
    pub extra: ExtraUpdate,
}

impl Default for UpdateMetadataCommand {
    fn default() -> Self {
        Self {
            id: MetadataId::new(0),
            name: None,
            metadata_type: None,
            value_type: None,
            capabilities: None,
            extra: ExtraUpdate::Keep,
        }
    }
}

/// 扩展字段更新策略。
#[derive(Default)]
pub enum ExtraUpdate {
    /// 保持现状，不修改。
    #[default]
    Keep,
    /// 设置为给定 JSON 值。
    Set(JsonValue),
    /// 清空字段。
    Clear,
}
