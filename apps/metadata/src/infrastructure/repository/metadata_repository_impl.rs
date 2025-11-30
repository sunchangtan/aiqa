use domain_core::domain_error::DomainError;
use domain_core::expression::{Comparison, Expression, FilterValue, OrderBy, QueryOptions, SortDirection};
use domain_core::pagination::{PageResult, DEFAULT_PAGE_SIZE};
use domain_core::repository::Repository;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order as SeaOrder, QueryFilter};
use sea_orm::PaginatorTrait;

use crate::domain::metadata::metadata::Metadata;
use crate::domain::metadata::repository::MetadataRepository;
use crate::domain::metadata::value_object::MetadataId;
use crate::infrastructure::mapper::metadata_mapping;
use crate::infrastructure::persistence::entity::metadata;
use crate::infrastructure::persistence::entity::prelude::Metadata as MetadataEntity;
use crate::infrastructure::persistence::future::{repo_future, RepoFuture};
use crate::infrastructure::persistence::query::{apply_ordering, build_condition};


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
        Some(if negate {
            condition.not()
        } else {
            condition
        })
    }

    fn cond_eq<T>(column: metadata::Column, value: T) -> Condition
    where
        T: Into<sea_orm::Value>,
    {
        Condition::all().add(column.eq(value))
    }

    fn resolve_order(order: &OrderBy) -> Option<(metadata::Column, SeaOrder)> {
        Self::column_for(&order.field).map(|column| {
            let dir = match order.direction {
                SortDirection::Asc => SeaOrder::Asc,
                SortDirection::Desc => SeaOrder::Desc,
            };
            (column, dir)
        })
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
    type InsertFuture<'a> = RepoFuture<'a, ()> where Self: 'a;
    type UpdateFuture<'a> = RepoFuture<'a, ()> where Self: 'a;
    type DeleteFuture<'a> = RepoFuture<'a, ()> where Self: 'a;
    type FindByIdFuture<'a> = RepoFuture<'a, Option<Metadata>> where Self: 'a;
    type QueryFuture<'a> = RepoFuture<'a, PageResult<Metadata>> where Self: 'a;

    fn insert(&self, aggregate: Metadata) -> Self::InsertFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let active = metadata_mapping::to_active_model(&aggregate);
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
            let active = metadata_mapping::to_active_model(&aggregate);
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
            Ok(model.map(|m| metadata_mapping::from_entity(&m)))
        })
    }

    fn query(
        &self,
        expr: Expression,
        options: QueryOptions,
    ) -> Self::QueryFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let limit = options.limit.unwrap_or(DEFAULT_PAGE_SIZE).max(1);
            let offset = options.offset.unwrap_or(0);
            let page_index = if limit == 0 { 0 } else { offset / limit };

            let condition = build_condition(&expr, &|cmp| match cmp {
                Comparison::Eq { field, value } => Self::field_condition(field, value, false),
                Comparison::Ne { field, value } => Self::field_condition(field, value, true),
                _ => None,
            });
            let base_query = MetadataEntity::find().filter(condition);
            let ordered_query = apply_ordering(base_query, &options.order_bys, &Self::resolve_order);

            let paginator = ordered_query.paginate(&db, limit);
            let models = paginator
                .fetch_page(u64::try_from(page_index).unwrap_or(0))
                .await
                .map_err(Self::map_db_err)?;

            let total = paginator.num_items().await.map_err(Self::map_db_err)?;

            let items = models
                .into_iter()
                .map(|model| metadata_mapping::from_entity(&model))
                .collect();

            Ok(
                PageResult::builder(items, total)
                    .page_index(page_index)
                    .page_size(limit)
                    .build(),
            )
        })
    }
}

impl MetadataRepository for MetadataRepositoryImpl {}
