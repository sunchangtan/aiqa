use crate::domain::biz_metadata::value_object::{
    BizMetadataCode, BizMetadataId, BizMetadataName, BizMetadataStatus, DataClass, ObjectType,
    Source, TenantId, Unit, ValueType, Version,
};
use chrono::{DateTime, Utc};
use domain_core::prelude::{AggregateRoot, Audit, DomainError, Entity, validate_non_empty};
use domain_core::value_object::ValueObject;

/// 元数据聚合根，表示系统中的一个元数据定义实体。
///
/// 该聚合对齐 `biz_metadata` 表（规范 v1.0），并包含以下关键不变式：
/// - `object_type != feature` 时，`data_class/value_type/unit` 必须为空
/// - `object_type == feature` 时，`data_class/value_type` 必须非空
/// - `unit` 仅允许在 `data_class=metric` 下填写
/// - `data_class=identifier` 时，`unit` 必须为空（value_type 的严格集合由门禁工具与 DB CHECK 共同保障）
///
/// # 示例
/// ```
/// use biz_metadata::{BizMetadata, DataClass, TenantId, ValueType};
///
/// let tenant = TenantId::new("default")?;
/// let mut feature = BizMetadata::new_feature(
///     tenant,
///     "company.base.name_cn",
///     "公司中文名",
///     DataClass::Attribute,
///     ValueType::new("string")?,
/// )?;
/// assert!(feature.data_class().is_some());
/// # Ok::<(), domain_core::domain_error::DomainError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BizMetadata {
    tenant_id: TenantId,
    version: Version,
    id: BizMetadataId,
    code: BizMetadataCode,
    name: BizMetadataName,
    description: Option<String>,
    object_type: ObjectType,
    parent_id: Option<BizMetadataId>,
    data_class: Option<DataClass>,
    value_type: Option<ValueType>,
    unit: Option<Unit>,
    status: BizMetadataStatus,
    source: Source,
    audit: Audit,
}

/// 聚合的完整状态快照，用于持久化重建。
#[derive(Debug, Clone)]
pub struct MetadataSnapshot {
    pub tenant_id: TenantId,
    pub version: Version,
    pub id: BizMetadataId,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub object_type: ObjectType,
    pub parent_id: Option<BizMetadataId>,
    pub data_class: Option<DataClass>,
    pub value_type: Option<String>,
    pub unit: Option<Unit>,
    pub status: BizMetadataStatus,
    pub source: Source,
    pub audit: Audit,
}

impl BizMetadata {
    /// 构造一个新的非 feature 节点（entity/event/relation/document）。
    pub fn new_node(
        tenant_id: TenantId,
        code: impl Into<String>,
        name: impl Into<String>,
        object_type: ObjectType,
    ) -> Result<Self, DomainError> {
        if object_type == ObjectType::Feature {
            return Err(DomainError::Validation {
                message: "use new_feature() to create object_type=feature".into(),
            });
        }
        let now = Utc::now();
        Self::from_snapshot(MetadataSnapshot {
            tenant_id,
            version: Version::new(1)?,
            id: BizMetadataId::new(0),
            code: code.into(),
            name: name.into(),
            description: None,
            object_type,
            parent_id: None,
            data_class: None,
            value_type: None,
            unit: None,
            status: BizMetadataStatus::Active,
            source: Source::Manual,
            audit: Audit::new(now),
        })
    }

    /// 构造一个新的 feature 节点（字段/特征）。
    pub fn new_feature(
        tenant_id: TenantId,
        code: impl Into<String>,
        name: impl Into<String>,
        data_class: DataClass,
        value_type: ValueType,
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        Self::from_snapshot(MetadataSnapshot {
            tenant_id,
            version: Version::new(1)?,
            id: BizMetadataId::new(0),
            code: code.into(),
            name: name.into(),
            description: None,
            object_type: ObjectType::Feature,
            parent_id: None,
            data_class: Some(data_class),
            value_type: Some(value_type.into_inner()),
            unit: None,
            status: BizMetadataStatus::Active,
            source: Source::Manual,
            audit: Audit::new(now),
        })
    }

