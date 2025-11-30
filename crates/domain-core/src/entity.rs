//! 实体 (Entity) 抽象，聚焦于“同一个身份”判断。

use std::fmt::Debug;
use std::hash::Hash;

/// 领域中所有“有身份”的业务对象都应该实现的基类 trait。
pub trait Entity {
    /// 实体 ID 的类型，例如 `UserId`、`OrderId`。
    type Id: Copy + Eq + Hash + Debug;

    /// 返回实体的唯一标识。
    fn id(&self) -> Self::Id;

    /// 基于 ID 判断是否为同一个实体，常用于领域服务去重或缓存命中。
    fn same_identity(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
