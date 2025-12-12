use chrono::{FixedOffset, Utc};
use sea_orm::ActiveValue::{NotSet, Set};

use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::domain::biz_metadata_alias::{
    BizMetadataAlias, BizMetadataAliasSnapshot,
    value_object::{AliasSource, BizMetadataAliasId},
};
use crate::infrastructure::persistence::entity::biz_metadata_alias;
use crate::infrastructure::persistence::mapper::{ActiveModelMapper, EntityMapper};
use domain_core::prelude::{Audit, DomainError};

/// `biz_metadata_alias` 的 ORM 映射器。
pub struct BizMetadataAliasMapper;

impl BizMetadataAliasMapper {
    /// 仅将变更字段写入 ActiveModel，未变字段保持 `Unchanged/NotSet`。
    pub fn apply_changes(
        aggregate: &BizMetadataAlias,
        active: &mut biz_metadata_alias::ActiveModel,
    ) -> Result<(), DomainError> {
        let tz = FixedOffset::east_opt(0).expect("UTC offset");

        active.id = Set(i64::from(aggregate.id()));
        active.metadata_id = Set(i64::from(aggregate.metadata_id()));
        active.alias = Set(aggregate.alias().as_str().to_string());
        active.source = Set(aggregate.source().as_str().to_string());
        active.weight = Set(aggregate.weight().value());
        active.is_primary = Set(aggregate.is_primary());
        active.language = Set(aggregate.language().as_str().to_string());
        active.created_at = Set(aggregate.created_at().with_timezone(&tz));
        active.updated_at = NotSet;
        active.deleted_at = Set(aggregate.delete_at().map(|d| d.with_timezone(&tz)));

        Ok(())
    }
}

impl EntityMapper<biz_metadata_alias::Model, BizMetadataAlias> for BizMetadataAliasMapper {
    fn map_to_domain(model: &biz_metadata_alias::Model) -> Result<BizMetadataAlias, DomainError> {
        let snapshot = BizMetadataAliasSnapshot {
            id: BizMetadataAliasId::from(model.id),
            metadata_id: BizMetadataId::from(model.metadata_id),
            alias: model.alias.clone(),
            source: AliasSource::new(&model.source)?,
            weight: model.weight,
            is_primary: model.is_primary,
            language: model.language.clone(),
            audit: Audit::reconstruct(
                model.created_at.with_timezone(&Utc),
                model.updated_at.with_timezone(&Utc),
                model.deleted_at.map(|d| d.with_timezone(&Utc)),
            )?,
        };
        BizMetadataAlias::from_snapshot(snapshot)
    }
}

impl ActiveModelMapper<BizMetadataAlias, biz_metadata_alias::ActiveModel>
    for BizMetadataAliasMapper
{
    fn map_to_active_model(
        alias: &BizMetadataAlias,
    ) -> Result<biz_metadata_alias::ActiveModel, DomainError> {
        let tz = FixedOffset::east_opt(0).expect("UTC offset");
        let id_value = i64::from(alias.id());
        let is_new = id_value == 0;

        Ok(biz_metadata_alias::ActiveModel {
            id: if is_new { NotSet } else { Set(id_value) },
            metadata_id: Set(i64::from(alias.metadata_id())),
            alias: Set(alias.alias().as_str().to_string()),
            source: Set(alias.source().as_str().to_string()),
            weight: Set(alias.weight().value()),
            is_primary: Set(alias.is_primary()),
            language: Set(alias.language().as_str().to_string()),
            created_at: if is_new {
                NotSet
            } else {
                Set(alias.created_at().with_timezone(&tz))
            },
            updated_at: NotSet,
            deleted_at: Set(alias.delete_at().map(|d| d.with_timezone(&tz))),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::biz_metadata_alias::value_object::AliasText;

    #[test]
    fn round_trip_between_model_and_domain() {
        let alias = BizMetadataAlias::new(BizMetadataId::new(1), "销售额").unwrap();
        let active = BizMetadataAliasMapper::map_to_active_model(&alias).unwrap();
        assert_eq!(active.metadata_id.unwrap(), 1);

        let model = biz_metadata_alias::Model {
            id: 1,
            metadata_id: 1,
            alias: AliasText::new("销售额").unwrap().into_inner(),
            source: "manual".to_string(),
            weight: 10,
            is_primary: true,
            language: "zh-CN".to_string(),
            created_at: alias
                .created_at()
                .with_timezone(&FixedOffset::east_opt(0).unwrap()),
            updated_at: alias
                .updated_at()
                .with_timezone(&FixedOffset::east_opt(0).unwrap()),
            deleted_at: None,
        };
        let domain = BizMetadataAliasMapper::map_to_domain(&model).unwrap();
        assert_eq!(domain.alias().as_str(), "销售额");
        assert!(domain.is_primary());
        assert_eq!(domain.weight().value(), 10);
    }
}
