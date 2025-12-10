use crate::domain::metadata::value_object::MetadataId;
use crate::domain::metadata_relation::value_object::MetadataRelationId;

/// 更新（重连）元数据关系的指令。
///
/// # Examples
/// ```
/// use metadata::{MetadataId, MetadataRelationId, RelinkMetadataRelationCommand};
///
/// let cmd = RelinkMetadataRelationCommand {
///     id: MetadataRelationId::new(2),
///     from_metadata_id: MetadataId::new(5),
///     to_metadata_id: MetadataId::new(6),
/// };
/// assert_eq!(cmd.to_metadata_id.value(), 6);
/// ```
#[derive(Debug, Clone)]
pub struct RelinkMetadataRelationCommand {
    pub id: MetadataRelationId,
    pub from_metadata_id: MetadataId,
    pub to_metadata_id: MetadataId,
}
