use domain_core::domain_error::DomainError;
use domain_core::expression::{Expression, FilterValue, OrderBy, QueryOptions};
use domain_core::pagination::{DEFAULT_PAGE_SIZE, PageResult};
use domain_core::repository::Repository;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order as SeaOrder,
    PaginatorTrait, QueryFilter,
};

use crate::domain::metadata_relation::MetadataRelation;
use crate::domain::metadata_relation::repository::MetadataRelationRepository;
use crate::domain::metadata_relation::value_object::MetadataRelationId;
use crate::infrastructure::persistence::entity::metadata_relation;
use crate::infrastructure::persistence::entity::prelude::MetadataRelation as MetadataRelationEntity;
use crate::infrastructure::persistence::mapper::{
    ActiveModelMapper, EntityMapper, metadata_relation_mapping::MetadataRelationMapper,
};
use crate::infrastructure::persistence::query::{
    PaginationParams, apply_ordering, build_eq_ne_condition, resolve_order_direction,
};
use crate::infrastructure::persistence::repository::future::{RepoFuture, repo_future};

/// SeaORM 实现的元数据关系仓储。
pub struct MetadataRelationRepositoryImpl {
    db: DatabaseConnection,
}

impl MetadataRelationRepositoryImpl {
    /// 基于数据库连接创建仓储实现。
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn map_db_err(err: sea_orm::DbErr) -> DomainError {
        DomainError::Persistence {
            message: err.to_string(),
        }
    }

    fn field_condition(field: &str, value: &FilterValue, negate: bool) -> Option<Condition> {
        let column = Self::column_for(field)?;
        let condition = match column {
            metadata_relation::Column::Id
            | metadata_relation::Column::FromMetadataId
            | metadata_relation::Column::ToMetadataId => Self::cond_eq(column, value.as_i64()?),
            _ => None?,
        };
        Some(if negate { condition.not() } else { condition })
    }

    fn cond_eq<T>(column: metadata_relation::Column, value: T) -> Condition
    where
        T: Into<sea_orm::Value>,
    {
        Condition::all().add(column.eq(value))
    }

    fn resolve_order(order: &OrderBy) -> Option<(metadata_relation::Column, SeaOrder)> {
        Self::column_for(&order.field)
            .map(|column| (column, resolve_order_direction(&order.direction)))
    }

    fn column_for(field: &str) -> Option<metadata_relation::Column> {
        match field {
            "id" => Some(metadata_relation::Column::Id),
            "from_metadata_id" => Some(metadata_relation::Column::FromMetadataId),
            "to_metadata_id" => Some(metadata_relation::Column::ToMetadataId),
            "created_at" => Some(metadata_relation::Column::CreatedAt),
            "updated_at" => Some(metadata_relation::Column::UpdatedAt),
            "delete_at" => Some(metadata_relation::Column::DeleteAt),
            _ => None,
        }
    }
}

impl Repository<MetadataRelation> for MetadataRelationRepositoryImpl {
    type InsertFuture<'a>
        = RepoFuture<'a, ()>
    where
        Self: 'a;
    type UpdateFuture<'a>
        = RepoFuture<'a, ()>
    where
        Self: 'a;
    type DeleteFuture<'a>
        = RepoFuture<'a, ()>
    where
        Self: 'a;
    type FindByIdFuture<'a>
        = RepoFuture<'a, Option<MetadataRelation>>
    where
        Self: 'a;
    type QueryFuture<'a>
        = RepoFuture<'a, PageResult<MetadataRelation>>
    where
        Self: 'a;

    fn insert(&self, aggregate: MetadataRelation) -> Self::InsertFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let active = MetadataRelationMapper::map_to_active_model(&aggregate)?;
            MetadataRelationEntity::insert(active)
                .exec(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn update(&self, aggregate: MetadataRelation) -> Self::UpdateFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let active = MetadataRelationMapper::map_to_active_model(&aggregate)?;
            active
                .update(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn delete(&self, id: MetadataRelationId) -> Self::DeleteFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            MetadataRelationEntity::delete_many()
                .filter(metadata_relation::Column::Id.eq(id.value()))
                .exec(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn find_by_id(&self, id: MetadataRelationId) -> Self::FindByIdFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let model = MetadataRelationEntity::find_by_id(id.value())
                .one(&db)
                .await
                .map_err(Self::map_db_err)?;
            model
                .map(|m| MetadataRelationMapper::map_to_domain(&m))
                .transpose()
        })
    }

    fn query(&self, expr: Expression, options: QueryOptions) -> Self::QueryFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let pagination =
                PaginationParams::compute(options.limit, options.offset, DEFAULT_PAGE_SIZE);

            let condition = build_eq_ne_condition(&expr, &|field, value, neg| {
                Self::field_condition(field, value, neg)
            });
            let base_query = MetadataRelationEntity::find().filter(condition);
            let ordered_query =
                apply_ordering(base_query, &options.order_bys, &Self::resolve_order);

            let paginator = ordered_query.paginate(&db, pagination.limit);
            let models = paginator
                .fetch_page(pagination.page_index)
                .await
                .map_err(Self::map_db_err)?;

            let total = paginator.num_items().await.map_err(Self::map_db_err)?;

            let items = models
                .iter()
                .map(MetadataRelationMapper::map_to_domain)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(PageResult::builder(items, total)
                .page_index(pagination.page_index)
                .page_size(pagination.limit)
                .build())
        })
    }
}

impl MetadataRelationRepository for MetadataRelationRepositoryImpl {}
