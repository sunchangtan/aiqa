use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum MetadataType {
    #[sea_orm(iden = "metadata_type")]
    Enum,
    #[sea_orm(iden = "attribute")]
    Attribute,
    #[sea_orm(iden = "entity")]
    Entity,
    #[sea_orm(iden = "event")]
    Event,
}
