use chrono::{FixedOffset, Utc};
use sea_orm::ActiveValue::Set;

use crate::domain::metadata::value_object::MetadataId;
use crate::domain::metadata_relation::MetadataRelation;
use crate::domain::metadata_relation::value_object::MetadataRelationId;
use crate::infrastructure::persistence::entity::metadata_relation;

/// 将持久化模型转换为领域聚合。
pub fn from_entity(model: &metadata_relation::Model) -> MetadataRelation {
    MetadataRelation::reconstruct(
        MetadataRelationId::from(model.id),
        MetadataId::from(model.from_metadata_id),
        MetadataId::from(model.to_metadata_id),
        model.created_at.with_timezone(&Utc),
        model.updated_at.with_timezone(&Utc),
        model.delete_at.map(|t| t.with_timezone(&Utc)),
    )
    .expect("invalid metadata_relation record from persistence")
}

/// 将领域聚合转换为 SeaORM 的 ActiveModel，便于持久化层直接写入。
pub fn to_active_model(relation: &MetadataRelation) -> metadata_relation::ActiveModel {
    let tz = FixedOffset::east_opt(0).expect("UTC offset");

    metadata_relation::ActiveModel {
        id: Set(i64::from(relation.id())),
        from_metadata_id: Set(i64::from(relation.from_id())),
        to_metadata_id: Set(i64::from(relation.to_id())),
        created_at: Set(relation.created_at().with_timezone(&tz)),
        updated_at: Set(relation.updated_at().with_timezone(&tz)),
        delete_at: Set(relation.delete_at().map(|d| d.with_timezone(&tz))),
    }
}
