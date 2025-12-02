//! 聚合根 (Aggregate Root) 抽象，约束聚合边界与一致性。
//!
//! 聚合根是聚合的一致性边界，依赖实体基础能力。

use super::entity::Entity;

/// 聚合根必须是实体，并对整个聚合的一致性负责。
pub trait AggregateRoot: Entity {}
