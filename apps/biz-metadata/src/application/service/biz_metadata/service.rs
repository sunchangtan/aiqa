use domain_core::domain_error::DomainError;
use domain_core::pagination::PageResult;

use crate::application::service::biz_metadata::command::{
    CreateBizMetadataCommand, FieldUpdate, UpdateBizMetadataCommand,
};
use crate::application::service::biz_metadata::query::BizMetadataQueryRequest;
use crate::domain::biz_metadata::BizMetadata;
use crate::domain::biz_metadata::repository::BizMetadataRepository;
use crate::domain::biz_metadata::value_object::{BizMetadataId, BizMetadataName, Unit, ValueType};

/// 元数据的应用服务，负责协调命令与查询。
///
/// # 示例
/// ```
/// use biz_metadata::{
///     CreateBizMetadataCommand, FieldUpdate, BizMetadataId, BizMetadataRepository, BizMetadataService,
///     BizMetadataType, BizMetadataQueryRequest, DataClass,
/// };
/// use domain_core::expression::{Expression, QueryOptions};
/// use domain_core::pagination::PageResult;
/// use domain_core::domain_error::DomainError;
/// use std::future::{ready, Ready, Future};
/// use std::pin::Pin;
/// use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
///
/// struct InMemoryRepo;
/// impl BizMetadataRepository for InMemoryRepo {}
///
/// impl domain_core::repository::Repository<biz_metadata::BizMetadata> for InMemoryRepo {
///     type InsertFuture<'a> = Ready<Result<biz_metadata::BizMetadata, DomainError>> where Self: 'a;
///     type UpdateFuture<'a> = Ready<Result<biz_metadata::BizMetadata, DomainError>> where Self: 'a;
///     type DeleteFuture<'a> = Ready<Result<(), DomainError>> where Self: 'a;
///     type FindByIdFuture<'a> = Ready<Result<Option<biz_metadata::BizMetadata>, DomainError>> where Self: 'a;
///     type QueryFuture<'a> = Ready<Result<PageResult<biz_metadata::BizMetadata>, DomainError>> where Self: 'a;
///
///     fn insert(&self, aggregate: biz_metadata::BizMetadata) -> Self::InsertFuture<'_> { ready(Ok(aggregate)) }
///     fn update(&self, aggregate: biz_metadata::BizMetadata) -> Self::UpdateFuture<'_> { ready(Ok(aggregate)) }
///     fn delete(&self, _id: BizMetadataId) -> Self::DeleteFuture<'_> { ready(Ok(())) }
///     fn find_by_id(&self, _id: BizMetadataId) -> Self::FindByIdFuture<'_> { ready(Ok(None)) }
///     fn query(&self, _expr: Expression, _options: QueryOptions) -> Self::QueryFuture<'_> {
///         ready(Ok(PageResult::empty(None, 0, None)))
///     }
/// }
///
/// async fn demo() -> Result<(), DomainError> {
///     let repo = InMemoryRepo;
///     let service = BizMetadataService::new(repo);
///     let created = service.create_biz_metadata(CreateBizMetadataCommand {
///         code: "code".into(),
///         name: "name".into(),
///         description: None,
///         metadata_type: BizMetadataType::Attribute,
///         data_class: DataClass::Dimension,
///         value_type: "string".into(),
///         owner_id: None,
///         unit: None,
///         is_identifier: false,
///         status: None,
///     }).await?;
///     assert_eq!(created.code().as_str(), "code");
///
///     let _ = service.query_biz_metadata(BizMetadataQueryRequest {
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
pub struct BizMetadataService<R>
where
    R: BizMetadataRepository,
{
    repository: R,
}

impl<R> BizMetadataService<R>
where
    R: BizMetadataRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_biz_metadata(
        &self,
        cmd: CreateBizMetadataCommand,
    ) -> Result<BizMetadata, DomainError> {
        let mut biz_metadata = BizMetadata::new(
            cmd.code,
            cmd.name,
            cmd.metadata_type,
            cmd.data_class,
            cmd.value_type,
        )?;

        biz_metadata.set_description(cmd.description)?;
        biz_metadata.set_owner_id(cmd.owner_id)?;
        let unit = cmd.unit.map(Unit::new).transpose()?;
        biz_metadata.set_unit(unit)?;
        biz_metadata.set_identifier(cmd.is_identifier)?;
        if let Some(status) = cmd.status {
            biz_metadata.change_status(status)?;
        }

        self.repository.insert_biz_metadata(biz_metadata).await
    }

    pub async fn update_biz_metadata(
        &self,
        cmd: UpdateBizMetadataCommand,
    ) -> Result<BizMetadata, DomainError> {
        let mut biz_metadata = self
            .repository
            .find_biz_metadata_by_id(cmd.id)
            .await?
            .ok_or_else(|| DomainError::Validation {
                message: format!("biz_metadata {} not found", cmd.id.value()),
            })?;

        if let Some(name) = cmd.name {
            let name = BizMetadataName::new(name)?;
            biz_metadata.rename(name)?;
        }

        if let Some(metadata_type) = cmd.metadata_type {
            biz_metadata.change_metadata_type(metadata_type)?;
        }

        match cmd.description {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(desc) => biz_metadata.set_description(Some(desc))?,
            FieldUpdate::Clear => biz_metadata.set_description(None)?,
        }

        if let Some(data_class) = cmd.data_class {
            biz_metadata.change_data_class(data_class)?;
        }

        if let Some(value_type) = cmd.value_type {
            let value_type = ValueType::new(value_type)?;
            biz_metadata.change_value_type(value_type)?;
        }

        match cmd.unit {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(value) => biz_metadata.set_unit(Some(Unit::new(value)?))?,
            FieldUpdate::Clear => biz_metadata.set_unit(None)?,
        }

        match cmd.owner_id {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(owner) => biz_metadata.set_owner_id(Some(owner))?,
            FieldUpdate::Clear => biz_metadata.set_owner_id(None)?,
        }

        if let Some(is_identifier) = cmd.is_identifier {
            biz_metadata.set_identifier(is_identifier)?;
        }

        if let Some(status) = cmd.status {
            biz_metadata.change_status(status)?;
        }

        self.repository.update_biz_metadata(biz_metadata).await
    }

    pub async fn delete_biz_metadata(&self, id: BizMetadataId) -> Result<(), DomainError> {
        self.repository.delete_biz_metadata(id).await
    }

    pub async fn query_biz_metadata(
        &self,
        request: BizMetadataQueryRequest,
    ) -> Result<PageResult<BizMetadata>, DomainError> {
        self.repository
            .query_biz_metadata(request.expression, request.options)
            .await
    }

    pub fn repository(&self) -> &R {
        &self.repository
    }

    /// 便捷查询：按 ID 查找。
    pub async fn find_biz_metadata_by_id(
        &self,
        id: BizMetadataId,
    ) -> Result<Option<BizMetadata>, DomainError> {
        self.repository.find_biz_metadata_by_id(id).await
    }
}
