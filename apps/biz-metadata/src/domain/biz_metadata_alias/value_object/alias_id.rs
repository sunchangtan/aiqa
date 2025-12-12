use domain_core::prelude::ValueObject;

/// `biz_metadata_alias` 的唯一标识。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BizMetadataAliasId(i64);

impl BizMetadataAliasId {
    /// 以原始 `i64` 构造 ID。
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// 返回底层的 `i64` 值。
    pub const fn value(self) -> i64 {
        self.0
    }
}

impl ValueObject for BizMetadataAliasId {}

impl From<i64> for BizMetadataAliasId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<BizMetadataAliasId> for i64 {
    fn from(value: BizMetadataAliasId) -> Self {
        value.0
    }
}
