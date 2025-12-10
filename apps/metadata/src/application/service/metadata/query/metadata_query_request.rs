use domain_core::expression::{Expression, QueryOptions};

/// 分页查询请求，封装表达式与分页选项。
///
/// # Examples
/// ```
/// use domain_core::expression::Expression;
/// use domain_core::expression::QueryOptions;
/// use metadata::MetadataQueryRequest;
///
/// let req = MetadataQueryRequest::new(Expression::True, QueryOptions::default());
/// assert!(matches!(req.expression, Expression::True));
/// ```
pub struct MetadataQueryRequest {
    pub expression: Expression,
    pub options: QueryOptions,
}

impl MetadataQueryRequest {
    pub fn new(expression: Expression, options: QueryOptions) -> Self {
        Self {
            expression,
            options,
        }
    }
}
