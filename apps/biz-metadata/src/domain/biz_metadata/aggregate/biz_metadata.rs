use crate::domain::biz_metadata::value_object::{
    BizMetadataCode, BizMetadataId, BizMetadataName, BizMetadataStatus, BizMetadataType, DataClass,
    Unit, ValueType, validate_non_empty,
};
use chrono::{DateTime, Utc};
use domain_core::prelude::{AggregateRoot, Audit, DomainError, Entity};
use domain_core::value_object::ValueObject;

/// 元数据聚合根，表示系统中的一个元数据定义实体。
///
/// # 示例
/// 创建并更新元数据：
/// ```
/// use biz_metadata::{BizMetadata, BizMetadataId, BizMetadataType, DataClass, BizMetadataStatus, ValueType};
///
/// # fn main() -> Result<(), domain_core::domain_error::DomainError> {
/// let mut biz_metadata = BizMetadata::new(
///     BizMetadataId::new(1),
///     "company.finance.revenue",
///     "营业收入",
///     BizMetadataType::Attribute,
///     DataClass::Metric,
///     "decimal",
/// )?;
/// biz_metadata.change_value_type(ValueType::new("int")?)?;
/// biz_metadata.change_status(BizMetadataStatus::Deprecated)?;
/// assert_eq!(biz_metadata.data_class().as_str(), "metric");
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BizMetadata {
    /// 元数据标识。
    id: BizMetadataId,
    /// 元数据编码。
    code: BizMetadataCode,
    /// 标准业务名称。
    name: BizMetadataName,
    /// 业务含义/口径描述。
    description: Option<String>,
    /// 元数据所属的大类（attribute/entity/event）。
    metadata_type: BizMetadataType,
    /// 数据分类，描述值的语义类型。
    data_class: DataClass,
    /// 单个值类型描述（包含基础、联合、自定义等）。
    value_type: ValueType,
    /// 单位（仅 metric 时生效）。
    unit: Option<Unit>,
    /// 归属父节点 ID。
    owner_id: Option<BizMetadataId>,
    /// 是否为唯一标识符。
    is_identifier: bool,
    /// 生命周期状态。
    status: BizMetadataStatus,
    /// 时间线审计信息。
    audit: Audit,
}

/// 聚合的完整状态快照，用于持久化重建。
#[derive(Debug, Clone)]
pub struct MetadataSnapshot {
    pub id: BizMetadataId,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub metadata_type: BizMetadataType,
    pub owner_id: Option<BizMetadataId>,
    pub data_class: DataClass,
    pub value_type: String,
    pub unit: Option<Unit>,
    pub is_identifier: bool,
    pub status: BizMetadataStatus,
    pub audit: Audit,
}

impl BizMetadata {
    /// 构造一个新的元数据聚合，并执行基础校验。
    pub fn new(
        id: impl Into<BizMetadataId>,
        code: impl Into<String>,
        name: impl Into<String>,
        metadata_type: BizMetadataType,
        data_class: DataClass,
        value_type: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        Self::from_snapshot(MetadataSnapshot {
            id: id.into(),
            code: code.into(),
            name: name.into(),
            description: None,
            metadata_type,
            owner_id: None,
            data_class,
            value_type: value_type.into(),
            unit: None,
            is_identifier: false,
            status: BizMetadataStatus::Active,
            audit: Audit::new(now),
        })
    }

    /// 按字段与审计信息重建元数据聚合，保留时间戳与可选删除标记。
    pub fn from_snapshot(snapshot: MetadataSnapshot) -> Result<Self, DomainError> {
        let MetadataSnapshot {
            id,
            code,
            name,
            description,
            metadata_type,
            owner_id,
            data_class,
            value_type,
            unit,
            is_identifier,
            status,
            audit,
        } = snapshot;
        let code = BizMetadataCode::new(code)?;
        let name = BizMetadataName::new(name)?;
        let value_type = ValueType::new(value_type)?;
        metadata_type.validate()?;
        data_class.validate()?;
        status.validate()?;

        if let Some(desc) = description.as_ref() {
            validate_non_empty(desc, "description")?;
        }

        if let Some(unit) = unit.as_ref() {
            unit.validate()?;
        }

        Ok(Self {
            id,
            code,
            name,
            metadata_type,
            description,
            owner_id,
            data_class,
            value_type,
            unit,
            is_identifier,
            status,
            audit,
        })
    }

    /// 返回元数据 ID。
    pub fn id(&self) -> BizMetadataId {
        self.id
    }

    /// 获取元数据编码。
    pub fn code(&self) -> &BizMetadataCode {
        &self.code
    }

    /// 获取当前元数据名称。
    pub fn name(&self) -> &BizMetadataName {
        &self.name
    }

