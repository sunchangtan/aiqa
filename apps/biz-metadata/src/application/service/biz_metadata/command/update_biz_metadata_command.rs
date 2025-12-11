use crate::domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataStatus, BizMetadataType, DataClass,
};

/// 更新命令，包含可选的增量字段。
///
/// # Examples
/// ```
/// use biz_metadata::{FieldUpdate, BizMetadataId, BizMetadataType, DataClass, UpdateBizMetadataCommand};
///
/// let cmd = UpdateBizMetadataCommand {
///     id: BizMetadataId::new(1),
///     name: Some("new name".into()),
///     description: FieldUpdate::Clear,
///     metadata_type: Some(BizMetadataType::Entity),
///     data_class: Some(DataClass::Metric),
///     value_type: Some("string".into()),
///     unit: FieldUpdate::Keep,
///     owner_id: FieldUpdate::Keep,
///     is_identifier: Some(true),
///     status: None,
/// };
/// assert!(matches!(cmd.description, FieldUpdate::Clear));
/// ```
pub struct UpdateBizMetadataCommand {
    pub id: BizMetadataId,
    pub name: Option<String>,
    pub metadata_type: Option<BizMetadataType>,
    pub description: FieldUpdate<String>,
    pub data_class: Option<DataClass>,
    pub value_type: Option<String>,
    pub unit: FieldUpdate<String>,
    pub owner_id: FieldUpdate<BizMetadataId>,
    pub is_identifier: Option<bool>,
    pub status: Option<BizMetadataStatus>,
}

impl Default for UpdateBizMetadataCommand {
    fn default() -> Self {
        Self {
            id: BizMetadataId::new(0),
            name: None,
            metadata_type: None,
            description: FieldUpdate::Keep,
            data_class: None,
            value_type: None,
            unit: FieldUpdate::Keep,
            owner_id: FieldUpdate::Keep,
            is_identifier: None,
            status: None,
        }
    }
}

/// 可选字段更新策略。
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum FieldUpdate<T> {
    /// 保持现状，不修改。
    #[default]
    Keep,
    /// 设置为给定值。
    Set(T),
    /// 清空字段。
    Clear,
}
