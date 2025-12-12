use chrono::{DateTime, Utc};
use domain_core::prelude::{AggregateRoot, Audit, DomainError, Entity};
use domain_core::value_object::ValueObject;

use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::domain::biz_metadata_alias::value_object::{
    AliasSource, AliasText, AliasWeight, BizMetadataAliasId, LanguageCode,
};

/// 元数据别名聚合，描述一个标准元数据的自然语言同义词。
///
/// # 示例
/// 创建别名并调整权重：
/// ```
/// use biz_metadata::{BizMetadataAlias, BizMetadataId};
///
/// # fn main() -> Result<(), domain_core::domain_error::DomainError> {
/// let mut alias = BizMetadataAlias::new(BizMetadataId::new(1), "营收")?;
/// alias.change_weight(80)?;
/// assert_eq!(alias.alias().as_str(), "营收");
/// assert_eq!(alias.weight().value(), 80);
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BizMetadataAlias {
    /// 别名标识。
    id: BizMetadataAliasId,
    /// 对应的元数据 ID。
    metadata_id: BizMetadataId,
    /// 自然语言别名。
    alias: AliasText,
    /// 别名来源。
    source: AliasSource,
    /// 匹配权重。
    weight: AliasWeight,
    /// 是否首选别名。
    is_primary: bool,
    /// 语言编码。
    language: LanguageCode,
    /// 审计信息。
    audit: Audit,
}

/// 聚合快照，用于持久化重建。
#[derive(Debug, Clone)]
pub struct BizMetadataAliasSnapshot {
    pub id: BizMetadataAliasId,
    pub metadata_id: BizMetadataId,
    pub alias: String,
    pub source: AliasSource,
    pub weight: i32,
    pub is_primary: bool,
    pub language: String,
    pub audit: Audit,
}

impl BizMetadataAlias {
    /// 创建新的元数据别名，使用默认来源、权重与语言。
    pub fn new(metadata_id: BizMetadataId, alias: impl Into<String>) -> Result<Self, DomainError> {
        let now = Utc::now();
        Self::from_snapshot(BizMetadataAliasSnapshot {
            id: BizMetadataAliasId::new(0),
            metadata_id,
            alias: alias.into(),
            source: AliasSource::Manual,
            weight: 0,
            is_primary: false,
            language: "zh-CN".to_string(),
            audit: Audit::new(now),
        })
    }

    /// 按照持久化快照重建聚合。
    pub fn from_snapshot(snapshot: BizMetadataAliasSnapshot) -> Result<Self, DomainError> {
        let BizMetadataAliasSnapshot {
            id,
            metadata_id,
            alias,
            source,
            weight,
            is_primary,
            language,
            audit,
        } = snapshot;

        let alias = AliasText::new(alias)?;
        let weight = AliasWeight::new(weight)?;
        let language = LanguageCode::new(language)?;

        Ok(Self {
            id,
            metadata_id,
            alias,
            source,
            weight,
            is_primary,
            language,
            audit,
        })
    }

    /// 返回别名标识。
    pub fn id(&self) -> BizMetadataAliasId {
        self.id
    }

    /// 所属元数据标识。
    pub fn metadata_id(&self) -> BizMetadataId {
        self.metadata_id
    }

    /// 别名值对象。
    pub fn alias(&self) -> &AliasText {
        &self.alias
    }

    /// 别名来源。
    pub fn source(&self) -> AliasSource {
        self.source
    }

    /// 匹配权重。
    pub fn weight(&self) -> AliasWeight {
        self.weight
    }

    /// 是否为首选别名。
    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    /// 语言编码。
    pub fn language(&self) -> &LanguageCode {
        &self.language
    }

    /// 创建时间。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.audit.created_at()
    }

    /// 最近更新时间。
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.audit.updated_at()
    }

    /// 软删除时间。
    pub fn delete_at(&self) -> Option<DateTime<Utc>> {
        self.audit.delete_at()
    }

    /// 返回审计信息，便于持久化与测试。
    pub fn audit(&self) -> &Audit {
        &self.audit
    }

    /// 调整关联的元数据 ID。
    pub fn change_metadata_id(&mut self, metadata_id: BizMetadataId) -> Result<(), DomainError> {
        self.metadata_id = metadata_id;
        self.bump_updated(Utc::now())
    }

    /// 更新别名文本。
    pub fn update_alias(&mut self, alias: AliasText) -> Result<(), DomainError> {
        alias.validate()?;
        self.alias = alias;
        self.bump_updated(Utc::now())
    }

    /// 调整别名来源。
    pub fn change_source(&mut self, source: AliasSource) -> Result<(), DomainError> {
        self.source = source;
        self.bump_updated(Utc::now())
    }

    /// 调整匹配权重。
    pub fn change_weight(&mut self, weight: i32) -> Result<(), DomainError> {
        self.weight = AliasWeight::new(weight)?;
        self.bump_updated(Utc::now())
    }

    /// 切换首选标记。
    pub fn set_primary(&mut self, is_primary: bool) -> Result<(), DomainError> {
        self.is_primary = is_primary;
        self.bump_updated(Utc::now())
    }

    /// 修改语言编码。
    pub fn change_language(&mut self, language: LanguageCode) -> Result<(), DomainError> {
        language.validate()?;
        self.language = language;
        self.bump_updated(Utc::now())
    }

    /// 软删除别名。
    pub fn mark_deleted(&mut self, delete_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.mark_deleted(delete_at)
    }

    fn bump_updated(&mut self, now: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.bump_updated(now)
    }
}

impl Entity for BizMetadataAlias {
    type Id = BizMetadataAliasId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl AggregateRoot for BizMetadataAlias {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_alias_defaults() {
        let alias = BizMetadataAlias::new(BizMetadataId::new(1), "营收").unwrap();
        assert_eq!(alias.source(), AliasSource::Manual);
        assert_eq!(alias.weight().value(), 0);
        assert_eq!(alias.language().as_str(), "zh-CN");
        assert!(!alias.is_primary());
    }

    #[test]
    fn rejects_blank_alias() {
        let result = BizMetadataAlias::new(BizMetadataId::new(1), "   ");
        assert!(result.is_err());
    }

    #[test]
    fn prevents_invalid_weight() {
        let result = BizMetadataAlias::from_snapshot(BizMetadataAliasSnapshot {
            id: BizMetadataAliasId::new(1),
            metadata_id: BizMetadataId::new(2),
            alias: "test".to_string(),
            source: AliasSource::Manual,
            weight: 200,
            is_primary: false,
            language: "zh-CN".to_string(),
            audit: Audit::new(Utc::now()),
        });
        assert!(result.is_err());
    }
}
