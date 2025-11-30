use domain_core::prelude::{Expression, QueryOptions, Repository};
use super::metadata::Metadata;
use super::value_object::MetadataId;

pub trait MetadataRepository: Repository<Metadata> {
    fn insert_metadata(&self, metadata: Metadata) -> Self::InsertFuture<'_> {
        self.insert(metadata)
    }

    fn update_metadata(&self, metadata: Metadata) -> Self::UpdateFuture<'_> {
        self.update(metadata)
    }

    fn delete_metadata(&self, id: MetadataId) -> Self::DeleteFuture<'_> {
        self.delete(id)
    }

    fn find_metadata_by_id(&self, id: MetadataId) -> Self::FindByIdFuture<'_> {
        self.find_by_id(id)
    }

    fn query_metadata(&self, expr: Expression, options: QueryOptions) -> Self::QueryFuture<'_> {
        self.query(expr, options)
    }
}

