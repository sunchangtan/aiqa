use domain_core::expression::{Expression, QueryOptions};

/// 分页查询请求，封装表达式与分页选项。
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
