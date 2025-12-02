/// SeaORM 模型与领域对象之间的映射接口与实现。
use domain_core::domain_error::DomainError;

/// 持久化层到领域层的通用映射接口。
pub trait EntityMapper<Entity, Domain> {
    /// 将持久化模型转换为领域对象。
    fn map_to_domain(entity: &Entity) -> Result<Domain, DomainError>;
}

/// 领域层到持久化层的通用映射接口。
pub trait ActiveModelMapper<Domain, ActiveModel> {
    /// 将领域对象转换为可持久化的 ActiveModel。
    fn map_to_active_model(domain: &Domain) -> Result<ActiveModel, DomainError>;
}
pub mod metadata_mapping;
pub mod metadata_relation_mapping;
