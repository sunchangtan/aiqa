//! 聚合根 (Aggregate Root) 抽象，约束聚合边界与一致性。

use crate::entity::Entity;

/// 聚合根必须是实体，并对整个聚合的一致性负责。
pub trait AggregateRoot: Entity {}
