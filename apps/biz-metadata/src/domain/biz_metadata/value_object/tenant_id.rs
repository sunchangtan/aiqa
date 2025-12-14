use domain_core::prelude::{DomainError, ValueObject, validate_non_empty};

/// 多租户隔离标识。
///
/// 当前阶段 HTTP 层固定使用 `default`，但领域模型仍保留该字段，以便后续扩展为真正的多租户。
///
/// # 示例
/// ```
/// use biz_metadata::TenantId;
///
/// let tenant = TenantId::new("default").unwrap();
/// assert_eq!(tenant.as_str(), "default");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TenantId(String);

impl TenantId {
    /// 构造租户 ID 并校验非空。
    pub fn new(raw: impl Into<String>) -> Result<Self, DomainError> {
        let raw = raw.into();
        validate_non_empty(&raw, "tenant_id")?;
        Ok(Self(raw))
    }

    /// 返回字符串引用。
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 消费自身并返回底层 `String`。
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl ValueObject for TenantId {
    fn validate(&self) -> Result<(), DomainError> {
        validate_non_empty(&self.0, "tenant_id")
    }
}

impl From<TenantId> for String {
    fn from(value: TenantId) -> Self {
        value.0
    }
}
