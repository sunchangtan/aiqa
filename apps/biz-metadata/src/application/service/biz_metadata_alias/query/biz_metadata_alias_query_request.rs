use domain_core::expression::{Expression, QueryOptions};

/// 别名查询请求，封装筛选表达式与分页排序。
///
/// # Examples
/// ```
/// use biz_metadata::BizMetadataAliasQueryRequest;
/// use domain_core::expression::{Expression, QueryOptions};
///
/// let req = BizMetadataAliasQueryRequest {
///     expression: Expression::True,
///     options: QueryOptions::default(),
/// };
/// assert!(matches!(req.expression, Expression::True));
/// ```
pub struct BizMetadataAliasQueryRequest {
    pub expression: Expression,
    pub options: QueryOptions,
}
