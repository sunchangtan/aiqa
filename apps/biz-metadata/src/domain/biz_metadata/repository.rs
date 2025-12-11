use super::BizMetadata;
use super::value_object::BizMetadataId;
use domain_core::prelude::{Expression, QueryOptions, Repository};

pub trait BizMetadataRepository: Repository<BizMetadata> {
    fn insert_biz_metadata(&self, biz_metadata: BizMetadata) -> Self::InsertFuture<'_> {
        self.insert(biz_metadata)
    }

    fn update_biz_metadata(&self, biz_metadata: BizMetadata) -> Self::UpdateFuture<'_> {
        self.update(biz_metadata)
    }

    fn delete_biz_metadata(&self, id: BizMetadataId) -> Self::DeleteFuture<'_> {
        self.delete(id)
    }

    fn find_biz_metadata_by_id(&self, id: BizMetadataId) -> Self::FindByIdFuture<'_> {
        self.find_by_id(id)
    }

    fn query_biz_metadata(&self, expr: Expression, options: QueryOptions) -> Self::QueryFuture<'_> {
        self.query(expr, options)
    }
}
