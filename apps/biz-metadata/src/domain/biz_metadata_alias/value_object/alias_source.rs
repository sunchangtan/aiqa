use domain_core::prelude::{DomainError, ValueObject};

/// 别名来源枚举，标记别名的生成方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AliasSource {
    /// 人工维护。
    Manual,
    /// 自动挖掘。
    AutoMine,
    /// 日志归纳。
    Log,
    /// 向量生成。
    Embedding,
}

impl AliasSource {
    /// 根据字符串创建来源枚举。
    pub fn new(raw: impl AsRef<str>) -> Result<Self, DomainError> {
        match raw.as_ref() {
            "manual" => Ok(Self::Manual),
            "auto_mine" => Ok(Self::AutoMine),
            "log" => Ok(Self::Log),
            "embedding" => Ok(Self::Embedding),
            other => Err(DomainError::Validation {
                message: format!("invalid alias source: {other}"),
            }),
        }
    }

    /// 返回枚举对应的字符串值。
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::AutoMine => "auto_mine",
            Self::Log => "log",
            Self::Embedding => "embedding",
        }
    }
}

impl ValueObject for AliasSource {}
