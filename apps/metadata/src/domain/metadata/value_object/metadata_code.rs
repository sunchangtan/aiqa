use domain_core::prelude::{DomainError, ValueObject};

use super::validate_non_empty;

/// 强类型的元数据编码。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetadataCode(String);

impl MetadataCode {
    /// 根据字符串创建编码，并校验非空。
    pub fn new(code: impl Into<String>) -> Result<Self, DomainError> {
        let code = code.into();
        validate_non_empty(&code, "metadata code")?;
        Ok(Self(code))
    }

    /// 以 `&str` 形式读取编码。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费自身并返回底层 `String`。
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl ValueObject for MetadataCode {
    fn validate(&self) -> Result<(), DomainError> {
        validate_non_empty(&self.0, "metadata code")
    }
}

impl From<MetadataCode> for String {
    fn from(value: MetadataCode) -> Self {
        value.0
    }
}
