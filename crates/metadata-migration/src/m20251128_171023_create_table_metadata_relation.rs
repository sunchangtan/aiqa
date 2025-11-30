use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("metadata_relation")
                    .if_not_exists()
                    .col(big_pk_auto("id").comment("关系 ID，自增主键"))
                    .col(
                        big_integer("from_metadata_id")
                            .not_null()
                            .comment("起点元数据 ID，引用 metadata(id)"),
                    )
                    .col(
                        big_integer("to_metadata_id")
                            .not_null()
                            .comment("终点元数据 ID，引用 metadata(id)"),
                    )
                    .col(
                        timestamp_with_time_zone("created_at")
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp_with_time_zone("updated_at")
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .col(
                        timestamp_with_time_zone("delete_at")
                            .null()
                            .comment("删除时间(软删)"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_metadata_relation_from")
                            .from("metadata_relation", "from_metadata_id")
                            .to("metadata", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_metadata_relation_to")
                            .from("metadata_relation", "to_metadata_id")
                            .to("metadata", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("metadata_relation").to_owned())
            .await
    }
}
