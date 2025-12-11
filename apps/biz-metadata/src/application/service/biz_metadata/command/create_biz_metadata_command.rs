use crate::domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataStatus, BizMetadataType, DataClass,
};

/// 创建命令，封装创建所需字段。
///
/// # Examples
/// ```
/// use biz_metadata::{CreateBizMetadataCommand, BizMetadataType, DataClass};
///
/// let cmd = CreateBizMetadataCommand {
///     code: "user".into(),
///     name: "用户".into(),
///     description: None,
///     metadata_type: BizMetadataType::Attribute,
///     data_class: DataClass::Dimension,
///     value_type: "string".into(),
///     owner_id: None,
///     unit: None,
///     is_identifier: false,
///     status: None,
/// };
/// assert_eq!(cmd.code, "user");
/// ```
pub struct CreateBizMetadataCommand {
    pub code: String,
    pub name: String,
    pub metadata_type: BizMetadataType,
    pub description: Option<String>,
    pub data_class: DataClass,
    pub value_type: String,
    pub owner_id: Option<BizMetadataId>,
    pub unit: Option<String>,
    pub is_identifier: bool,
    /// 可选的生命周期状态，不传则默认 active。
    pub status: Option<BizMetadataStatus>,
}
