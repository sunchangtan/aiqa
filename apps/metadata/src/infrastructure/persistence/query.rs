use domain_core::expression::{Comparison, Expression, OrderBy};
use sea_orm::{Condition, EntityTrait, Order, QueryOrder, Select};

/// 根据表达式构建 ORM 条件，比较节点交由 `handler` 解析。
pub fn build_condition(
    expr: &Expression,
    handler: &impl Fn(&Comparison) -> Option<Condition>,
) -> Condition {
    match expr {
        Expression::Comparison(cmp) => handler(cmp).unwrap_or_else(Condition::all),
        Expression::And(children) => children
            .iter()
            .fold(Condition::all(), |acc, child| acc.add(build_condition(child, handler))),
        Expression::Or(children) => children
            .iter()
            .fold(Condition::any(), |acc, child| acc.add(build_condition(child, handler))),
        Expression::Not(child) => build_condition(child, handler).not(),
        Expression::True => Condition::all(),
        Expression::False => Condition::all().add(Condition::any()).not(),
    }
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
