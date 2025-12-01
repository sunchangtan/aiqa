use domain_core::domain_error::DomainError;
use domain_core::pagination::PageResult;

use crate::application::service::metadata_relation::command::{
    CreateMetadataRelationCommand, RelinkMetadataRelationCommand,
};
use crate::application::service::metadata_relation::query::MetadataRelationQueryRequest;
use crate::domain::metadata_relation::MetadataRelation;
use crate::domain::metadata_relation::repository::MetadataRelationRepository;
use crate::domain::metadata_relation::value_object::MetadataRelationId;

/// 元数据关系的应用服务，协调命令执行与查询。
///
/// # 示例
/// ```rust
/// use metadata::{
///     CreateMetadataRelationCommand, MetadataId, MetadataRelation,
///     MetadataRelationId, MetadataRelationQueryRequest,
///     MetadataRelationService, RelinkMetadataRelationCommand,
///     MetadataRelationRepository,
/// };
/// use domain_core::expression::{Expression, QueryOptions};
/// use domain_core::pagination::PageResult;
/// use domain_core::domain_error::DomainError;
/// use std::future::{ready, Ready, Future};
/// use std::pin::Pin;
/// use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
///
/// // 一个最简内存仓储实现，只用于文档示例。
/// struct InMemoryRepo;
///
/// impl MetadataRelationRepository for InMemoryRepo {}
///
/// impl domain_core::repository::Repository<MetadataRelation> for InMemoryRepo {
///     type InsertFuture<'a> = Ready<Result<(), DomainError>> where Self: 'a;
///     type UpdateFuture<'a> = Ready<Result<(), DomainError>> where Self: 'a;
///     type DeleteFuture<'a> = Ready<Result<(), DomainError>> where Self: 'a;
///     type FindByIdFuture<'a> = Ready<Result<Option<MetadataRelation>, DomainError>> where Self: 'a;
///     type QueryFuture<'a> = Ready<Result<PageResult<MetadataRelation>, DomainError>> where Self: 'a;
///
///     fn insert(&self, _aggregate: MetadataRelation) -> Self::InsertFuture<'_> {
///         ready(Ok(()))
///     }
///
///     fn update(&self, _aggregate: MetadataRelation) -> Self::UpdateFuture<'_> {
///         ready(Ok(()))
///     }
///
///     fn delete(&self, _id: MetadataRelationId) -> Self::DeleteFuture<'_> {
///         ready(Ok(()))
///     }
///
///     fn find_by_id(&self, _id: MetadataRelationId) -> Self::FindByIdFuture<'_> {
///         ready(Ok(None))
///     }
///
///     fn query(&self, _expr: Expression, _options: QueryOptions) -> Self::QueryFuture<'_> {
///         ready(Ok(PageResult::empty(None, 0, None)))
///     }
/// }
///
/// async fn demo() -> Result<(), DomainError> {
///     let repo = InMemoryRepo;
///     let service = MetadataRelationService::new(repo);
///
///     service
///         .create_relation(CreateMetadataRelationCommand {
///             id: MetadataRelationId::new(1),
///             from_metadata_id: MetadataId::new(10),
///             to_metadata_id: MetadataId::new(11),
///         })
///         .await?;
///
///     let _ = service
///         .query_relations(MetadataRelationQueryRequest::new(
///             Expression::True,
///             QueryOptions::default(),
///         ))
///         .await?;
///
///     Ok(())
/// }
///
/// // 由于示例仓储返回立即完成的 Future，这里用最小阻塞器执行它。
/// fn block_on<F: Future>(mut fut: F) -> F::Output {
///     fn dummy_raw_waker() -> RawWaker {
///         fn no_op(_: *const ()) {}
///         fn clone(_: *const ()) -> RawWaker {
///             dummy_raw_waker()
///         }
///         let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
///         RawWaker::new(std::ptr::null(), vtable)
///     }
///
///     let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
///     let mut cx = Context::from_waker(&waker);
///     // SAFETY: 我们不会在此示例中对同一 Future 重复使用 Pin。
///     let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
///     match fut.as_mut().poll(&mut cx) {
///         Poll::Ready(val) => val,
///         Poll::Pending => panic!("demo future should complete immediately"),
///     }
/// }
///
/// # block_on(demo()).unwrap();
/// ```
pub struct MetadataRelationService<R>
where
    R: MetadataRelationRepository,
{
    repository: R,
}

impl<R> MetadataRelationService<R>
where
    R: MetadataRelationRepository,
{
    /// 创建服务实例。
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// 创建元数据关系，会执行端点校验。
    pub async fn create_relation(
        &self,
        cmd: CreateMetadataRelationCommand,
    ) -> Result<(), DomainError> {
        let relation = MetadataRelation::new(cmd.id, cmd.from_metadata_id, cmd.to_metadata_id)?;
        self.repository.insert_relation(relation).await
    }

    /// 重连元数据关系，未找到关系时返回 `DomainError::Validation`。
    pub async fn relink_relation(
        &self,
        cmd: RelinkMetadataRelationCommand,
    ) -> Result<(), DomainError> {
        let mut relation = self
            .repository
            .find_relation_by_id(cmd.id)
            .await?
            .ok_or_else(|| DomainError::Validation {
                message: format!("metadata_relation {} not found", cmd.id.value()),
            })?;

        relation.relink(cmd.from_metadata_id, cmd.to_metadata_id)?;
        self.repository.update_relation(relation).await
    }

    /// 删除元数据关系。
    pub async fn delete_relation(&self, id: MetadataRelationId) -> Result<(), DomainError> {
        self.repository.delete_relation(id).await
    }

    /// 查询元数据关系。
    pub async fn query_relations(
        &self,
        request: MetadataRelationQueryRequest,
    ) -> Result<PageResult<MetadataRelation>, DomainError> {
        self.repository
            .query_relations(request.expression, request.options)
            .await
    }

    /// 访问底层仓储，便于组合使用。
    pub fn repository(&self) -> &R {
        &self.repository
    }
}
