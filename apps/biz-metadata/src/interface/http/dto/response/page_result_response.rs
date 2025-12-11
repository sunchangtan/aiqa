use serde::Serialize;
use utoipa::ToSchema;

use domain_core::pagination::{Page, PageResult};

/// HTTP 层通用的分页响应载荷，基于领域层 [`PageResult`] 转换。
#[derive(Debug, Serialize, ToSchema)]
pub struct PageResultResponse<T>
where
    T: Serialize + utoipa::ToSchema + Send + Sync,
{
    /// 当前页的数据列表。
    pub items: Vec<T>,
    /// 记录总数。
    pub total_count: u64,
    /// 当前页索引。
    pub page_index: u64,
    /// 每页大小。
    pub page_size: u64,
    /// 页索引起始值。
    pub index_from: u64,
}

impl<T> PageResultResponse<T>
where
    T: Serialize + utoipa::ToSchema + Send + Sync,
{
    /// 从领域层分页结果构建。
    pub fn from_page(page: PageResult<T>) -> Self {
        Self {
            total_count: page.total_count(),
            page_index: page.page_index(),
            page_size: page.page_size(),
            index_from: page.index_from(),
            items: page.into_items(),
        }
    }
}
