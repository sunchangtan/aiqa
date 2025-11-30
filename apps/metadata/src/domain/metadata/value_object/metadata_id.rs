use domain_core::prelude::ValueObject;

/// 聚合根 ID 的新类型，避免“原始类型痴迷”。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MetadataId(i64);

impl MetadataId {
    /// 通过 `i64` 创建新的标识。
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// 以原始 `i64` 形式取回标识。
    pub const fn value(self) -> i64 {
        self.0
    }
}

impl ValueObject for MetadataId {}

impl From<i64> for MetadataId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<MetadataId> for i64 {
    fn from(value: MetadataId) -> Self {
        value.0
    }
}
