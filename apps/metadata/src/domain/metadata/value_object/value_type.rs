use domain_core::prelude::{DomainError, ValueObject};

use super::validate_non_empty;

/// 单个允许的值类型（如 "int"、"decimal"、"string"）。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueType(String);

impl ValueType {
    /// 创建新的值类型并校验非空。
    pub fn new(value_type: impl Into<String>) -> Result<Self, DomainError> {
        let value_type = value_type.into();
        validate_non_empty(&value_type, "value type")?;
        Ok(Self(value_type))
    }

    /// 以 `&str` 读取值类型。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费自身并返回底层 `String`。
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl ValueObject for ValueType {
    fn validate(&self) -> Result<(), DomainError> {
        validate_non_empty(&self.0, "value type")
    }
}

impl From<ValueType> for String {
    fn from(value: ValueType) -> Self {
        value.0
    }
}
