use domain_core::domain_error::DomainError;
use domain_core::pagination::PageResult;

use crate::application::service::biz_metadata_alias::command::{
    AliasFieldUpdate, CreateBizMetadataAliasCommand, UpdateBizMetadataAliasCommand,
};
use crate::application::service::biz_metadata_alias::query::BizMetadataAliasQueryRequest;
use crate::domain::biz_metadata_alias::value_object::{AliasText, BizMetadataAliasId};
use crate::domain::biz_metadata_alias::{BizMetadataAlias, BizMetadataAliasRepository};

/// 元数据别名的应用服务，协调命令与查询。
pub struct BizMetadataAliasService<R>
where
    R: BizMetadataAliasRepository,
{
    repository: R,
}

impl<R> BizMetadataAliasService<R>
where
    R: BizMetadataAliasRepository,
{
    /// 构造服务。
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// 创建别名。
    pub async fn create_alias(
        &self,
        cmd: CreateBizMetadataAliasCommand,
    ) -> Result<BizMetadataAlias, DomainError> {
        let mut alias = BizMetadataAlias::new(cmd.metadata_id, cmd.alias)?;
        if let Some(src) = cmd.source {
            alias.change_source(src)?;
        }
        if let Some(weight) = cmd.weight {
            alias.change_weight(weight.value())?;
        }
        if let Some(is_primary) = cmd.is_primary {
            alias.set_primary(is_primary)?;
        }
        if let Some(lang) = cmd.language {
            alias.change_language(lang)?;
        }
        self.repository.insert_alias(alias).await
    }

    /// 更新别名。
    pub async fn update_alias(
        &self,
        cmd: UpdateBizMetadataAliasCommand,
    ) -> Result<BizMetadataAlias, DomainError> {
        let mut alias = self
            .repository
            .find_alias_by_id(cmd.id)
            .await?
            .ok_or_else(|| DomainError::Validation {
                message: format!("biz_metadata_alias {} not found", cmd.id.value()),
            })?;

        if let Some(metadata_id) = cmd.metadata_id {
            alias.change_metadata_id(metadata_id)?;
        }

        match cmd.alias {
            AliasFieldUpdate::Keep => {}
            AliasFieldUpdate::Set(value) => alias.update_alias(AliasText::new(value)?)?,
            AliasFieldUpdate::Clear => {
                return Err(DomainError::Validation {
                    message: "alias cannot be cleared".into(),
                });
            }
        }

        if let Some(src) = cmd.source {
            alias.change_source(src)?;
        }
        if let Some(weight) = cmd.weight {
            alias.change_weight(weight.value())?;
        }
        if let Some(is_primary) = cmd.is_primary {
            alias.set_primary(is_primary)?;
        }
        if let Some(lang) = cmd.language {
            alias.change_language(lang)?;
        }

        self.repository.update_alias(alias).await
    }

    /// 删除别名。
    pub async fn delete_alias(&self, id: BizMetadataAliasId) -> Result<(), DomainError> {
        self.repository.delete_alias(id).await
    }

    /// 查询单个别名。
    pub async fn find_by_id(
        &self,
        id: BizMetadataAliasId,
    ) -> Result<Option<BizMetadataAlias>, DomainError> {
        self.repository.find_alias_by_id(id).await
    }

    /// 分页查询别名。
    pub async fn query_alias(
        &self,
        request: BizMetadataAliasQueryRequest,
    ) -> Result<PageResult<BizMetadataAlias>, DomainError> {
        self.repository
            .query_alias(request.expression, request.options)
            .await
    }

    /// 仓储访问器，便于测试。
    pub fn repository(&self) -> &R {
        &self.repository
    }
}
