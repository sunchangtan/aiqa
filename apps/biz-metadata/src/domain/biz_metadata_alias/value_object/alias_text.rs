use domain_core::prelude::{DomainError, ValueObject, validate_non_empty};

/// 自然语言别名值对象，要求非空且包含可见字符。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AliasText(String);

impl AliasText {
    /// 根据输入字符串创建别名并校验。
    pub fn new(alias: impl Into<String>) -> Result<Self, DomainError> {
        let alias = alias.into();
        validate_non_empty(&alias, "biz_metadata_alias.alias")?;
        Ok(Self(alias))
    }

    /// 以 `&str` 形式返回别名。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费自身返回底层字符串。
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl ValueObject for AliasText {
    fn validate(&self) -> Result<(), DomainError> {
        validate_non_empty(&self.0, "biz_metadata_alias.alias")
    }
}
