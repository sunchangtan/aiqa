use crate::domain::metadata::value_object::MetadataId;
use crate::domain::metadata_relation::value_object::MetadataRelationId;

/// 创建元数据关系的指令。
///
/// # Examples
/// ```
/// use metadata::{CreateMetadataRelationCommand, MetadataId, MetadataRelationId};
///
/// let cmd = CreateMetadataRelationCommand {
///     id: MetadataRelationId::new(1),
///     from_metadata_id: MetadataId::new(10),
///     to_metadata_id: MetadataId::new(11),
/// };
/// assert_eq!(cmd.from_metadata_id.value(), 10);
/// ```
#[derive(Debug, Clone)]
pub struct CreateMetadataRelationCommand {
    pub id: MetadataRelationId,
    pub from_metadata_id: MetadataId,
    pub to_metadata_id: MetadataId,
}
