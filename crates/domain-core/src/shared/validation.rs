//! 通用校验工具。
use crate::error::domain_error::DomainError;

/// 校验字符串是否包含非空白字符。
pub fn validate_non_empty(value: &str, label: &str) -> Result<(), DomainError> {
    if value.trim().is_empty() {
        Err(DomainError::Validation {
            message: format!("{label} cannot be blank"),
        })
    } else {
        Ok(())
    }
}
