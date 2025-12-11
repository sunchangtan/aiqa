use domain_core::prelude::{DomainError, ValueObject};

/// 元数据的类别定义。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BizMetadataType {
    /// 属性型元数据：描述实体上的单个字段或度量。
    Attribute,
    /// 实体型元数据：表示业务实体本身，可被关联引用。
    Entity,
    /// 事件型元数据：用于记录行为或日志事件。
    Event,
}

impl ValueObject for BizMetadataType {
    fn validate(&self) -> Result<(), DomainError> {
        // 当前没有复杂校验逻辑，如后续需要可在此扩展。
        Ok(())
    }
}
