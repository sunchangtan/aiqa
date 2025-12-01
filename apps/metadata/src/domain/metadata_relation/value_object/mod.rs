use domain_core::prelude::DomainError;

mod metadata_relation_id;

pub use metadata_relation_id::MetadataRelationId;

/// 校验关系 ID 是否有效地帮助函数。
///
/// # 示例
/// ```
/// # use domain_core::domain_error::DomainError;
/// # use metadata::MetadataRelationId;
/// # fn main() -> Result<(), DomainError> {
/// metadata::validate_relation_id(MetadataRelationId::new(1))?;
/// assert!(metadata::validate_relation_id(MetadataRelationId::new(-1)).is_err());
/// # Ok(()) }
/// ```
pub fn validate_relation_id(id: MetadataRelationId) -> Result<(), DomainError> {
    if id.value() <= 0 {
        return Err(DomainError::Validation {
            message: "metadata relation id must be positive".into(),
        });
    }
    Ok(())
}
