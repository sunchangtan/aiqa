use domain_core::prelude::{DomainError, ValueObject, validate_non_empty};

/// 计量单位值对象，用于限制空白字符串。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unit(String);

impl Unit {
    /// 创建新的单位描述。
    pub fn new(unit: impl Into<String>) -> Result<Self, DomainError> {
        let unit = unit.into();
        validate_non_empty(&unit, "unit")?;
        Ok(Self(unit))
    }

    /// 以 `&str` 形式读取单位。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费自身并返回内部字符串。
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl ValueObject for Unit {
    fn validate(&self) -> Result<(), DomainError> {
        validate_non_empty(&self.0, "unit")
    }
}

impl From<Unit> for String {
    fn from(value: Unit) -> Self {
        value.0
    }
}
