use serde_json::Value as JsonValue;

use crate::domain::metadata::value_object::{MetadataCapabilities, MetadataId, MetadataType};

/// 创建命令，封装创建所需字段。
///
/// # Examples
/// ```
/// use metadata::{CreateMetadataCommand, MetadataId, MetadataType};
///
/// let cmd = CreateMetadataCommand {
///     id: MetadataId::new(1),
///     code: "user".into(),
///     name: "用户".into(),
///     metadata_type: MetadataType::Attribute,
///     value_type: "string".into(),
///     capabilities: None,
///     extra: None,
/// };
/// assert_eq!(cmd.code, "user");
/// ```
pub struct CreateMetadataCommand {
    pub id: MetadataId,
    pub code: String,
    pub name: String,
    pub metadata_type: MetadataType,
    pub value_type: String,
    pub capabilities: Option<MetadataCapabilities>,
    pub extra: Option<JsonValue>,
}
