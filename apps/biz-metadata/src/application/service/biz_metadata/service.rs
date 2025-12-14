use domain_core::domain_error::DomainError;
use domain_core::pagination::PageResult;

use crate::application::service::biz_metadata::command::{
    CreateBizMetadataCommand, FieldUpdate, UpdateBizMetadataCommand,
};
use crate::application::service::biz_metadata::query::BizMetadataQueryRequest;
use crate::domain::biz_metadata::BizMetadata;
use crate::domain::biz_metadata::repository::BizMetadataRepository;
use crate::domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataName, ObjectType, TenantId, Unit, ValueType, Version,
};
use chrono::Utc;

/// 元数据的应用服务，负责协调命令与查询。
///
/// ```
/// use biz_metadata::{BizMetadataService, CreateBizMetadataCommand, DataClass, ObjectType};
/// use domain_core::domain_error::DomainError;
/// use domain_core::expression::{Expression, QueryOptions};
/// use domain_core::pagination::PageResult;
/// use std::future::{ready, Ready};
///
/// struct InMemoryRepo;
/// impl biz_metadata::BizMetadataRepository for InMemoryRepo {}
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
///     fn delete(&self, _id: biz_metadata::BizMetadataId) -> Self::DeleteFuture<'_> { ready(Ok(())) }
///     fn find_by_id(&self, _id: biz_metadata::BizMetadataId) -> Self::FindByIdFuture<'_> { ready(Ok(None)) }
///     fn query(&self, _expr: Expression, _options: QueryOptions) -> Self::QueryFuture<'_> {
///         ready(Ok(PageResult::empty(None, 0, None)))
///     }
/// }
///
/// let repo = InMemoryRepo;
/// let service = BizMetadataService::new(repo);
/// let rt = tokio::runtime::Runtime::new().unwrap();
/// rt.block_on(async {
///     let created = service.create_biz_metadata(CreateBizMetadataCommand {
///         code: "company.base.name_cn".into(),
///         name: "公司中文名".into(),
///         description: None,
///         object_type: ObjectType::Feature,
///         parent_id: None,
///         data_class: Some(DataClass::Attribute),
///         value_type: Some("string".into()),
///         unit: None,
///         status: None,
///         source: None,
///     }).await?;
///     assert_eq!(created.object_type().as_str(), "feature");
///     Ok::<(), DomainError>(())
/// }).unwrap();
/// ```
pub struct BizMetadataService<R>
where
    R: BizMetadataRepository,
{
    repository: R,
}

const DEFAULT_TENANT_ID: &str = "default";

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
        let tenant_id = TenantId::new(DEFAULT_TENANT_ID)?;
        let object_type = cmd.object_type;
        let mut biz_metadata = match object_type {
            ObjectType::Feature => {
                let data_class = cmd.data_class.ok_or(DomainError::Validation {
                    message: "object_type=feature requires data_class".into(),
                })?;
                let value_type = cmd.value_type.ok_or(DomainError::Validation {
                    message: "object_type=feature requires value_type".into(),
                })?;
                BizMetadata::new_feature(
                    tenant_id,
                    cmd.code,
                    cmd.name,
                    data_class,
                    ValueType::new(value_type)?,
                )?
            }
            _ => BizMetadata::new_node(tenant_id, cmd.code, cmd.name, object_type)?,
        };
        biz_metadata.set_description(cmd.description)?;
        biz_metadata.set_parent_id(cmd.parent_id)?;
        if object_type == ObjectType::Feature {
            let unit = cmd.unit.map(Unit::new).transpose()?;
            biz_metadata.set_unit(unit)?;
        }
        if let Some(status) = cmd.status {
            biz_metadata.change_status(status)?;
        }
        if let Some(source) = cmd.source {
            biz_metadata.change_source(source)?;
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

        if biz_metadata.version() != cmd.version {
            return Err(DomainError::Validation {
                message: "version not match".into(),
            });
        }

        if let Some(name) = cmd.name {
            let name = BizMetadataName::new(name)?;
            biz_metadata.rename(name)?;
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

        match cmd.parent_id {
            FieldUpdate::Keep => {}
            FieldUpdate::Set(parent_id) => biz_metadata.set_parent_id(Some(parent_id))?,
            FieldUpdate::Clear => biz_metadata.set_parent_id(None)?,
        }

        if let Some(status) = cmd.status {
            biz_metadata.change_status(status)?;
        }

        if let Some(source) = cmd.source {
            biz_metadata.change_source(source)?;
        }

        self.repository.update_biz_metadata(biz_metadata).await
    }

    pub async fn delete_biz_metadata(
        &self,
        id: BizMetadataId,
        version: Version,
    ) -> Result<(), DomainError> {
        let mut biz_metadata = self
            .repository
            .find_biz_metadata_by_id(id)
            .await?
            .ok_or_else(|| DomainError::Validation {
                message: format!("biz_metadata {} not found", id.value()),
            })?;

        if biz_metadata.version() != version {
            return Err(DomainError::Validation {
                message: "version not match".into(),
            });
        }

        biz_metadata.mark_deleted(Utc::now())?;
        let _ = self.repository.update_biz_metadata(biz_metadata).await?;
        Ok(())
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
