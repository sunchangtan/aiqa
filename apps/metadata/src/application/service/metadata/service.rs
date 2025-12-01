use domain_core::domain_error::DomainError;
use domain_core::pagination::PageResult;

use crate::application::service::metadata::command::{
    CreateMetadataCommand, ExtraUpdate, UpdateMetadataCommand,
};
use crate::application::service::metadata::query::MetadataQueryRequest;
use crate::domain::metadata::Metadata;
use crate::domain::metadata::repository::MetadataRepository;
use crate::domain::metadata::value_object::{MetadataId, MetadataName, ValueType};

/// 元数据的应用服务，负责协调命令与查询。
///
/// # 示例
/// ```
/// use metadata::{
///     CreateMetadataCommand, ExtraUpdate, MetadataId, MetadataRepository, MetadataService,
///     MetadataType, MetadataQueryRequest,
/// };
/// use domain_core::expression::{Expression, QueryOptions};
/// use domain_core::pagination::PageResult;
/// use domain_core::domain_error::DomainError;
/// use std::future::{ready, Ready, Future};
/// use std::pin::Pin;
/// use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
///
/// struct InMemoryRepo;
/// impl MetadataRepository for InMemoryRepo {}
///
/// impl domain_core::repository::Repository<metadata::Metadata> for InMemoryRepo {
///     type InsertFuture<'a> = Ready<Result<(), DomainError>> where Self: 'a;
///     type UpdateFuture<'a> = Ready<Result<(), DomainError>> where Self: 'a;
///     type DeleteFuture<'a> = Ready<Result<(), DomainError>> where Self: 'a;
///     type FindByIdFuture<'a> = Ready<Result<Option<metadata::Metadata>, DomainError>> where Self: 'a;
///     type QueryFuture<'a> = Ready<Result<PageResult<metadata::Metadata>, DomainError>> where Self: 'a;
///
///     fn insert(&self, _aggregate: metadata::Metadata) -> Self::InsertFuture<'_> { ready(Ok(())) }
///     fn update(&self, _aggregate: metadata::Metadata) -> Self::UpdateFuture<'_> { ready(Ok(())) }
///     fn delete(&self, _id: MetadataId) -> Self::DeleteFuture<'_> { ready(Ok(())) }
///     fn find_by_id(&self, _id: MetadataId) -> Self::FindByIdFuture<'_> { ready(Ok(None)) }
///     fn query(&self, _expr: Expression, _options: QueryOptions) -> Self::QueryFuture<'_> {
///         ready(Ok(PageResult::empty(None, 0, None)))
///     }
/// }
///
/// async fn demo() -> Result<(), DomainError> {
///     let repo = InMemoryRepo;
///     let service = MetadataService::new(repo);
///     service.create_metadata(CreateMetadataCommand {
///         id: MetadataId::new(1),
///         code: "code".into(),
///         name: "name".into(),
///         metadata_type: MetadataType::Attribute,
///         value_type: "string".into(),
///         capabilities: None,
///         extra: None,
///     }).await?;
///
///     let _ = service.query_metadata(MetadataQueryRequest {
///         expression: Expression::True,
///         options: QueryOptions::default(),
///     }).await?;
///     Ok(())
/// }
///
/// fn block_on<F: Future>(mut fut: F) -> F::Output {
///     fn dummy_raw_waker() -> RawWaker {
///         fn no_op(_: *const ()) {}
///         fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
///         let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
///         RawWaker::new(std::ptr::null(), vtable)
///     }
///     let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
///     let mut cx = Context::from_waker(&waker);
///     // SAFETY: 文档示例中 Future 仅使用一次。
///     let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
///     match fut.as_mut().poll(&mut cx) {
///         Poll::Ready(val) => val,
///         Poll::Pending => panic!("demo future should complete immediately"),
///     }
/// }
///
/// # block_on(demo()).unwrap();
/// ```
pub struct MetadataService<R>
where
    R: MetadataRepository,
{
    repository: R,
}

impl<R> MetadataService<R>
where
    R: MetadataRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_metadata(&self, cmd: CreateMetadataCommand) -> Result<(), DomainError> {
        let mut metadata = Metadata::new(
            cmd.id,
            cmd.code,
            cmd.name,
            cmd.metadata_type,
            cmd.value_type,
        )?;

        if let Some(caps) = cmd.capabilities {
            metadata.set_capabilities(caps)?;
        }

        if let Some(extra) = cmd.extra {
            metadata.set_extra(Some(extra))?;
        }

        self.repository.insert_metadata(metadata).await
    }

    pub async fn update_metadata(&self, cmd: UpdateMetadataCommand) -> Result<(), DomainError> {
        let mut metadata = self
            .repository
            .find_metadata_by_id(cmd.id)
            .await?
            .ok_or_else(|| DomainError::Validation {
                message: format!("metadata {} not found", cmd.id.value()),
            })?;

        if let Some(name) = cmd.name {
            let name = MetadataName::new(name)?;
            metadata.rename(name)?;
        }

        if let Some(metadata_type) = cmd.metadata_type {
            metadata.change_metadata_type(metadata_type)?;
        }

        if let Some(value_type) = cmd.value_type {
            let value_type = ValueType::new(value_type)?;
            metadata.change_value_type(value_type)?;
        }

        if let Some(caps) = cmd.capabilities {
            metadata.set_capabilities(caps)?;
        }

        match cmd.extra {
            ExtraUpdate::Keep => {}
            ExtraUpdate::Set(value) => metadata.set_extra(Some(value))?,
            ExtraUpdate::Clear => metadata.set_extra(None)?,
        }

        self.repository.update_metadata(metadata).await
    }

    pub async fn delete_metadata(&self, id: MetadataId) -> Result<(), DomainError> {
        self.repository.delete_metadata(id).await
    }

    pub async fn query_metadata(
        &self,
        request: MetadataQueryRequest,
    ) -> Result<PageResult<Metadata>, DomainError> {
        self.repository
            .query_metadata(request.expression, request.options)
            .await
    }

    pub fn repository(&self) -> &R {
        &self.repository
    }
}
