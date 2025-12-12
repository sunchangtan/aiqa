use domain_core::prelude::{DomainError, ValueObject};

/// 匹配权重，取值范围 0~100，数值越高优先级越高。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AliasWeight(i32);

impl AliasWeight {
    /// 创建权重并校验范围。
    pub fn new(weight: i32) -> Result<Self, DomainError> {
        if (0..=100).contains(&weight) {
            Ok(Self(weight))
        } else {
            Err(DomainError::Validation {
                message: format!("weight must be between 0 and 100, got {weight}"),
            })
        }
    }

    /// 以原始 `i32` 形式返回权重值。
    pub const fn value(self) -> i32 {
        self.0
    }
}

impl ValueObject for AliasWeight {}
