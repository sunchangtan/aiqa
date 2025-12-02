use chrono::{FixedOffset, Utc};
use sea_orm::ActiveValue::Set;

use crate::domain::metadata::value_object::MetadataId;
use crate::domain::metadata_relation::MetadataRelation;
use crate::domain::metadata_relation::value_object::MetadataRelationId;
use crate::infrastructure::persistence::entity::metadata_relation;
use crate::infrastructure::persistence::mapper::{ActiveModelMapper, EntityMapper};
use domain_core::prelude::DomainError;

/// MetadataRelation 的持久化与领域映射器。
pub struct MetadataRelationMapper;

impl EntityMapper<metadata_relation::Model, MetadataRelation> for MetadataRelationMapper {
    fn map_to_domain(model: &metadata_relation::Model) -> Result<MetadataRelation, DomainError> {
        MetadataRelation::reconstruct(
            MetadataRelationId::from(model.id),
            MetadataId::from(model.from_metadata_id),
            MetadataId::from(model.to_metadata_id),
            model.created_at.with_timezone(&Utc),
            model.updated_at.with_timezone(&Utc),
            model.delete_at.map(|t| t.with_timezone(&Utc)),
        )
    }
}

impl ActiveModelMapper<MetadataRelation, metadata_relation::ActiveModel>
    for MetadataRelationMapper
{
    fn map_to_active_model(
        relation: &MetadataRelation,
    ) -> Result<metadata_relation::ActiveModel, DomainError> {
        let tz = FixedOffset::east_opt(0).expect("UTC offset");

        Ok(metadata_relation::ActiveModel {
            id: Set(i64::from(relation.id())),
            from_metadata_id: Set(i64::from(relation.source_id())),
            to_metadata_id: Set(i64::from(relation.target_id())),
            created_at: Set(relation.created_at().with_timezone(&tz)),
            updated_at: Set(relation.updated_at().with_timezone(&tz)),
            delete_at: Set(relation.delete_at().map(|d| d.with_timezone(&tz))),
        })
    }
}
