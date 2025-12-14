use domain_core::prelude::{DomainError, ValueObject};

/// 元数据的数据分类，描述值的语义类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataClass {
    /// 普通属性字段。
    Attribute,
    /// 指标/度量字段（可聚合，通常需要 unit）。
    Metric,
    /// 长文本（检索/RAG）。
    Text,
    /// 对象结构/字段组。
    Object,
    /// 列表/多值结构。
    Array,
    /// 标识符字段（替代旧 is_identifier）。
    Identifier,
}

impl DataClass {
    /// 从字符串创建数据分类，大小写不敏感。
    pub fn new(raw: impl AsRef<str>) -> Result<Self, DomainError> {
        Self::try_from(raw.as_ref())
    }

    /// 返回标准字符串表示。
    pub fn as_str(&self) -> &'static str {
        match self {
            DataClass::Attribute => "attribute",
            DataClass::Metric => "metric",
            DataClass::Text => "text",
            DataClass::Object => "object",
            DataClass::Array => "array",
            DataClass::Identifier => "identifier",
        }
    }
}

impl TryFrom<&str> for DataClass {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "attribute" => Ok(DataClass::Attribute),
            "metric" => Ok(DataClass::Metric),
            "text" => Ok(DataClass::Text),
            "object" => Ok(DataClass::Object),
            "array" => Ok(DataClass::Array),
            "identifier" => Ok(DataClass::Identifier),
            other => Err(DomainError::Validation {
                message: format!("invalid data_class: {other}"),
            }),
        }
    }
}

impl ValueObject for DataClass {
    fn validate(&self) -> Result<(), DomainError> {
        Ok(())
    }
}

impl From<DataClass> for String {
    fn from(value: DataClass) -> Self {
        value.as_str().to_string()
    }
}
