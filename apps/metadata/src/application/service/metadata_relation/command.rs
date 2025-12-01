use crate::domain::metadata::value_object::MetadataId;
use crate::domain::metadata_relation::value_object::MetadataRelationId;

/// 创建元数据关系的指令。
#[derive(Debug, Clone)]
pub struct CreateMetadataRelationCommand {
    pub id: MetadataRelationId,
    pub from_metadata_id: MetadataId,
    pub to_metadata_id: MetadataId,
}

/// 更新（重连）元数据关系的指令。
#[derive(Debug, Clone)]
pub struct RelinkMetadataRelationCommand {
    pub id: MetadataRelationId,
    pub from_metadata_id: MetadataId,
    pub to_metadata_id: MetadataId,
}
