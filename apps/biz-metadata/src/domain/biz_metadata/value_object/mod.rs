//! 元数据领域中用到的所有值对象定义，集中处理字段校验与不可变约束。

mod biz_metadata_code;
mod biz_metadata_id;
mod biz_metadata_name;
mod biz_metadata_status;
mod biz_metadata_type;
mod data_class;
mod unit;
mod value_type;

pub use biz_metadata_code::BizMetadataCode;
pub use biz_metadata_id::BizMetadataId;
pub use biz_metadata_name::BizMetadataName;
pub use biz_metadata_status::BizMetadataStatus;
pub use biz_metadata_type::BizMetadataType;
pub use data_class::DataClass;
pub use unit::Unit;
pub use value_type::ValueType;

use domain_core::prelude::DomainError;

/// 校验字符串是否非空并包含非空白字符。
pub(crate) fn validate_non_empty(value: &str, label: &str) -> Result<(), DomainError> {
    if value.trim().is_empty() {
        Err(DomainError::Validation {
            message: format!("{label} cannot be blank"),
        })
    } else {
        Ok(())
    }
}
