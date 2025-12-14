use crate::domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataStatus, DataClass, Source, Version,
};

/// 更新命令，包含可选的增量字段。
///
/// 更新必须携带 `version`，用于乐观锁校验。
///
/// ```
/// use biz_metadata::{FieldUpdate, BizMetadataId, DataClass, UpdateBizMetadataCommand, Version};
///
/// let cmd = UpdateBizMetadataCommand {
///     id: BizMetadataId::new(1),
///     version: Version::new(1).unwrap(),
///     name: Some("new name".into()),
///     description: FieldUpdate::Clear,
///     parent_id: FieldUpdate::Keep,
///     data_class: Some(DataClass::Metric),
///     value_type: Some("int".into()),
///     unit: FieldUpdate::Set("CNY".into()),
///     status: None,
///     source: None,
/// };
/// assert!(matches!(cmd.description, FieldUpdate::Clear));
/// ```
pub struct UpdateBizMetadataCommand {
    pub id: BizMetadataId,
    pub version: Version,
    pub name: Option<String>,
    pub description: FieldUpdate<String>,
    pub data_class: Option<DataClass>,
    pub value_type: Option<String>,
    pub unit: FieldUpdate<String>,
    pub parent_id: FieldUpdate<BizMetadataId>,
    pub status: Option<BizMetadataStatus>,
    pub source: Option<Source>,
}

impl Default for UpdateBizMetadataCommand {
    fn default() -> Self {
        Self {
            id: BizMetadataId::new(0),
            version: Version::new(1).expect("default version"),
            name: None,
            description: FieldUpdate::Keep,
            data_class: None,
            value_type: None,
            unit: FieldUpdate::Keep,
            parent_id: FieldUpdate::Keep,
            status: None,
            source: None,
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
