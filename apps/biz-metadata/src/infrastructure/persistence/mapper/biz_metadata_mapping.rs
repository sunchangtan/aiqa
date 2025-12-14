use chrono::{FixedOffset, Utc};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};

use crate::domain::biz_metadata::value_object::{
    BizMetadataCode, BizMetadataId, BizMetadataName, BizMetadataStatus, DataClass, ObjectType,
    Source, TenantId, Unit, ValueType as DomainValueType, Version,
};
use crate::domain::biz_metadata::{BizMetadata, MetadataSnapshot};
use crate::infrastructure::persistence::entity::biz_metadata;
use crate::infrastructure::persistence::mapper::{ActiveModelMapper, EntityMapper};
use domain_core::prelude::{Audit, DomainError};

/// Metadata 的持久化与领域映射器。
pub struct BizMetadataMapper;

impl BizMetadataMapper {
    /// 仅将变更字段写入 ActiveModel，未变字段保持 `Unchanged/NotSet`。
    pub fn apply_changes(
        aggregate: &BizMetadata,
        active: &mut biz_metadata::ActiveModel,
    ) -> Result<(), DomainError> {
        let tz = FixedOffset::east_opt(0).expect("UTC offset");

        active.id = Unchanged(i64::from(aggregate.id()));
        active.tenant_id = Unchanged(aggregate.tenant_id().as_str().to_string());
        active.code = Set(aggregate.code().as_str().to_string());
        active.name = Set(aggregate.name().as_str().to_string());
        active.description = Set(aggregate.description().map(|d| d.to_string()));
        active.object_type = Set(aggregate.object_type().as_str().to_string());
        active.parent_id = Set(aggregate.parent_id().map(i64::from));
        active.data_class = Set(aggregate.data_class().map(|v| v.as_str().to_string()));
        active.value_type = Set(aggregate.value_type().map(|v| v.as_str().to_string()));
        active.unit = Set(aggregate.unit().map(|u| u.as_str().to_string()));
        active.status = Set(aggregate.status().as_str().to_string());
        active.source = Set(aggregate.source().as_str().to_string());
        active.created_at = NotSet;
        // updated_at 留给 DB 触发器自动更新，避免覆盖。
        active.updated_at = NotSet;
        active.deleted_at = Set(aggregate.delete_at().map(|d| d.with_timezone(&tz)));
        // version 由仓储层做乐观锁控制（where version=... 并 set version=version+1）。
        active.version = NotSet;

        Ok(())
    }
}

impl EntityMapper<biz_metadata::Model, BizMetadata> for BizMetadataMapper {
    fn map_to_domain(model: &biz_metadata::Model) -> Result<BizMetadata, DomainError> {
        let id = BizMetadataId::from(model.id);
        let tenant_id = TenantId::new(model.tenant_id.clone())?;
        let version = Version::new(model.version)?;
        let code =
            BizMetadataCode::new(model.code.clone()).map_err(|e| DomainError::Validation {
                message: e.to_string(),
            })?;
        let name =
            BizMetadataName::new(model.name.clone()).map_err(|e| DomainError::Validation {
                message: e.to_string(),
            })?;
        let object_type = ObjectType::new(&model.object_type)?;
        let data_class = model
            .data_class
            .as_deref()
            .map(DataClass::new)
            .transpose()
            .map_err(|e| DomainError::Validation {
                message: e.to_string(),
            })?;
        let status =
            BizMetadataStatus::new(&model.status).map_err(|e| DomainError::Validation {
                message: e.to_string(),
            })?;
        let source = Source::new(&model.source)?;
        let value_type = model
            .value_type
            .clone()
            .map(DomainValueType::new)
            .transpose()
            .map_err(|e| DomainError::Validation {
                message: e.to_string(),
            })?;
        let unit = model
            .unit
            .as_ref()
            .map(|u| Unit::new(u.clone()))
            .transpose()
            .map_err(|e| DomainError::Validation {
                message: e.to_string(),
            })?;

        BizMetadata::from_snapshot(MetadataSnapshot {
            tenant_id,
            version,
            id,
            code: code.into_inner(),
            name: name.into_inner(),
            description: model.description.clone(),
            object_type,
            parent_id: model.parent_id.map(BizMetadataId::from),
            data_class,
            value_type: value_type.map(DomainValueType::into_inner),
            unit,
            status,
            source,
            audit: Audit::reconstruct(
                model.created_at.with_timezone(&Utc),
                model.updated_at.with_timezone(&Utc),
                model.deleted_at.map(|d| d.with_timezone(&Utc)),
            )?,
        })
    }
}

impl ActiveModelMapper<BizMetadata, biz_metadata::ActiveModel> for BizMetadataMapper {
    fn map_to_active_model(user: &BizMetadata) -> Result<biz_metadata::ActiveModel, DomainError> {
        let tz = FixedOffset::east_opt(0).expect("UTC offset");
        let id_value = i64::from(user.id());
        let is_new = id_value == 0;

        Ok(biz_metadata::ActiveModel {
            id: if is_new { NotSet } else { Set(id_value) },
            tenant_id: Set(user.tenant_id().as_str().to_string()),
            version: Set(i32::from(user.version())),
            code: Set(user.code().as_str().to_string()),
            name: Set(user.name().as_str().to_string()),
            description: Set(user.description().map(|d| d.to_string())),
            object_type: Set(user.object_type().as_str().to_string()),
            parent_id: Set(user.parent_id().map(i64::from)),
            data_class: Set(user.data_class().map(|v| v.as_str().to_string())),
            value_type: Set(user.value_type().map(|v| v.as_str().to_string())),
            unit: Set(user.unit().map(|u| u.as_str().to_string())),
            status: Set(user.status().as_str().to_string()),
            source: Set(user.source().as_str().to_string()),
            created_at: if is_new {
                NotSet
            } else {
                Set(user.created_at().with_timezone(&tz))
            },
            updated_at: if is_new {
                NotSet
            } else {
                Set(user.updated_at().with_timezone(&tz))
            },
            deleted_at: Set(user.delete_at().map(|d| d.with_timezone(&tz))),
        })
    }
}
