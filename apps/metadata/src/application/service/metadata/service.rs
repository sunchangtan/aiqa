use domain_core::domain_error::DomainError;
use domain_core::pagination::PageResult;

use crate::application::service::metadata::command::{
    CreateMetadataCommand,
    ExtraUpdate,
    UpdateMetadataCommand,
};
use crate::application::service::metadata::query::MetadataQueryRequest;
use crate::domain::metadata::metadata::Metadata;
use crate::domain::metadata::repository::MetadataRepository;
use crate::domain::metadata::value_object::{MetadataId, MetadataName, ValueType};

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

    pub async fn create_metadata(
        &self,
        cmd: CreateMetadataCommand,
    ) -> Result<(), DomainError> {
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

    pub async fn update_metadata(
        &self,
        cmd: UpdateMetadataCommand,
    ) -> Result<(), DomainError> {
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
        self
            .repository
            .query_metadata(request.expression, request.options)
            .await
    }

    pub fn repository(&self) -> &R {
        &self.repository
    }
}
