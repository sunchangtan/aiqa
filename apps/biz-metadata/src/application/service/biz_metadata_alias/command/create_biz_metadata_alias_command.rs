use crate::domain::biz_metadata::value_object::BizMetadataId;
use crate::domain::biz_metadata_alias::value_object::{AliasSource, AliasWeight, LanguageCode};

/// 创建别名命令。
///
/// # Examples
/// ```
/// use biz_metadata::{CreateBizMetadataAliasCommand, BizMetadataId, AliasWeight, LanguageCode};
///
/// # fn main() -> Result<(), domain_core::domain_error::DomainError> {
/// let cmd = CreateBizMetadataAliasCommand {
///     metadata_id: BizMetadataId::new(1),
///     alias: "营收".into(),
///     source: None,
///     weight: Some(AliasWeight::new(10)?),
///     is_primary: Some(true),
///     language: Some(LanguageCode::new("zh-CN")?),
/// };
/// assert_eq!(cmd.metadata_id.value(), 1);
/// # Ok(()) }
/// ```
pub struct CreateBizMetadataAliasCommand {
    pub metadata_id: BizMetadataId,
    pub alias: String,
    /// 别名来源，未指定时使用默认 manual。
    pub source: Option<AliasSource>,
    /// 匹配权重，可选。
    pub weight: Option<AliasWeight>,
    /// 是否首选别名，可选。
    pub is_primary: Option<bool>,
    /// 语言编码，未指定时使用默认 zh-CN。
    pub language: Option<LanguageCode>,
}
