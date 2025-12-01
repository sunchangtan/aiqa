use domain_core::prelude::ValueObject;

/// 元数据关系的唯一标识。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetadataRelationId(i64);

impl MetadataRelationId {
    /// 通过 `i64` 创建新的标识。
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// 以原始 `i64` 形式取回标识。
    pub const fn value(self) -> i64 {
        self.0
    }
}

impl ValueObject for MetadataRelationId {}

impl From<i64> for MetadataRelationId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<MetadataRelationId> for i64 {
    fn from(value: MetadataRelationId) -> Self {
        value.0
    }
}
