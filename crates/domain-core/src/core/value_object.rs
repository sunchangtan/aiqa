//! 值对象 (Value Object) 抽象，强调不可共享的属性组合。

use crate::error::domain_error::DomainError;
use std::fmt::Debug;
use std::hash::Hash;

/// 所有值对象的共性：按值相等、通常不可变、无全局 ID。
pub trait ValueObject: Clone + Eq + Hash + Debug {
    /// 针对内部不变式的统一校验入口，默认实现为“无需校验”。
    fn validate(&self) -> Result<(), DomainError> {
        Ok(())
    }
}
