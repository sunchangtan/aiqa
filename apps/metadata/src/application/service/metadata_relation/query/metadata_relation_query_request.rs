use domain_core::expression::{Expression, QueryOptions};

/// 查询元数据关系的请求载体。
#[derive(Debug, Clone)]
pub struct MetadataRelationQueryRequest {
    pub expression: Expression,
    pub options: QueryOptions,
}

impl MetadataRelationQueryRequest {
    /// 创建一个新的查询请求。
    ///
    /// # Examples
    /// ```
    /// use domain_core::expression::{Expression, QueryOptions};
    /// use metadata::MetadataRelationQueryRequest;
    ///
    /// let req = MetadataRelationQueryRequest::new(Expression::True, QueryOptions::default());
    /// assert!(matches!(req.expression, Expression::True));
    /// ```
    pub fn new(expression: Expression, options: QueryOptions) -> Self {
        Self {
            expression,
            options,
        }
    }
}
