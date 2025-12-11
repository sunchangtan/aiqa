use domain_core::prelude::{DomainError, ValueObject};

/// 元数据的数据分类，描述值的语义类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataClass {
    /// 数值型，可聚合。
    Metric,
    /// 枚举/离散/标识/日期，用于筛选或分组。
    Dimension,
    /// 文本描述。
    Text,
    /// 结构化分组/容器。
    Group,
}

impl DataClass {
    /// 从字符串创建数据分类，大小写不敏感。
    pub fn new(raw: impl AsRef<str>) -> Result<Self, DomainError> {
        Self::try_from(raw.as_ref())
    }

    /// 返回标准字符串表示。
    pub fn as_str(&self) -> &'static str {
        match self {
            DataClass::Metric => "metric",
            DataClass::Dimension => "dimension",
            DataClass::Text => "text",
            DataClass::Group => "group",
        }
    }
}

impl TryFrom<&str> for DataClass {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "metric" => Ok(DataClass::Metric),
            "dimension" => Ok(DataClass::Dimension),
            "text" => Ok(DataClass::Text),
            "group" => Ok(DataClass::Group),
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
