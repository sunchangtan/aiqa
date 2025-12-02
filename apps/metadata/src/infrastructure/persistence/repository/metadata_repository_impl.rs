use domain_core::domain_error::DomainError;
use domain_core::expression::{Expression, FilterValue, OrderBy, QueryOptions};
use domain_core::pagination::{DEFAULT_PAGE_SIZE, PageResult};
use domain_core::repository::Repository;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order as SeaOrder,
    PaginatorTrait, QueryFilter,
};

use crate::domain::metadata::Metadata;
use crate::domain::metadata::repository::MetadataRepository;
use crate::domain::metadata::value_object::MetadataId;
use crate::infrastructure::persistence::entity::metadata;
use crate::infrastructure::persistence::entity::prelude::Metadata as MetadataEntity;
use crate::infrastructure::persistence::mapper::{
    ActiveModelMapper, EntityMapper, metadata_mapping::MetadataMapper,
};
use crate::infrastructure::persistence::query::{
    PaginationParams, apply_ordering, build_eq_ne_condition, resolve_order_direction,
};
use crate::infrastructure::persistence::repository::future::{RepoFuture, repo_future};

pub struct MetadataRepositoryImpl {
    db: DatabaseConnection,
}

impl MetadataRepositoryImpl {
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
            metadata::Column::Id => Self::cond_eq(column, value.as_i64()?),
            metadata::Column::Code | metadata::Column::Name | metadata::Column::ValueType => {
                Self::cond_eq(column, value.as_string()?)
            }
            metadata::Column::MetadataType => Self::cond_eq(column, value.as_string()?),
            metadata::Column::IsChainable
            | metadata::Column::IsFilterable
            | metadata::Column::IsSortable
            | metadata::Column::IsGroupable
            | metadata::Column::IsRelationDerived => Self::cond_eq(column, value.as_bool()?),
            _ => None?,
        };
        Some(if negate { condition.not() } else { condition })
    }

    fn cond_eq<T>(column: metadata::Column, value: T) -> Condition
    where
        T: Into<sea_orm::Value>,
    {
        Condition::all().add(column.eq(value))
    }

    fn resolve_order(order: &OrderBy) -> Option<(metadata::Column, SeaOrder)> {
        Self::column_for(&order.field)
            .map(|column| (column, resolve_order_direction(&order.direction)))
    }

    fn column_for(field: &str) -> Option<metadata::Column> {
        match field {
            "id" => Some(metadata::Column::Id),
            "code" => Some(metadata::Column::Code),
            "name" => Some(metadata::Column::Name),
            "metadata_type" => Some(metadata::Column::MetadataType),
            "value_type" => Some(metadata::Column::ValueType),
            "is_chainable" => Some(metadata::Column::IsChainable),
            "is_filterable" => Some(metadata::Column::IsFilterable),
            "is_sortable" => Some(metadata::Column::IsSortable),
            "is_groupable" => Some(metadata::Column::IsGroupable),
            "is_relation_derived" => Some(metadata::Column::IsRelationDerived),
            _ => None,
        }
    }
}

impl Repository<Metadata> for MetadataRepositoryImpl {
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
        = RepoFuture<'a, Option<Metadata>>
    where
        Self: 'a;
    type QueryFuture<'a>
        = RepoFuture<'a, PageResult<Metadata>>
    where
        Self: 'a;

    fn insert(&self, aggregate: Metadata) -> Self::InsertFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let active = MetadataMapper::map_to_active_model(&aggregate)?;
            MetadataEntity::insert(active)
                .exec(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn update(&self, aggregate: Metadata) -> Self::UpdateFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let active = MetadataMapper::map_to_active_model(&aggregate)?;
            active
                .update(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn delete(&self, id: MetadataId) -> Self::DeleteFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            MetadataEntity::delete_many()
                .filter(metadata::Column::Id.eq(id.value()))
                .exec(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn find_by_id(&self, id: MetadataId) -> Self::FindByIdFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let model = MetadataEntity::find_by_id(id.value())
                .one(&db)
                .await
                .map_err(Self::map_db_err)?;
            model.map(|m| MetadataMapper::map_to_domain(&m)).transpose()
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
            let base_query = MetadataEntity::find().filter(condition);
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
                .map(MetadataMapper::map_to_domain)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(PageResult::builder(items, total)
                .page_index(pagination.page_index)
                .page_size(pagination.limit)
                .build())
        })
    }
}

impl MetadataRepository for MetadataRepositoryImpl {}
