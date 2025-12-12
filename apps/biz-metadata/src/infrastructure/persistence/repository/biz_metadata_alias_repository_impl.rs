use crate::domain::biz_metadata_alias::BizMetadataAlias;
use crate::domain::biz_metadata_alias::repository::BizMetadataAliasRepository;
use crate::domain::biz_metadata_alias::value_object::BizMetadataAliasId;
use crate::infrastructure::persistence::entity::biz_metadata_alias;
use crate::infrastructure::persistence::entity::prelude::BizMetadataAlias as BizMetadataAliasEntity;
use crate::infrastructure::persistence::mapper::{
    ActiveModelMapper, EntityMapper, biz_metadata_alias_mapping::BizMetadataAliasMapper,
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

/// SeaORM 版 `biz_metadata_alias` 仓储实现。
pub struct BizMetadataAliasRepositoryImpl {
    db: DatabaseConnection,
}

impl BizMetadataAliasRepositoryImpl {
    /// 构造仓储。
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
            biz_metadata_alias::Column::Id => Self::cond_eq(column, value.as_i64()?),
            biz_metadata_alias::Column::MetadataId => Self::cond_eq(column, value.as_i64()?),
            biz_metadata_alias::Column::Alias
            | biz_metadata_alias::Column::Source
            | biz_metadata_alias::Column::Language => Self::cond_eq(column, value.as_string()?),
            biz_metadata_alias::Column::Weight => {
                let weight = value.as_i64()? as i32;
                Self::cond_eq(column, weight)
            }
            biz_metadata_alias::Column::IsPrimary => Self::cond_eq(column, value.as_bool()?),
            biz_metadata_alias::Column::CreatedAt
            | biz_metadata_alias::Column::UpdatedAt
            | biz_metadata_alias::Column::DeletedAt => None?,
        };
        Some(if negate { condition.not() } else { condition })
    }

    fn cond_eq<T>(column: biz_metadata_alias::Column, value: T) -> Condition
    where
        T: Into<sea_orm::Value>,
    {
        Condition::all().add(column.eq(value))
    }

    fn resolve_order(order: &OrderBy) -> Option<(biz_metadata_alias::Column, SeaOrder)> {
        Self::column_for(&order.field)
            .map(|column| (column, resolve_order_direction(&order.direction)))
    }

    fn column_for(field: &str) -> Option<biz_metadata_alias::Column> {
        match field {
            "id" => Some(biz_metadata_alias::Column::Id),
            "metadata_id" => Some(biz_metadata_alias::Column::MetadataId),
            "alias" => Some(biz_metadata_alias::Column::Alias),
            "source" => Some(biz_metadata_alias::Column::Source),
            "weight" => Some(biz_metadata_alias::Column::Weight),
            "is_primary" => Some(biz_metadata_alias::Column::IsPrimary),
            "language" => Some(biz_metadata_alias::Column::Language),
            "created_at" => Some(biz_metadata_alias::Column::CreatedAt),
            "updated_at" => Some(biz_metadata_alias::Column::UpdatedAt),
            "deleted_at" => Some(biz_metadata_alias::Column::DeletedAt),
            _ => None,
        }
    }
}

impl Repository<BizMetadataAlias> for BizMetadataAliasRepositoryImpl {
    type InsertFuture<'a>
        = RepoFuture<'a, BizMetadataAlias>
    where
        Self: 'a;
    type UpdateFuture<'a>
        = RepoFuture<'a, BizMetadataAlias>
    where
        Self: 'a;
    type DeleteFuture<'a>
        = RepoFuture<'a, ()>
    where
        Self: 'a;
    type FindByIdFuture<'a>
        = RepoFuture<'a, Option<BizMetadataAlias>>
    where
        Self: 'a;
    type QueryFuture<'a>
        = RepoFuture<'a, PageResult<BizMetadataAlias>>
    where
        Self: 'a;

    fn insert(&self, aggregate: BizMetadataAlias) -> Self::InsertFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let active = BizMetadataAliasMapper::map_to_active_model(&aggregate)?;
            let insert_result = BizMetadataAliasEntity::insert(active)
                .exec(&db)
                .await
                .map_err(Self::map_db_err)?;

            let model = BizMetadataAliasEntity::find_by_id(insert_result.last_insert_id)
                .one(&db)
                .await
                .map_err(Self::map_db_err)?
                .ok_or_else(|| DomainError::Persistence {
                    message: format!(
                        "biz_metadata_alias {} not found after insert",
                        insert_result.last_insert_id
                    ),
                })?;

            BizMetadataAliasMapper::map_to_domain(&model)
        })
    }

    fn update(&self, aggregate: BizMetadataAlias) -> Self::UpdateFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let existing = BizMetadataAliasEntity::find_by_id(aggregate.id().value())
                .one(&db)
                .await
                .map_err(Self::map_db_err)?
                .ok_or_else(|| DomainError::Persistence {
                    message: format!("biz_metadata_alias {} not found", aggregate.id().value()),
                })?;

            let mut active: biz_metadata_alias::ActiveModel = existing.into();

            BizMetadataAliasMapper::apply_changes(&aggregate, &mut active)?;

            let updated_model = active.update(&db).await.map_err(Self::map_db_err)?;

            BizMetadataAliasMapper::map_to_domain(&updated_model)
        })
    }

    fn delete(&self, id: BizMetadataAliasId) -> Self::DeleteFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            BizMetadataAliasEntity::delete_many()
                .filter(biz_metadata_alias::Column::Id.eq(id.value()))
                .exec(&db)
                .await
                .map(|_| ())
                .map_err(Self::map_db_err)
        })
    }

    fn find_by_id(&self, id: BizMetadataAliasId) -> Self::FindByIdFuture<'_> {
        let db = self.db.clone();
        repo_future(async move {
            let model = BizMetadataAliasEntity::find_by_id(id.value())
                .one(&db)
                .await
                .map_err(Self::map_db_err)?;
            model
                .map(|m| BizMetadataAliasMapper::map_to_domain(&m))
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
            let base_query = BizMetadataAliasEntity::find().filter(condition);
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
                .map(BizMetadataAliasMapper::map_to_domain)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(PageResult::builder(items, total)
                .page_index(pagination.page_index)
                .page_size(pagination.limit)
                .build())
        })
    }
}

impl BizMetadataAliasRepository for BizMetadataAliasRepositoryImpl {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_resolver_supports_alias() {
        let cond = BizMetadataAliasRepositoryImpl::field_condition(
            "alias",
            &FilterValue::from("foo"),
            false,
        );
        assert!(cond.is_some());
    }

    #[test]
    fn maps_db_errors() {
        let err = sea_orm::DbErr::Custom("oops".into());
        let mapped = BizMetadataAliasRepositoryImpl::map_db_err(err);
        assert!(matches!(mapped, DomainError::Persistence { .. }));
    }
}
