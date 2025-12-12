use domain_core::prelude::{DomainError, ValueObject, validate_non_empty};

/// 语言编码（如 zh-CN / en-US），最长 16 字符。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageCode(String);

impl LanguageCode {
    /// 创建语言编码并校验非空与长度。
    pub fn new(code: impl Into<String>) -> Result<Self, DomainError> {
        let code = code.into();
        validate_non_empty(&code, "biz_metadata_alias.language")?;
        if code.len() > 16 {
            return Err(DomainError::Validation {
                message: "language code length must be <= 16".into(),
            });
        }
        Ok(Self(code))
    }

    /// 以 `&str` 形式读取语言编码。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费自身返回底层字符串。
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl ValueObject for LanguageCode {
    fn validate(&self) -> Result<(), DomainError> {
        if self.0.len() > 16 {
            Err(DomainError::Validation {
                message: "language code length must be <= 16".into(),
            })
        } else {
            validate_non_empty(&self.0, "biz_metadata_alias.language")
        }
    }
}
