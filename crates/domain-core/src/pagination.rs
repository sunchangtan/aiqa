//! 通用分页抽象，辅助仓储返回分页数据。

/// 默认分页大小，供仓储或查询层复用。
pub const DEFAULT_PAGE_SIZE: u64 = 20;

/// 泛型分页结果 trait，描述分页必要的元数据与访问方法。
pub trait Page<T>: Send + Sync
where
    T: Send + Sync,
{
    /// 当前页的数据项集合引用。
    fn items(&self) -> &[T];
    /// 分页索引的起始值，通常为 0 或 1。
    fn index_from(&self) -> u64;
    /// 当前页索引（基于 [`index_from`](Self::index_from)）。
    fn page_index(&self) -> u64;
    /// 每页最大记录数（PageSize）。
    fn page_size(&self) -> u64;
    /// 记录总数（TotalCount）。
    fn total_count(&self) -> u64;

    /// 计算总页数（TotalPages），若不能整除则向上取整。
    fn total_pages(&self) -> u64 {
        let page_size = self.page_size();
        if page_size == 0 {
            return 0;
        }
        (self.total_count() + page_size - 1) / page_size
    }

    /// 是否存在上一页（HasPreviousPage）。
    fn has_previous_page(&self) -> bool {
        self.page_index().saturating_sub(self.index_from()) > 0
    }

    /// 是否存在下一页（HasNextPage）。
    fn has_next_page(&self) -> bool {
        let relative_index = self.page_index().saturating_sub(self.index_from());
        let total_pages = self.total_pages();
        total_pages > 0 && relative_index.saturating_add(1) < total_pages
    }

    /// 是否还有更多数据可分页读取（等同 [`has_next_page`](Self::has_next_page)）。
    fn has_more(&self) -> bool {
        self.has_next_page()
    }
}

/// 默认的分页实体实现，持有真实数据集合。
pub struct PageResult<T> {
    items: Vec<T>,
    index_from: u64,
    page_index: u64,
    page_size: u64,
    total_count: u64,
}

/// 分页构建器，允许按需覆盖分页参数。
pub struct PageResultBuilder<T> {
    items: Vec<T>,
    total_count: u64,
    page_index: u64,
    page_size: Option<u64>,
    index_from: Option<u64>,
}

impl<T> PageResult<T>
where
    T: Send + Sync,
{
    /// 以构建器方式创建分页结果。
    pub fn builder(items: Vec<T>, total_count: u64) -> PageResultBuilder<T> {
        PageResultBuilder {
            items,
            total_count,
            page_index: 0,
            page_size: None,
            index_from: None,
        }
    }

    /// 创建一个新的分页结果，保留基础计数信息。
    ///
    /// # 参数
    /// - `items`: 当前页携带的数据集合。
    /// - `total_count`: 符合筛选条件的记录总数。
    /// - `page_index`: 当前页索引，遵循 `index_from` 的计数方式。
    /// - `page_size`: 每页允许返回的最大记录数。
    /// - `index_from`: 页索引起点，通常为 `0` 或 `1`。
    pub fn new(
        items: Vec<T>,
        total_count: u64,
        page_index: u64,
        page_size: Option<u64>,
        index_from: Option<u64>,
    ) -> Self {
        let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let index_from = index_from.unwrap_or(0);

        Self {
            items,
            index_from,
            page_index,
            page_size,
            total_count,
        }
    }

    /// 返回一个空结果，常用于无匹配记录的查询，保留分页参数一致性。
    pub fn empty(page_size: Option<u64>, page_index: u64, index_from: Option<u64>) -> Self {
        Self::new(Vec::new(), 0, page_index, page_size, index_from)
    }

    /// 消费分页结果并返回内部数据集合。
    pub fn into_items(self) -> Vec<T> {
        self.items
    }
}

impl<T> PageResultBuilder<T>
where
    T: Send + Sync,
{
    pub fn page_index(mut self, page_index: u64) -> Self {
        self.page_index = page_index;
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn index_from(mut self, index_from: u64) -> Self {
        self.index_from = Some(index_from);
        self
    }

    pub fn build(self) -> PageResult<T> {
        PageResult::new(
            self.items,
            self.total_count,
            self.page_index,
            self.page_size,
            self.index_from,
        )
    }
}

impl<T> Page<T> for PageResult<T>
where
    T: Send + Sync,
{
    fn items(&self) -> &[T] {
        &self.items
    }

    fn index_from(&self) -> u64 {
        self.index_from
    }

    fn page_index(&self) -> u64 {
        self.page_index
    }

    fn page_size(&self) -> u64 {
        self.page_size
    }

    fn total_count(&self) -> u64 {
        self.total_count
    }
}
