//! `biz_metadata_alias` 领域使用的值对象集合，负责输入校验与语义约束。

mod alias_id;
mod alias_source;
mod alias_text;
mod alias_weight;
mod language_code;

pub use alias_id::BizMetadataAliasId;
pub use alias_source::AliasSource;
pub use alias_text::AliasText;
pub use alias_weight::AliasWeight;
pub use language_code::LanguageCode;
