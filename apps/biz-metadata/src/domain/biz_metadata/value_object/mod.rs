//! 元数据领域中用到的所有值对象定义，集中处理字段校验与不可变约束。

mod biz_metadata_code;
mod biz_metadata_id;
mod biz_metadata_name;
mod biz_metadata_status;
mod data_class;
mod object_type;
mod source;
mod tenant_id;
mod unit;
mod value_type;
mod version;

pub use biz_metadata_code::BizMetadataCode;
pub use biz_metadata_id::BizMetadataId;
pub use biz_metadata_name::BizMetadataName;
pub use biz_metadata_status::BizMetadataStatus;
pub use data_class::DataClass;
pub use object_type::ObjectType;
pub use source::Source;
pub use tenant_id::TenantId;
pub use unit::Unit;
pub use value_type::ValueType;
pub use version::Version;
