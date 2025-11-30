//! 元数据领域中用到的所有值对象定义，集中处理字段校验与不可变约束。

mod metadata_capabilities;
mod metadata_code;
mod metadata_id;
mod metadata_name;
mod metadata_type;
mod value_type;

pub use metadata_capabilities::MetadataCapabilities;
pub use metadata_code::MetadataCode;
pub use metadata_id::MetadataId;
pub use metadata_name::MetadataName;
pub use metadata_type::MetadataType;
pub use value_type::ValueType;

use domain_core::prelude::DomainError;

/// 校验字符串是否非空并包含非空白字符。
fn validate_non_empty(value: &str, label: &str) -> Result<(), DomainError> {
    if value.trim().is_empty() {
        Err(DomainError::Validation {
            message: format!("{label} cannot be blank"),
        })
    } else {
        Ok(())
    }
}
