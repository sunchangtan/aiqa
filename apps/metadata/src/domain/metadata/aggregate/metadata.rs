use crate::domain::metadata::value_object::{
    MetadataCapabilities, MetadataCode, MetadataId, MetadataName, MetadataType, ValueType,
};
use chrono::{DateTime, Utc};
use domain_core::prelude::{AggregateRoot, Audit, DomainError, Entity};
use domain_core::value_object::ValueObject;
use serde_json::Value as JsonValue;

/// 元数据聚合根，表示系统中的一个元数据定义实体。
///
/// # 示例
/// 创建并更新元数据：
/// ```
/// # use domain_core::domain_error::DomainError;
/// use metadata::{Metadata, MetadataId, MetadataType, ValueType};
///
/// # fn main() -> Result<(), DomainError> {
/// let mut metadata = Metadata::new(
///     MetadataId::new(1),
///     "code",
///     "name",
///     MetadataType::Attribute,
///     "string",
/// )?;
/// metadata.change_value_type(ValueType::new("int")?)?;
/// metadata.mark_deleted(metadata.updated_at())?;
/// assert!(metadata.is_deleted());
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    /// 元数据标识。
    id: MetadataId,
    /// 元数据编码。
    code: MetadataCode,
    /// 展示名称/业务名称。
    name: MetadataName,
    /// 元数据所属的大类（attribute/entity/event）。
    metadata_type: MetadataType,
    /// 单个值类型描述（包含基础、联合、自定义等）。
    value_type: ValueType,
    /// 是否可筛选、可排序等能力开关。
    capabilities: MetadataCapabilities,
    /// 额外的 JSON 扩展信息。
    extra: Option<JsonValue>,
    /// 时间线审计信息。
    audit: Audit,
}

impl Metadata {
    /// 构造一个新的元数据聚合，并执行基础校验。
    pub fn new(
        id: impl Into<MetadataId>,
        code: impl Into<String>,
        name: impl Into<String>,
        metadata_type: MetadataType,
        value_type: impl Into<String>,
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        Self::reconstruct(MetadataReconstructParams {
            id: id.into(),
            code: code.into(),
            name: name.into(),
            metadata_type,
            value_type: value_type.into(),
            capabilities: MetadataCapabilities::default(),
            extra: None,
            audit: Audit::new(now),
        })
    }

    /// 按字段与审计信息重建元数据聚合，保留时间戳与可选删除标记。
    pub fn reconstruct(params: MetadataReconstructParams) -> Result<Self, DomainError> {
        let MetadataReconstructParams {
            id,
            code,
            name,
            metadata_type,
            value_type,
            capabilities,
            extra,
            audit,
        } = params;
        let code = MetadataCode::new(code)?;
        let name = MetadataName::new(name)?;
        let value_type = ValueType::new(value_type)?;

        Ok(Self {
            id,
            code,
            name,
            metadata_type,
            value_type,
            capabilities,
            extra,
            audit,
        })
    }

    /// 返回元数据 ID。
    pub fn id(&self) -> MetadataId {
        self.id
    }

    /// 获取元数据编码。
    pub fn code(&self) -> &MetadataCode {
        &self.code
    }

    /// 获取当前元数据名称。
    pub fn name(&self) -> &MetadataName {
        &self.name
    }

    /// 更新元数据名称，写入前会重新校验。
    pub fn rename(&mut self, name: MetadataName) -> Result<(), DomainError> {
        name.validate()?;
        self.name = name;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    /// 返回元数据所属类别。
    pub fn metadata_type(&self) -> MetadataType {
        self.metadata_type
    }

    /// 修改元数据所属类别。
    /// 为保持领域语义清晰，使用语义化命名而非通用的 `set_` 前缀。
    pub fn change_metadata_type(&mut self, metadata_type: MetadataType) -> Result<(), DomainError> {
        metadata_type.validate()?;
        self.metadata_type = metadata_type;
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

    /// 返回能力开关的快照值。
    pub fn capabilities(&self) -> MetadataCapabilities {
        self.capabilities
    }

    /// 设置所有能力开关。
    pub fn set_capabilities(
        &mut self,
        capabilities: MetadataCapabilities,
    ) -> Result<(), DomainError> {
        self.capabilities = capabilities;
        self.bump_updated_at(Utc::now())?;
        Ok(())
    }

    /// 获取扩展 JSON 数据的引用。
    pub fn extra(&self) -> Option<&JsonValue> {
        self.extra.as_ref()
    }

    /// 设置扩展 JSON 数据，调用方需要自行保证内容合法。
    pub fn set_extra(&mut self, extra: Option<JsonValue>) -> Result<(), DomainError> {
        self.extra = extra;
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

    /// 外部手动触碰更新时间，仍会校验单调性。
    pub fn touch(&mut self, updated_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.bump_updated_at(updated_at)
    }

    /// 内部通用的更新时间写入逻辑，保证不回退。
    fn bump_updated_at(&mut self, updated_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.bump_updated(updated_at)
    }
}

impl Entity for Metadata {
    type Id = MetadataId;

    fn id(&self) -> Self::Id {
        self.id
    }
}
impl AggregateRoot for Metadata {}

/// 重建元数据聚合所需的字段集合。
#[derive(Debug, Clone)]
pub struct MetadataReconstructParams {
    pub id: MetadataId,
    pub code: String,
    pub name: String,
    pub metadata_type: MetadataType,
    pub value_type: String,
    pub capabilities: MetadataCapabilities,
    pub extra: Option<JsonValue>,
    pub audit: Audit,
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn constructs_metadata_with_valid_inputs() {
        let metadata = Metadata::new(
            MetadataId::new(1),
            "code",
            "name",
            MetadataType::Attribute,
            "string",
        )
        .expect("valid metadata");

        assert_eq!(metadata.code().as_str(), "code");
        assert_eq!(metadata.name().as_str(), "name");
        assert_eq!(metadata.value_type().as_str(), "string");
        assert!(!metadata.is_deleted());
    }

    #[test]
    fn rejects_empty_code() {
        let err = Metadata::new(
            MetadataId::new(1),
            "",
            "name",
            MetadataType::Attribute,
            "string",
        )
        .unwrap_err();

        matches!(err, DomainError::Validation { .. });
    }

    #[test]
    fn prevents_backward_delete_timestamp() {
        let mut metadata = Metadata::new(
            MetadataId::new(1),
            "code",
            "name",
            MetadataType::Attribute,
            "string",
        )
        .unwrap();

        let earlier = metadata.created_at() - Duration::seconds(1);
        assert!(metadata.mark_deleted(earlier).is_err());
    }
}
