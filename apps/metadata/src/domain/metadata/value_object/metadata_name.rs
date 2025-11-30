use domain_core::prelude::{DomainError, ValueObject};

use super::validate_non_empty;

/// 强类型的元数据展示名称。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetadataName(String);

impl MetadataName {
    /// 根据字符串创建名称，并校验非空。
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into();
        validate_non_empty(&name, "metadata name")?;
        Ok(Self(name))
    }

    /// 以 `&str` 形式读取名称。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费自身并返回底层 `String`。
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl ValueObject for MetadataName {
    fn validate(&self) -> Result<(), DomainError> {
        validate_non_empty(&self.0, "metadata name")
    }
}

impl From<MetadataName> for String {
    fn from(value: MetadataName) -> Self {
        value.0
    }
}
