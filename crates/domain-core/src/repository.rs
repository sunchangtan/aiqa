use crate::aggregate_root::AggregateRoot;
use crate::domain_error::DomainError;
use crate::expression::{Expression, QueryOptions};
use crate::pagination::PageResult;
use std::future::Future;

/// 通用仓储接口，抽象聚合根的持久化读写与查询能力。
pub trait Repository<A>: Send + Sync
where
    A: AggregateRoot + Send + Sync,
    A::Id: Send + Sync,
{
    /// 插入操作返回的异步任务类型，允许实现自定义 Future。
    type InsertFuture<'a>: Future<Output = Result<(), DomainError>> + Send + 'a
    where
        Self: 'a,
        A: 'a;
    /// 更新操作返回的异步任务类型。
    type UpdateFuture<'a>: Future<Output = Result<(), DomainError>> + Send + 'a
    where
        Self: 'a,
        A: 'a;
    /// 删除操作返回的异步任务类型。
    type DeleteFuture<'a>: Future<Output = Result<(), DomainError>> + Send + 'a
    where
        Self: 'a,
        A: 'a;
    /// 根据 ID 查询单个聚合根时返回的异步任务类型。
    type FindByIdFuture<'a>: Future<Output = Result<Option<A>, DomainError>> + Send + 'a
    where
        Self: 'a,
        A: 'a;
    /// 复杂筛选分页查询所返回的异步任务类型。
    type QueryFuture<'a>: Future<Output = Result<PageResult<A>, DomainError>> + Send + 'a
    where
        Self: 'a,
        A: 'a;

    /// 新增一个聚合根实例，若违反领域约束或唯一性应返回 [`DomainError`].
    fn insert(&self, aggregate: A) -> Self::InsertFuture<'_>;

    /// 按照聚合根当前状态执行持久化更新。
    fn update(&self, aggregate: A) -> Self::UpdateFuture<'_>;

    /// 根据聚合根 ID 删除实体，可实现软删或硬删策略。
    fn delete(&self, id: A::Id) -> Self::DeleteFuture<'_>;

    /// 通过 ID 拉取聚合根，未命中返回 `Ok(None)`。
    fn find_by_id(&self, id: A::Id) -> Self::FindByIdFuture<'_>;

    /// 按表达式与分页参数查询多条聚合根，返回 [`PageResult`] 承载结果及分页信息。
    fn query(&self, expr: Expression, options: QueryOptions) -> Self::QueryFuture<'_>;
}
