use chrono::{DateTime, Utc};
use domain_core::prelude::{AggregateRoot, Audit, DomainError, Entity};

use crate::domain::metadata::value_object::MetadataId;
use crate::domain::metadata_relation::value_object::{MetadataRelationId, validate_relation_id};

/// 元数据之间的关系聚合根，记录起点与终点元数据的连接。
///
/// # 示例
/// 创建关系并重连到新的端点：
/// ```
/// # use domain_core::domain_error::DomainError;
/// use metadata::{MetadataRelation, MetadataRelationId, MetadataId};
///
/// # fn main() -> Result<(), DomainError> {
/// let mut relation = MetadataRelation::new(
///     MetadataRelationId::new(1),
///     MetadataId::new(10),
///     MetadataId::new(11),
/// )?;
/// relation.relink(MetadataId::new(12), MetadataId::new(13))?;
///
/// assert_eq!(relation.from_id().value(), 12);
/// assert_eq!(relation.to_id().value(), 13);
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataRelation {
    /// 关系标识。
    id: MetadataRelationId,
    /// 起点元数据 ID。
    from_metadata_id: MetadataId,
    /// 终点元数据 ID。
    to_metadata_id: MetadataId,
    /// 时间线审计信息。
    audit: Audit,
}

impl MetadataRelation {
    /// 创建新的元数据关系，要求起止元数据不同。
    pub fn new(
        id: impl Into<MetadataRelationId>,
        from_metadata_id: impl Into<MetadataId>,
        to_metadata_id: impl Into<MetadataId>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        let from_metadata_id = from_metadata_id.into();
        let to_metadata_id = to_metadata_id.into();
        validate_relation_id(id)?;
        Self::validate_endpoints(from_metadata_id, to_metadata_id)?;

        let now = Utc::now();
        let audit = Audit::new(now);
        Ok(Self {
            id,
            from_metadata_id,
            to_metadata_id,
            audit,
        })
    }

    /// 从持久化数据重建关系，允许指定时间戳。
    pub fn reconstruct(
        id: impl Into<MetadataRelationId>,
        from_metadata_id: impl Into<MetadataId>,
        to_metadata_id: impl Into<MetadataId>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        delete_at: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        let from_metadata_id = from_metadata_id.into();
        let to_metadata_id = to_metadata_id.into();
        let id = id.into();
        validate_relation_id(id)?;
        Self::validate_endpoints(from_metadata_id, to_metadata_id)?;
        let audit = Audit::reconstruct(created_at, updated_at, delete_at)?;

        Ok(Self {
            id,
            from_metadata_id,
            to_metadata_id,
            audit,
        })
    }

    /// 当前关系的唯一标识。
    pub fn id(&self) -> MetadataRelationId {
        self.id
    }

    /// 起点元数据标识。
    pub fn from_id(&self) -> MetadataId {
        self.from_metadata_id
    }

    /// 终点元数据标识。
    pub fn to_id(&self) -> MetadataId {
        self.to_metadata_id
    }

    /// 更新关系两端的元数据，保持不允许自指向。
    pub fn relink(
        &mut self,
        from_metadata_id: MetadataId,
        to_metadata_id: MetadataId,
    ) -> Result<(), DomainError> {
        Self::validate_endpoints(from_metadata_id, to_metadata_id)?;
        self.from_metadata_id = from_metadata_id;
        self.to_metadata_id = to_metadata_id;
        self.audit.bump_updated(Utc::now())?;
        Ok(())
    }

    /// 软删除关系，要求删除时间不早于 `updated_at`。
    pub fn mark_deleted(&mut self, delete_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.mark_deleted(delete_at)
    }

    /// 判断关系是否已经被软删除。
    pub fn is_deleted(&self) -> bool {
        self.audit.is_deleted()
    }

    /// 手动触碰更新时间，要求单调递增。
    pub fn touch(&mut self, updated_at: DateTime<Utc>) -> Result<(), DomainError> {
        self.audit.bump_updated(updated_at)
    }

    /// 创建时间（UTC）。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.audit.created_at()
    }

    /// 最近一次更新时间（UTC）。
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.audit.updated_at()
    }

    /// 软删除时间（UTC），未删除时为 `None`。
    pub fn delete_at(&self) -> Option<DateTime<Utc>> {
        self.audit.delete_at()
    }

    fn validate_endpoints(
        from_metadata_id: MetadataId,
        to_metadata_id: MetadataId,
    ) -> Result<(), DomainError> {
        if from_metadata_id == to_metadata_id {
            return Err(DomainError::Validation {
                message: "from_metadata_id and to_metadata_id cannot be the same".into(),
            });
        }
        Ok(())
    }
}

impl Entity for MetadataRelation {
    type Id = MetadataRelationId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl AggregateRoot for MetadataRelation {}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn constructs_relation_and_prevents_self_link() {
        let relation = MetadataRelation::new(
            MetadataRelationId::new(1),
            MetadataId::new(10),
            MetadataId::new(11),
        )
        .expect("valid relation");

        assert_eq!(relation.from_id(), MetadataId::new(10));
        assert_eq!(relation.to_id(), MetadataId::new(11));
        assert!(!relation.is_deleted());

        let err = MetadataRelation::new(
            MetadataRelationId::new(1),
            MetadataId::new(10),
            MetadataId::new(10),
        )
        .unwrap_err();
        matches!(err, DomainError::Validation { .. });
    }

    #[test]
    fn prevents_backwards_delete_timestamp() {
        let mut relation = MetadataRelation::new(
            MetadataRelationId::new(1),
            MetadataId::new(10),
            MetadataId::new(11),
        )
        .unwrap();

        let earlier = relation.updated_at() - Duration::seconds(1);
        assert!(relation.mark_deleted(earlier).is_err());
    }

    #[test]
    fn reconstruct_validates_timestamps() {
        let now = Utc::now();
        let err = MetadataRelation::reconstruct(
            MetadataRelationId::new(1),
            MetadataId::new(10),
            MetadataId::new(11),
            now,
            now - Duration::seconds(1),
            None,
        );
        assert!(err.is_err());
    }
}
