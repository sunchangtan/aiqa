use domain_core::prelude::{DomainError, ValueObject};

/// 语义字典节点的对象类型（五类核心对象）。
///
/// 对齐规范：
/// - `docs/金融语义字典（biz_metadata）模型与强门禁校验规范_v1.0.md`（五类核心对象）
///
/// # 示例
/// ```
/// use biz_metadata::ObjectType;
///
/// let ty = ObjectType::new("feature").unwrap();
/// assert_eq!(ty.as_str(), "feature");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectType {
    /// 业务实体（如 company、person）。
    Entity,
    /// 业务事件（如 event.trade）。
    Event,
    /// 关系对象（如 relation.company_person）。
    Relation,
    /// 文档对象（如 document.report）。
    Document,
    /// 字段/特征定义（仅该类型允许填写 data_class/value_type/unit）。
    Feature,
}

impl ObjectType {
    /// 从字符串创建对象类型，大小写不敏感。
    pub fn new(raw: impl AsRef<str>) -> Result<Self, DomainError> {
        Self::try_from(raw.as_ref())
    }

    /// 返回规范字符串表示。
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectType::Entity => "entity",
            ObjectType::Event => "event",
            ObjectType::Relation => "relation",
            ObjectType::Document => "document",
            ObjectType::Feature => "feature",
        }
    }
}

impl TryFrom<&str> for ObjectType {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "entity" => Ok(ObjectType::Entity),
            "event" => Ok(ObjectType::Event),
            "relation" => Ok(ObjectType::Relation),
            "document" => Ok(ObjectType::Document),
            "feature" => Ok(ObjectType::Feature),
            other => Err(DomainError::Validation {
                message: format!("invalid object_type: {other}"),
            }),
        }
    }
}

impl ValueObject for ObjectType {
    fn validate(&self) -> Result<(), DomainError> {
        Ok(())
    }
}

impl From<ObjectType> for String {
    fn from(value: ObjectType) -> Self {
        value.as_str().to_string()
    }
}
