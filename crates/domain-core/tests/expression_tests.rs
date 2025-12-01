use domain_core::expression::*;

#[test]
fn combine_and_or_not() {
    let expr = Expression::and(vec![
        Expression::cmp(eq("status", "active")),
        Expression::or(vec![
            Expression::cmp(gt("score", 80)),
            Expression::negate(Expression::cmp(le("score", 30))),
        ]),
    ]);
    match expr {
        Expression::And(list) => {
            assert_eq!(list.len(), 2);
        }
        _ => panic!("unexpected expression tree"),
    }
}

#[test]
fn between_and_sort_options() {
    let cmp = between("created_at", 1_i64, 10_i64);
    match cmp {
        Comparison::Between { field, .. } => assert_eq!(field, "created_at"),
        _ => panic!("expect between variant"),
    }

    let opts = QueryOptions::new(Some(10), Some(5)).with_order_by(OrderBy::desc("score"));
    assert_eq!(opts.limit, Some(10));
    assert_eq!(opts.order_bys.len(), 1);
    assert_eq!(opts.order_bys[0].direction, SortDirection::Desc);
}
