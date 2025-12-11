use crate::domain::biz_metadata::BizMetadata;
use crate::domain::biz_metadata::repository::BizMetadataRepository;
use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::infrastructure::persistence::entity::biz_metadata;
use crate::infrastructure::persistence::entity::prelude::BizMetadata as BizMetadataEntity;
use crate::infrastructure::persistence::mapper::{
    ActiveModelMapper, EntityMapper, biz_metadata_mapping::BizMetadataMapper,
};
use crate::infrastructure::persistence::query::{
    PaginationParams, apply_ordering, build_eq_ne_condition, resolve_order_direction,
};
use crate::infrastructure::persistence::repository::future::{RepoFuture, repo_future};
use domain_core::domain_error::DomainError;
use domain_core::expression::{Expression, FilterValue, OrderBy, QueryOptions};
use domain_core::pagination::{DEFAULT_PAGE_SIZE, PageResult};
use domain_core::repository::Repository;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order as SeaOrder,
    PaginatorTrait, QueryFilter,
};

pub struct BizMetadataRepositoryImpl {
    db: DatabaseConnection,
}

impl BizMetadataRepositoryImpl {
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
            biz_metadata::Column::Id => Self::cond_eq(column, value.as_i64()?),
            biz_metadata::Column::Code
            | biz_metadata::Column::Name
            | biz_metadata::Column::Description
            | biz_metadata::Column::MetaType
            | biz_metadata::Column::DataClass
            | biz_metadata::Column::Status
            | biz_metadata::Column::ValueType
            | biz_metadata::Column::Unit => Self::cond_eq(column, value.as_string()?),
            biz_metadata::Column::OwnerId => Self::cond_eq(column, value.as_i64()?),
            biz_metadata::Column::IsIdentifier => Self::cond_eq(column, value.as_bool()?),
            _ => None?,
        };
        Some(if negate { condition.not() } else { condition })
    }

    fn cond_eq<T>(column: biz_metadata::Column, value: T) -> Condition
    where
        T: Into<sea_orm::Value>,
    {
        Condition::all().add(column.eq(value))
    }

    fn resolve_order(order: &OrderBy) -> Option<(biz_metadata::Column, SeaOrder)> {
        Self::column_for(&order.field)
            .map(|column| (column, resolve_order_direction(&order.direction)))
    }

    fn column_for(field: &str) -> Option<biz_metadata::Column> {
        match field {
            "id" => Some(biz_metadata::Column::Id),
            "code" => Some(biz_metadata::Column::Code),
            "name" => Some(biz_metadata::Column::Name),
            "description" => Some(biz_metadata::Column::Description),
            "meta_type" => Some(biz_metadata::Column::MetaType),
            "owner_id" => Some(biz_metadata::Column::OwnerId),
            "data_class" => Some(biz_metadata::Column::DataClass),
            "value_type" => Some(biz_metadata::Column::ValueType),
            "unit" => Some(biz_metadata::Column::Unit),
            "is_identifier" => Some(biz_metadata::Column::IsIdentifier),
            "status" => Some(biz_metadata::Column::Status),
            "created_at" => Some(biz_metadata::Column::CreatedAt),
            "updated_at" => Some(biz_metadata::Column::UpdatedAt),
            "deleted_at" => Some(biz_metadata::Column::DeletedAt),
            _ => None,
        }
    }
}

impl Repository<BizMetadata> for BizMetadataRepositoryImpl {
    type InsertFuture<'a>
        = RepoFuture<'a, BizMetadata>
    where
        Self: 'a;
    type UpdateFuture<'a>
        = RepoFuture<'a, BizMetadata>
    where
        Self: 'a;
    type DeleteFuture<'a>
        = RepoFuture<'a, ()>
    where
        Self: 'a;
    type FindByIdFuture<'a>
        = RepoFuture<'a, Option<BizMetadata>>
    where
        Self: 'a;
    type QueryFuture<'a>
        = RepoFuture<'a, PageResult<BizMetadata>>
    where
        Self: 'a;

    fn insert(&self, aggregate: BizMetadata) -> Self::InsertFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let active = BizMetadataMapper::map_to_active_model(&aggregate)?;
            let insert_result = BizMetadataEntity::insert(active)
                .exec(&db)
                .await
                .map_err(Self::map_db_err)?;

            let model = BizMetadataEntity::find_by_id(insert_result.last_insert_id)
                .one(&db)
                .await
                .map_err(Self::map_db_err)?
                .ok_or_else(|| DomainError::Persistence {
                    message: format!(
                        "biz_metadata {} not found after insert",
                        insert_result.last_insert_id
                    ),
                })?;

            BizMetadataMapper::map_to_domain(&model)
        })
    }

    fn update(&self, aggregate: BizMetadata) -> Self::UpdateFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let existing = BizMetadataEntity::find_by_id(aggregate.id().value())
                .one(&db)
                .await
                .map_err(Self::map_db_err)?
                .ok_or_else(|| DomainError::Persistence {
                    message: format!("biz_metadata {} not found", aggregate.id().value()),
                })?;

            let mut active: biz_metadata::ActiveModel = existing.into();

            BizMetadataMapper::apply_changes(&aggregate, &mut active)?;

            let updated_model = active.update(&db).await.map_err(Self::map_db_err)?;

            BizMetadataMapper::map_to_domain(&updated_model)
        })
    }

    fn delete(&self, id: BizMetadataId) -> Self::DeleteFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            BizMetadataEntity::delete_many()
                .filter(biz_metadata::Column::Id.eq(id.value()))
                .exec(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn find_by_id(&self, id: BizMetadataId) -> Self::FindByIdFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let model = BizMetadataEntity::find_by_id(id.value())
                .one(&db)
                .await
                .map_err(Self::map_db_err)?;
            model
                .map(|m| BizMetadataMapper::map_to_domain(&m))
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
            let base_query = BizMetadataEntity::find().filter(condition);
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
                .map(BizMetadataMapper::map_to_domain)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(PageResult::builder(items, total)
                .page_index(pagination.page_index)
                .page_size(pagination.limit)
                .build())
        })
    }
}

impl BizMetadataRepository for BizMetadataRepositoryImpl {}