    /// 返回业务口径描述。
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// 更新元数据名称，写入前会重新校验。
    pub fn rename(&mut self, name: BizMetadataName) -> Result<(), DomainError> {
        name.validate()?;
        self.name = name;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    /// 设置业务口径描述，空白字符串将被拒绝。
    pub fn set_description(&mut self, description: Option<String>) -> Result<(), DomainError> {
        if let Some(desc) = description.as_ref() {
            validate_non_empty(desc, "description")?;
        }
        self.description = description;
        self.bump_updated_at(Utc::now())
    }

    /// 返回元数据所属类别。
    pub fn metadata_type(&self) -> BizMetadataType {
        self.metadata_type
    }

    /// 修改元数据所属类别。
    /// 为保持领域语义清晰，使用语义化命名而非通用的 `set_` 前缀。
    pub fn change_metadata_type(
        &mut self,
        metadata_type: BizMetadataType,
    ) -> Result<(), DomainError> {
        metadata_type.validate()?;
        self.metadata_type = metadata_type;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    /// 返回数据分类。
    pub fn data_class(&self) -> DataClass {
        self.data_class
    }

    /// 修改数据分类。
    pub fn change_data_class(&mut self, data_class: DataClass) -> Result<(), DomainError> {
        data_class.validate()?;
        self.data_class = data_class;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    /// 返回当前值类型。
    pub fn value_type(&self) -> &ValueType {
        &self.value_type
    }

    /// 更新值类型定义。
    pub fn change_value_type(&mut self, value_type: ValueType) -> Result<(), DomainError> {
        value_type.validate()?;
        self.value_type = value_type;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    /// 返回单位。
    pub fn unit(&self) -> Option<&Unit> {
        self.unit.as_ref()
    }

    /// 更新单位，空白字符串将被拒绝。
    pub fn set_unit(&mut self, unit: Option<Unit>) -> Result<(), DomainError> {
        if let Some(ref u) = unit {
            u.validate()?;
        }
        self.unit = unit;
        self.bump_updated_at(Utc::now())
    }

    /// 返回归属父节点 ID。
    pub fn owner_id(&self) -> Option<BizMetadataId> {
        self.owner_id
    }

    /// 设置归属父节点 ID。
    pub fn set_owner_id(&mut self, owner_id: Option<BizMetadataId>) -> Result<(), DomainError> {
        if let Some(id) = owner_id {
            id.validate()?;
            self.owner_id = Some(id);
        } else {
            self.owner_id = None;
        }
        self.bump_updated_at(Utc::now())
    }

    /// 是否为唯一标识符。
    pub fn is_identifier(&self) -> bool {
        self.is_identifier
    }

    /// 设置唯一标识符标记。
    pub fn set_identifier(&mut self, is_identifier: bool) -> Result<(), DomainError> {
        self.is_identifier = is_identifier;
        self.bump_updated_at(Utc::now())
    }

    /// 返回生命周期状态。
    pub fn status(&self) -> BizMetadataStatus {
        self.status
    }

    /// 修改生命周期状态。
    pub fn change_status(&mut self, status: BizMetadataStatus) -> Result<(), DomainError> {
        status.validate()?;
        self.status = status;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    /// 返回创建时间（UTC）。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.audit.created_at()
    }

    /// 返回最近一次更新时间（UTC）。
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.audit.updated_at()
    }

    /// 返回软删除时间，没有删除则为 `None`。
    pub fn delete_at(&self) -> Option<DateTime<Utc>> {
        self.audit.delete_at()
    }

    /// 判断当前是否已经软删除。
    pub fn is_deleted(&self) -> bool {
        self.audit.is_deleted()
    }

    /// 设置软删除时间，要求晚于 `updated_at`。
    pub fn mark_deleted(&mut self, delete_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.mark_deleted(delete_at)
    }

    /// 内部通用的更新时间写入逻辑，保证不回退。
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
    use chrono::Duration;

    use super::*;

    #[test]
    fn constructs_metadata_with_valid_inputs() {
        let biz_metadata = BizMetadata::new(
            BizMetadataId::new(1),
            "code",
            "name",
            BizMetadataType::Attribute,
            DataClass::Dimension,
            "string",
        )
        .expect("valid biz_metadata");

        assert_eq!(biz_metadata.code().as_str(), "code");
        assert_eq!(biz_metadata.name().as_str(), "name");
        assert_eq!(biz_metadata.value_type().as_str(), "string");
        assert_eq!(biz_metadata.data_class().as_str(), "dimension");
        assert!(!biz_metadata.is_deleted());
    }

    #[test]
    fn rejects_empty_code() {
        let err = BizMetadata::new(
            BizMetadataId::new(1),
            "",
            "name",
            BizMetadataType::Attribute,
            DataClass::Dimension,
            "string",
        )
        .unwrap_err();

        matches!(err, DomainError::Validation { .. });
    }

    #[test]
    fn prevents_backward_delete_timestamp() {
        let mut biz_metadata = BizMetadata::new(
            BizMetadataId::new(1),
            "code",
            "name",
            BizMetadataType::Attribute,
            DataClass::Dimension,
            "string",
        )
        .unwrap();

        let earlier = biz_metadata.created_at() - Duration::seconds(1);
        assert!(biz_metadata.mark_deleted(earlier).is_err());
    }
}
