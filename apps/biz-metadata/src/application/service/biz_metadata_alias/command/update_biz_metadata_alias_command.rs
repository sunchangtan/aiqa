use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::domain::biz_metadata_alias::value_object::{
    AliasSource, AliasWeight, BizMetadataAliasId, LanguageCode,
};

/// 可选字段更新策略。
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum AliasFieldUpdate<T> {
    /// 保持现状，不修改。
    #[default]
    Keep,
    /// 设置为给定值。
    Set(T),
    /// 清空字段。
    Clear,
}

/// 更新别名命令。
pub struct UpdateBizMetadataAliasCommand {
    pub id: BizMetadataAliasId,
    pub metadata_id: Option<BizMetadataId>,
    pub alias: AliasFieldUpdate<String>,
    pub source: Option<AliasSource>,
    pub weight: Option<AliasWeight>,
    pub is_primary: Option<bool>,
    pub language: Option<LanguageCode>,
}

impl Default for UpdateBizMetadataAliasCommand {
    fn default() -> Self {
        Self {
            id: BizMetadataAliasId::new(0),
            metadata_id: None,
            alias: AliasFieldUpdate::Keep,
            source: None,
            weight: None,
            is_primary: None,
            language: None,
        }
    }
}
