//! 领域层统一错误定义，承载业务规则与不变式失败信息。

use thiserror::Error;

/// 领域层统一错误类型：表达业务规则、不变式等失败
#[derive(Debug, Error)]
pub enum DomainError {
    /// 通用校验错误（值对象、实体构造失败）
    #[error("validation error: {message}")]
    Validation { message: String },

    /// 不变式 / 状态约束被违反
    #[error("invariant violation: {message}")]
    InvariantViolation { message: String },
}
