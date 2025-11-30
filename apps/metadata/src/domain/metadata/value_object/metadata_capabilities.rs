use domain_core::prelude::ValueObject;

/// 能力位集合，集中管理链式、筛选、排序等布尔标记，
/// 避免在方法签名中传递过多零散参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetadataCapabilities {
    chainable: bool,
    filterable: bool,
    sortable: bool,
    groupable: bool,
    relation_derived: bool,
}

impl MetadataCapabilities {
    /// 构造新的能力集合。
    pub const fn new(
        chainable: bool,
        filterable: bool,
        sortable: bool,
        groupable: bool,
        relation_derived: bool,
    ) -> Self {
        Self {
            chainable,
            filterable,
            sortable,
            groupable,
            relation_derived,
        }
    }

    /// 是否为可链式实体属性。
    pub const fn chainable(self) -> bool {
        self.chainable
    }

    /// 是否可用于筛选条件。
    pub const fn filterable(self) -> bool {
        self.filterable
    }

    /// 是否可用于排序字段。
    pub const fn sortable(self) -> bool {
        self.sortable
    }

    /// 是否可用于分组字段。
    pub const fn groupable(self) -> bool {
        self.groupable
    }

    /// 是否来源于关系衍生指标。
    pub const fn relation_derived(self) -> bool {
        self.relation_derived
    }
}

impl Default for MetadataCapabilities {
    fn default() -> Self {
        Self::new(false, false, false, false, false)
    }
}

impl ValueObject for MetadataCapabilities {}