    /// 按字段与审计信息重建元数据聚合，保留时间戳与可选删除标记。
    pub fn from_snapshot(snapshot: MetadataSnapshot) -> Result<Self, DomainError> {
        let MetadataSnapshot {
            tenant_id,
            version,
            id,
            code,
            name,
            description,
            object_type,
            parent_id,
            data_class,
            value_type,
            unit,
            status,
            source,
            audit,
        } = snapshot;

        tenant_id.validate()?;
        version.validate()?;
        object_type.validate()?;
        status.validate()?;
        source.validate()?;

        let code = BizMetadataCode::new(code)?;
        let name = BizMetadataName::new(name)?;

        if let Some(desc) = description.as_ref() {
            validate_non_empty(desc, "description")?;
        }

        if let Some(v) = data_class.as_ref() {
            v.validate()?;
        }

        let value_type = value_type.map(ValueType::new).transpose()?;

        if let Some(unit) = unit.as_ref() {
            unit.validate()?;
        }

        Self::validate_scope(object_type, data_class, value_type.as_ref(), unit.as_ref())?;

        Ok(Self {
            tenant_id,
            version,
            id,
            code,
            name,
            description,
            object_type,
            parent_id,
            data_class,
            value_type,
            unit,
            status,
            source,
            audit,
        })
    }

    fn validate_scope(
        object_type: ObjectType,
        data_class: Option<DataClass>,
        value_type: Option<&ValueType>,
        unit: Option<&Unit>,
    ) -> Result<(), DomainError> {
        match object_type {
            ObjectType::Feature => {
                if data_class.is_none() || value_type.is_none() {
                    return Err(DomainError::Validation {
                        message: "object_type=feature requires non-empty data_class and value_type"
                            .into(),
                    });
                }
            }
            _ => {
                if data_class.is_some() || value_type.is_some() || unit.is_some() {
                    return Err(DomainError::Validation {
                        message: "object_type!=feature must keep data_class/value_type/unit empty"
                            .into(),
                    });
                }
            }
        }

        if unit.is_some() && data_class != Some(DataClass::Metric) {
            return Err(DomainError::Validation {
                message: format!("unit not allowed when data_class is {data_class:?}"),
            });
        }

        if data_class == Some(DataClass::Identifier) && unit.is_some() {
            return Err(DomainError::Validation {
                message: "identifier unit must be empty".into(),
            });
        }

        Ok(())
    }

    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn id(&self) -> BizMetadataId {
        self.id
    }

    pub fn code(&self) -> &BizMetadataCode {
        &self.code
    }

    pub fn name(&self) -> &BizMetadataName {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }

    pub fn parent_id(&self) -> Option<BizMetadataId> {
        self.parent_id
    }

    pub fn data_class(&self) -> Option<DataClass> {
        self.data_class
    }

    pub fn value_type(&self) -> Option<&ValueType> {
        self.value_type.as_ref()
    }

    pub fn unit(&self) -> Option<&Unit> {
        self.unit.as_ref()
    }

    pub fn is_identifier(&self) -> bool {
        self.object_type == ObjectType::Feature && self.data_class == Some(DataClass::Identifier)
    }

    pub fn status(&self) -> BizMetadataStatus {
        self.status
    }

    pub fn source(&self) -> Source {
        self.source
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.audit.created_at()
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.audit.updated_at()
    }

    pub fn delete_at(&self) -> Option<DateTime<Utc>> {
        self.audit.delete_at()
    }

    pub fn is_deleted(&self) -> bool {
        self.audit.is_deleted()
    }

