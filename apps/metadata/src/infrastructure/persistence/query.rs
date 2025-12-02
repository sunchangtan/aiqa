use domain_core::expression::{Comparison, Expression, FilterValue, OrderBy, SortDirection};
use sea_orm::{Condition, EntityTrait, Order, QueryOrder, Select};

/// 根据表达式构建 ORM 条件，比较节点交由 `handler` 解析。
pub fn build_condition(
    expr: &Expression,
    handler: &impl Fn(&Comparison) -> Option<Condition>,
) -> Condition {
    match expr {
        Expression::Comparison(cmp) => handler(cmp).unwrap_or_else(Condition::all),
        Expression::And(children) => children.iter().fold(Condition::all(), |acc, child| {
            acc.add(build_condition(child, handler))
        }),
        Expression::Or(children) => children.iter().fold(Condition::any(), |acc, child| {
            acc.add(build_condition(child, handler))
        }),
        Expression::Not(child) => build_condition(child, handler).not(),
        Expression::True => Condition::all(),
        Expression::False => Condition::all().add(Condition::any()).not(),
    }
}

/// 基于等于/不等于的常用条件构造，字段解析逻辑由调用方提供。
pub fn build_eq_ne_condition(
    expr: &Expression,
    resolver: &impl Fn(&str, &FilterValue, bool) -> Option<Condition>,
) -> Condition {
    build_condition(expr, &|cmp| match cmp {
        Comparison::Eq { field, value } => resolver(field, value, false),
        Comparison::Ne { field, value } => resolver(field, value, true),
        _ => None,
    })
}

/// 应用排序字段，解析逻辑交由 `resolver` 决定。
pub fn apply_ordering<E>(
    mut query: Select<E>,
    order_bys: &[OrderBy],
    resolver: &impl Fn(&OrderBy) -> Option<(E::Column, Order)>,
) -> Select<E>
where
    E: EntityTrait,
{
    for order in order_bys {
        if let Some((column, direction)) = resolver(order) {
            query = query.order_by(column, direction);
        }
    }
    query
}

/// 将领域层的排序方向转换为 SeaORM 的排序枚举。
pub fn resolve_order_direction(direction: &SortDirection) -> Order {
    match direction {
        SortDirection::Asc => Order::Asc,
        SortDirection::Desc => Order::Desc,
    }
}

/// 分页参数结构，封装分页计算逻辑。
#[derive(Debug, Clone, Copy)]
pub struct PaginationParams {
    /// 每页大小
    pub limit: u64,
    /// 页索引（从0开始）
    pub page_index: u64,
}

impl PaginationParams {
    /// 根据 limit 和 offset 计算分页参数。
    pub fn compute(limit: Option<u64>, offset: Option<u64>, default_page_size: u64) -> Self {
        let limit = limit.unwrap_or(default_page_size).max(1);
        let offset = offset.unwrap_or(0);
        let page_index = if limit == 0 { 0 } else { offset / limit };

        Self { limit, page_index }
    }
}
