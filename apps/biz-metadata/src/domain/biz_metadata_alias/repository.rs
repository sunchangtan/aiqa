use domain_core::prelude::{Expression, QueryOptions, Repository};

use super::BizMetadataAlias;
use super::value_object::BizMetadataAliasId;

/// `biz_metadata_alias` 的仓储抽象。
pub trait BizMetadataAliasRepository: Repository<BizMetadataAlias> {
    fn insert_alias(&self, alias: BizMetadataAlias) -> Self::InsertFuture<'_> {
        self.insert(alias)
    }

    fn update_alias(&self, alias: BizMetadataAlias) -> Self::UpdateFuture<'_> {
        self.update(alias)
    }

    fn delete_alias(&self, id: BizMetadataAliasId) -> Self::DeleteFuture<'_> {
        self.delete(id)
    }

    fn find_alias_by_id(&self, id: BizMetadataAliasId) -> Self::FindByIdFuture<'_> {
        self.find_by_id(id)
    }

    fn query_alias(&self, expr: Expression, options: QueryOptions) -> Self::QueryFuture<'_> {
        self.query(expr, options)
    }
}
