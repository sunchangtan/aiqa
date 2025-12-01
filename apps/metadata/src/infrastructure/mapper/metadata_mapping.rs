use chrono::{FixedOffset, Utc};
use sea_orm::ActiveValue::Set;

use crate::domain::metadata::Metadata;
use crate::domain::metadata::value_object::{
    MetadataCapabilities, MetadataCode, MetadataId, MetadataName, MetadataType,
    ValueType as DomainValueType,
};
use crate::infrastructure::persistence::entity::{metadata, sea_orm_active_enums};

fn to_domain_metadata_type(db_ty: sea_orm_active_enums::MetadataType) -> MetadataType {
    match db_ty {
        sea_orm_active_enums::MetadataType::Attribute => MetadataType::Attribute,
        sea_orm_active_enums::MetadataType::Entity => MetadataType::Entity,
        sea_orm_active_enums::MetadataType::Event => MetadataType::Event,
    }
}

fn to_db_metadata_type(dom_ty: MetadataType) -> sea_orm_active_enums::MetadataType {
    match dom_ty {
        MetadataType::Attribute => sea_orm_active_enums::MetadataType::Attribute,
        MetadataType::Entity => sea_orm_active_enums::MetadataType::Entity,
        MetadataType::Event => sea_orm_active_enums::MetadataType::Event,
    }
}

pub fn from_entity(model: &metadata::Model) -> Metadata {
    // 基础字段
    let id = MetadataId::from(model.id);
    let code = MetadataCode::new(model.code.clone()).expect("invalid code from persistence");
    let name = MetadataName::new(model.name.clone()).expect("invalid name from persistence");
    let metadata_type = to_domain_metadata_type(model.metadata_type.clone());
    let value_type = DomainValueType::new(model.value_type.clone())
        .expect("invalid value_type from persistence");

    // 使用领域构造函数创建聚合（时间戳将以当前时间初始化）。
    let mut agg = Metadata::new(
        id,
        code.into_inner(),
        name.into_inner(),
        metadata_type,
        value_type.into_inner(),
    )
    .expect("failed to construct domain metadata from persistence");

    // 能力位
    let caps = MetadataCapabilities::new(
        model.is_chainable,
        model.is_filterable,
        model.is_sortable,
        model.is_groupable,
        model.is_relation_derived,
    );
    agg.set_capabilities(caps)
        .expect("set capabilities should not fail");

    // 扩展 JSON
    let extra = model.extra.clone();
    agg.set_extra(extra).expect("set extra should not fail");

    // 注意：由于领域对象当前不支持回写较早的时间戳，这里不回放 created_at/updated_at/delete_at。
    // 回放 updated_at 和 delete_at：尽量与持久化对齐。
    let persisted_updated = model.updated_at.with_timezone(&Utc);
    let _ = agg.touch(persisted_updated);
    if let Some(del) = model.delete_at {
        let _ = agg.mark_deleted(del.with_timezone(&Utc));
    }

    // 如需严格对齐 created_at，也可在领域层提供 from_persistence 构造函数后再完善。

    agg
}

pub fn to_active_model(user: &Metadata) -> metadata::ActiveModel {
    let tz = FixedOffset::east_opt(0).expect("UTC offset");

    metadata::ActiveModel {
        id: Set(i64::from(user.id())),
        code: Set(user.code().as_str().to_string()),
        name: Set(user.name().as_str().to_string()),
        metadata_type: Set(to_db_metadata_type(user.metadata_type())),
        value_type: Set(user.value_type().as_str().to_string()),
        is_chainable: Set(user.capabilities().chainable()),
        is_filterable: Set(user.capabilities().filterable()),
        is_sortable: Set(user.capabilities().sortable()),
        is_groupable: Set(user.capabilities().groupable()),
        is_relation_derived: Set(user.capabilities().relation_derived()),
        extra: Set(user.extra().cloned()),
        created_at: Set(user.created_at().with_timezone(&tz)),
        updated_at: Set(user.updated_at().with_timezone(&tz)),
        delete_at: Set(user.delete_at().map(|d| d.with_timezone(&tz))),
    }
}
