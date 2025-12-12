pub mod biz_metadata;

// 暴露默认路由构建入口，便于上层使用。
pub use biz_metadata::{BIZ_METADATA_BASE, build_router};
