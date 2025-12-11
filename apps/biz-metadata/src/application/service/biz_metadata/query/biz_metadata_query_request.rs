use domain_core::expression::{Expression, QueryOptions};

/// 分页查询请求，封装表达式与分页选项。
///
/// # Examples
/// ```
/// use domain_core::expression::Expression;
/// use domain_core::expression::QueryOptions;
/// use biz_metadata::BizMetadataQueryRequest;
///
/// let req = BizMetadataQueryRequest::new(Expression::True, QueryOptions::default());
/// assert!(matches!(req.expression, Expression::True));
/// ```
pub struct BizMetadataQueryRequest {
    pub expression: Expression,
    pub options: QueryOptions,
}

impl BizMetadataQueryRequest {
    pub fn new(expression: Expression, options: QueryOptions) -> Self {
        Self {
            expression,
            options,
        }
    }
}