    pub fn rename(&mut self, name: BizMetadataName) -> Result<(), DomainError> {
        name.validate()?;
        self.name = name;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    pub fn set_description(&mut self, description: Option<String>) -> Result<(), DomainError> {
        if let Some(desc) = description.as_ref() {
            validate_non_empty(desc, "description")?;
        }
        self.description = description;
        self.bump_updated_at(Utc::now())
    }

    pub fn set_parent_id(&mut self, parent_id: Option<BizMetadataId>) -> Result<(), DomainError> {
        self.parent_id = parent_id;
        self.bump_updated_at(Utc::now())
    }

    pub fn change_data_class(&mut self, data_class: DataClass) -> Result<(), DomainError> {
        if self.object_type != ObjectType::Feature {
            return Err(DomainError::Validation {
                message: "non-feature node cannot set data_class".into(),
            });
        }
        data_class.validate()?;
        Self::validate_scope(
            self.object_type,
            Some(data_class),
            self.value_type.as_ref(),
            self.unit.as_ref(),
        )?;
        self.data_class = Some(data_class);
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    pub fn change_value_type(&mut self, value_type: ValueType) -> Result<(), DomainError> {
        if self.object_type != ObjectType::Feature {
            return Err(DomainError::Validation {
                message: "non-feature node cannot set value_type".into(),
            });
        }
        value_type.validate()?;
        Self::validate_scope(
            self.object_type,
            self.data_class,
            Some(&value_type),
            self.unit.as_ref(),
        )?;
        self.value_type = Some(value_type);
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    pub fn set_unit(&mut self, unit: Option<Unit>) -> Result<(), DomainError> {
        if self.object_type != ObjectType::Feature {
            return Err(DomainError::Validation {
                message: "non-feature node cannot set unit".into(),
            });
        }
        if let Some(ref u) = unit {
            u.validate()?;
        }
        Self::validate_scope(
            self.object_type,
            self.data_class,
            self.value_type.as_ref(),
            unit.as_ref(),
        )?;
        self.unit = unit;
        self.bump_updated_at(Utc::now())
    }

    pub fn change_status(&mut self, status: BizMetadataStatus) -> Result<(), DomainError> {
        status.validate()?;
        self.status = status;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    pub fn change_source(&mut self, source: Source) -> Result<(), DomainError> {
        source.validate()?;
        self.source = source;
        self.bump_updated_at(Utc::now())
    }

    pub fn mark_deleted(&mut self, delete_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.mark_deleted(delete_at)
    }

    fn bump_updated_at(&mut self, updated_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.bump_updated(updated_at)
    }
}

impl Entity for BizMetadata {
    type Id = BizMetadataId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl AggregateRoot for BizMetadata {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn constructs_metadata_with_valid_inputs() {
        let biz_metadata = BizMetadata::new_feature(
            TenantId::new("default").unwrap(),
            "code",
            "name",
            DataClass::Attribute,
            ValueType::new("string").unwrap(),
        )
        .expect("valid biz_metadata");

        assert_eq!(biz_metadata.code().as_str(), "code");
        assert_eq!(biz_metadata.name().as_str(), "name");
        assert_eq!(biz_metadata.value_type().unwrap().as_str(), "string");
        assert_eq!(biz_metadata.data_class().unwrap().as_str(), "attribute");
        assert!(!biz_metadata.is_deleted());
    }

    #[test]
    fn rejects_empty_code() {
        let err = BizMetadata::new_feature(
            TenantId::new("default").unwrap(),
            "",
            "name",
            DataClass::Attribute,
            ValueType::new("string").unwrap(),
        )
        .unwrap_err();

        matches!(err, DomainError::Validation { .. });
    }

    #[test]
    fn prevents_backward_delete_timestamp() {
        let mut biz_metadata = BizMetadata::new_feature(
            TenantId::new("default").unwrap(),
            "code",
            "name",
            DataClass::Attribute,
            ValueType::new("string").unwrap(),
        )
        .unwrap();

        let earlier = biz_metadata.created_at() - Duration::seconds(1);
        assert!(biz_metadata.mark_deleted(earlier).is_err());
    }
}
