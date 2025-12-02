use chrono::{FixedOffset, Utc};
use sea_orm::ActiveValue::Set;

use crate::domain::metadata::value_object::{
    MetadataCapabilities, MetadataCode, MetadataId, MetadataName, MetadataType,
    ValueType as DomainValueType,
};
use crate::domain::metadata::{Metadata, MetadataReconstructParams};
use crate::infrastructure::persistence::entity::{metadata, sea_orm_active_enums};
use crate::infrastructure::persistence::mapper::{ActiveModelMapper, EntityMapper};
use domain_core::prelude::{Audit, DomainError};

fn to_domain_metadata_type(db_ty: sea_orm_active_enums::MetadataType) -> MetadataType {
    match db_ty {
        sea_orm_active_enums::MetadataType::Attribute => MetadataType::Attribute,
        sea_orm_active_enums::MetadataType::Entity => MetadataType::Entity,
        sea_orm_active_enums::MetadataType::Event => MetadataType::Event,
    }
}

fn to_db_metadata_type(dom_ty: MetadataType) -> sea_orm_active_enums::MetadataType {
    match dom_ty {
        MetadataType::Attribute => sea_orm_active_enums::MetadataType::Attribute,
        MetadataType::Entity => sea_orm_active_enums::MetadataType::Entity,
        MetadataType::Event => sea_orm_active_enums::MetadataType::Event,
    }
}

/// Metadata 的持久化与领域映射器。
pub struct MetadataMapper;

impl EntityMapper<metadata::Model, Metadata> for MetadataMapper {
    fn map_to_domain(model: &metadata::Model) -> Result<Metadata, DomainError> {
        let id = MetadataId::from(model.id);
        let code = MetadataCode::new(model.code.clone()).map_err(|e| DomainError::Validation {
            message: e.to_string(),
        })?;
        let name = MetadataName::new(model.name.clone()).map_err(|e| DomainError::Validation {
            message: e.to_string(),
        })?;
        let metadata_type = to_domain_metadata_type(model.metadata_type.clone());
        let value_type = DomainValueType::new(model.value_type.clone()).map_err(|e| {
            DomainError::Validation {
                message: e.to_string(),
            }
        })?;

        Metadata::reconstruct(MetadataReconstructParams {
            id,
            code: code.into_inner(),
            name: name.into_inner(),
            metadata_type,
            value_type: value_type.into_inner(),
            capabilities: MetadataCapabilities::new(
                model.is_chainable,
                model.is_filterable,
                model.is_sortable,
                model.is_groupable,
                model.is_relation_derived,
            ),
            extra: model.extra.clone(),
            audit: Audit::reconstruct(
                model.created_at.with_timezone(&Utc),
                model.updated_at.with_timezone(&Utc),
                model.delete_at.map(|d| d.with_timezone(&Utc)),
            )?,
        })
    }
}

impl ActiveModelMapper<Metadata, metadata::ActiveModel> for MetadataMapper {
    fn map_to_active_model(user: &Metadata) -> Result<metadata::ActiveModel, DomainError> {
        let tz = FixedOffset::east_opt(0).expect("UTC offset");

        Ok(metadata::ActiveModel {
            id: Set(i64::from(user.id())),
            code: Set(user.code().as_str().to_string()),
            name: Set(user.name().as_str().to_string()),
            metadata_type: Set(to_db_metadata_type(user.metadata_type())),
            value_type: Set(user.value_type().as_str().to_string()),
            is_chainable: Set(user.capabilities().chainable()),
            is_filterable: Set(user.capabilities().filterable()),
            is_sortable: Set(user.capabilities().sortable()),
            is_groupable: Set(user.capabilities().groupable()),
            is_relation_derived: Set(user.capabilities().relation_derived()),
            extra: Set(user.extra().cloned()),
            created_at: Set(user.created_at().with_timezone(&tz)),
            updated_at: Set(user.updated_at().with_timezone(&tz)),
            delete_at: Set(user.delete_at().map(|d| d.with_timezone(&tz))),
        })
    }
}
