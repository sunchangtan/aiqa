use domain_core::prelude::{Expression, QueryOptions, Repository};

use crate::domain::metadata_relation::MetadataRelation;
use crate::domain::metadata_relation::value_object::MetadataRelationId;

/// 元数据关系的仓储接口，抽象持久化操作。
///
/// # 示例
/// 使用内存实现演示接口用法：
/// ```
/// use domain_core::domain_error::DomainError;
/// use domain_core::expression::{Expression, QueryOptions};
/// use domain_core::pagination::PageResult;
/// use metadata::{MetadataRelation, MetadataRelationId};
/// use metadata::MetadataRelationRepository;
/// use std::future::{ready, Ready};
///
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
/// ```
pub trait MetadataRelationRepository: Repository<MetadataRelation> {
    fn insert_relation(&self, relation: MetadataRelation) -> Self::InsertFuture<'_> {
        self.insert(relation)
    }

    fn update_relation(&self, relation: MetadataRelation) -> Self::UpdateFuture<'_> {
        self.update(relation)
    }

    fn delete_relation(&self, id: MetadataRelationId) -> Self::DeleteFuture<'_> {
        self.delete(id)
    }

    fn find_relation_by_id(&self, id: MetadataRelationId) -> Self::FindByIdFuture<'_> {
        self.find_by_id(id)
    }

    fn query_relations(&self, expr: Expression, options: QueryOptions) -> Self::QueryFuture<'_> {
        self.query(expr, options)
    }
}
