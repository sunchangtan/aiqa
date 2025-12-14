use domain_core::prelude::{DomainError, ValueObject};

/// 元数据生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BizMetadataStatus {
    /// 正常生效。
    Active,
    /// 已弃用。
    Deprecated,
}

impl BizMetadataStatus {
    /// 从字符串创建状态值，大小写不敏感。
    pub fn new(raw: impl AsRef<str>) -> Result<Self, DomainError> {
        Self::try_from(raw.as_ref())
    }

    /// 返回标准字符串表示。
    pub fn as_str(&self) -> &'static str {
        match self {
            BizMetadataStatus::Active => "active",
            BizMetadataStatus::Deprecated => "deprecated",
        }
    }
}

impl TryFrom<&str> for BizMetadataStatus {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "active" => Ok(BizMetadataStatus::Active),
            "deprecated" => Ok(BizMetadataStatus::Deprecated),
            other => Err(DomainError::Validation {
                message: format!("invalid status: {other}"),
            }),
        }
    }
}

impl ValueObject for BizMetadataStatus {
    fn validate(&self) -> Result<(), DomainError> {
        Ok(())
    }
}

impl From<BizMetadataStatus> for String {
    fn from(value: BizMetadataStatus) -> Self {
        value.as_str().to_string()
    }
}
