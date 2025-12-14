use crate::domain::biz_metadata::value_object::{
    BizMetadataId, BizMetadataStatus, DataClass, ObjectType, Source,
};

/// 创建命令，封装创建所需字段。
///
/// - `object_type=feature` 时必须提供 `data_class/value_type`
/// - `object_type!=feature` 时 `data_class/value_type/unit` 必须为空
///
/// ```
/// use biz_metadata::{CreateBizMetadataCommand, DataClass, ObjectType};
///
/// let cmd = CreateBizMetadataCommand {
///     code: "company.base.name_cn".into(),
///     name: "公司中文名".into(),
///     description: None,
///     object_type: ObjectType::Feature,
///     parent_id: None,
///     data_class: Some(DataClass::Attribute),
///     value_type: Some("string".into()),
///     unit: None,
///     status: None,
///     source: None,
/// };
/// assert_eq!(cmd.object_type.as_str(), "feature");
/// ```
pub struct CreateBizMetadataCommand {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    /// 语义对象类型（entity/event/relation/document/feature）。
    pub object_type: ObjectType,
    /// 可选父节点 ID。
    pub parent_id: Option<BizMetadataId>,
    /// `object_type=feature` 时填写数据分类。
    pub data_class: Option<DataClass>,
    /// `object_type=feature` 时填写值类型。
    pub value_type: Option<String>,
    pub unit: Option<String>,
    /// 可选的生命周期状态，不传则默认 active。
    pub status: Option<BizMetadataStatus>,
    /// 可选来源，不传则默认 manual。
    pub source: Option<Source>,
}
