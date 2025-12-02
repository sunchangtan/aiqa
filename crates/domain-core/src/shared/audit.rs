use chrono::{DateTime, Utc};

use crate::error::domain_error::DomainError;

/// 通用的审计时间信息，封装创建、更新时间以及软删除校验。
///
/// # 示例
/// ```
/// use chrono::{Duration, Utc};
/// use domain_core::audit::Audit;
///
/// let now = Utc::now();
/// let mut audit = Audit::new(now);
/// audit.bump_updated(now + Duration::seconds(5)).unwrap();
/// assert!(audit.mark_deleted(now + Duration::seconds(10)).is_ok());
/// assert!(audit.is_deleted());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audit {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    delete_at: Option<DateTime<Utc>>,
}

impl Audit {
    /// 以当前时间构造时间线。
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            created_at: now,
            updated_at: now,
            delete_at: None,
        }
    }

    /// 从持久化数据重建时间线并校验时间顺序。
    pub fn reconstruct(
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        delete_at: Option<DateTime<Utc>>,
    ) -> Result<Self, DomainError> {
        Self::validate(created_at, updated_at, delete_at)?;
        Ok(Self {
            created_at,
            updated_at,
            delete_at,
        })
    }

    /// 最近一次更新时间，UTC。
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// 创建时间，UTC。
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// 软删除时间，若未删除则为 `None`。
    pub fn delete_at(&self) -> Option<DateTime<Utc>> {
        self.delete_at
    }

    /// 是否已软删除。
    pub fn is_deleted(&self) -> bool {
        self.delete_at.is_some()
    }

    /// 更新 `updated_at`，要求不回退。
    pub fn bump_updated(&mut self, updated_at: DateTime<Utc>) -> Result<(), DomainError> {
        if updated_at < self.updated_at {
            return Err(DomainError::InvariantViolation {
                message: "updated_at cannot move backwards".into(),
            });
        }
        self.updated_at = updated_at;
        Ok(())
    }

    /// 标记删除，要求不早于当前 `updated_at`。
    pub fn mark_deleted(&mut self, delete_at: DateTime<Utc>) -> Result<(), DomainError> {
        if delete_at < self.updated_at {
            return Err(DomainError::InvariantViolation {
                message: "delete_at must be greater than or equal to updated_at".into(),
            });
        }
        self.delete_at = Some(delete_at);
        Ok(())
    }

    fn validate(
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        delete_at: Option<DateTime<Utc>>,
    ) -> Result<(), DomainError> {
        if updated_at < created_at {
            return Err(DomainError::InvariantViolation {
                message: "updated_at cannot be earlier than created_at".into(),
            });
        }
        if let Some(delete_at) = delete_at
            && delete_at < updated_at
        {
            return Err(DomainError::InvariantViolation {
                message: "delete_at must be greater than or equal to updated_at".into(),
            });
        }
        Ok(())
    }
}
