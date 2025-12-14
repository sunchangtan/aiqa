use domain_core::prelude::{DomainError, ValueObject};

/// 版本号（乐观锁）。
///
/// - 必须为正整数
/// - 更新/删除时必须携带并匹配，成功后版本号递增
///
/// # 示例
/// ```
/// use biz_metadata::Version;
///
/// let v = Version::new(1).unwrap();
/// assert_eq!(v.value(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(i32);

impl Version {
    /// 创建版本号。
    pub fn new(value: i32) -> Result<Self, DomainError> {
        if value <= 0 {
            return Err(DomainError::Validation {
                message: "version must be a positive integer".into(),
            });
        }
        Ok(Self(value))
    }

    /// 返回版本号数值。
    pub fn value(&self) -> i32 {
        self.0
    }

    /// 返回下一版本号（+1）。
    pub fn next(&self) -> Result<Self, DomainError> {
        Self::new(self.0.saturating_add(1))
    }
}

impl ValueObject for Version {
    fn validate(&self) -> Result<(), DomainError> {
        if self.0 <= 0 {
            return Err(DomainError::Validation {
                message: "version must be a positive integer".into(),
            });
        }
        Ok(())
    }
}

impl From<Version> for i32 {
    fn from(value: Version) -> Self {
        value.0
    }
}
