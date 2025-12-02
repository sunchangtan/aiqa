pub mod core;
pub mod error;
pub mod shared;

// 兼容旧路径，仍提供顶层模块入口。
pub mod aggregate_root {
    pub use crate::core::aggregate_root::*;
}

pub mod entity {
    pub use crate::core::entity::*;
}

pub mod repository {
    pub use crate::core::repository::*;
}

pub mod value_object {
    pub use crate::core::value_object::*;
}

pub mod domain_error {
    pub use crate::error::domain_error::*;
}

pub mod audit {
    pub use crate::shared::audit::*;
}

pub mod expression {
    pub use crate::shared::expression::*;
}

pub mod pagination {
    pub use crate::shared::pagination::*;
}

// 兼容别名
pub mod building_blocks {
    pub use crate::core::*;
}

pub mod support {
    pub use crate::shared::*;
}

pub mod prelude;
