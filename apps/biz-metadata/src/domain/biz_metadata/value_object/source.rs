use domain_core::prelude::{DomainError, ValueObject};

/// 元数据来源。
///
/// 对齐规范：
/// - `docs/金融语义字典（biz_metadata）模型与强门禁校验规范_v1.0.md`（source）
///
/// # 示例
/// ```
/// use biz_metadata::Source;
///
/// let source = Source::new("manual").unwrap();
/// assert_eq!(source.as_str(), "manual");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    /// 人工维护。
    Manual,
    /// 自动挖掘生成。
    AutoMine,
    /// 外部系统/API 同步。
    ApiSync,
}

impl Source {
    /// 从字符串创建来源，大小写不敏感。
    pub fn new(raw: impl AsRef<str>) -> Result<Self, DomainError> {
        Self::try_from(raw.as_ref())
    }

    /// 返回规范字符串表示。
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::Manual => "manual",
            Source::AutoMine => "auto_mine",
            Source::ApiSync => "api_sync",
        }
    }
}

impl TryFrom<&str> for Source {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "manual" => Ok(Source::Manual),
            "auto_mine" => Ok(Source::AutoMine),
            "api_sync" => Ok(Source::ApiSync),
            other => Err(DomainError::Validation {
                message: format!("invalid source: {other}"),
            }),
        }
    }
}

impl ValueObject for Source {
    fn validate(&self) -> Result<(), DomainError> {
        Ok(())
    }
}

impl From<Source> for String {
    fn from(value: Source) -> Self {
        value.as_str().to_string()
    }
}
