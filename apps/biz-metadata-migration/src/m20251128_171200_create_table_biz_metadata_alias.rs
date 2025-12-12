use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("biz_metadata_alias"))
                    .comment("业务元数据别名/同义词表 (用于 NLIR 解析)")
                    .if_not_exists()
                    .col(big_pk_auto("id").comment("自增主键"))
                    .col(
                        big_integer("metadata_id")
                            .not_null()
                            .comment("关联的标准元数据 ID"),
                    )
                    .col(
                        string_len("alias", 255)
                            .not_null()
                            .comment("自然语言别名。如标准名 \"营业收入\"，此处可存 \"营收\", \"销售额\", \"Top Line\" 等。"),
                    )
                    .col(
                        string_len("source", 64)
                            .not_null()
                            .default("manual")
                            .comment("别名来源：manual/auto_mine/log/embedding"),
                    )
                    .col(
                        integer("weight")
                            .not_null()
                            .default(0)
                            .comment("匹配权重 (0-100)，数值越高优先级越高"),
                    )
                    .col(
                        boolean("is_primary")
                            .not_null()
                            .default(false)
                            .comment("是否为首选别名 (用于生成回答时指代该指标)"),
                    )
                    .col(
                        string_len("language", 16)
                            .not_null()
                            .default("zh-CN")
                            .comment("语言编码，例如 zh-CN"),
                    )
                    .col(
                        timestamp_with_time_zone("created_at")
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp_with_time_zone("updated_at")
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .col(
                        timestamp_with_time_zone("deleted_at")
                            .null()
                            .comment("删除时间(软删)"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_metadata_alias_mid")
                    .table(Alias::new("biz_metadata_alias"))
                    .col(Alias::new("metadata_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_metadata_alias_alias")
                    .table(Alias::new("biz_metadata_alias"))
                    .col(Alias::new("alias"))
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_biz_metadata_alias_modtime
                    BEFORE UPDATE ON biz_metadata_alias
                    FOR EACH ROW EXECUTE FUNCTION update_timestamp_column();
                "#,
            )
            .await
            .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DROP TRIGGER IF EXISTS update_biz_metadata_alias_modtime ON biz_metadata_alias;
                "#,
            )
            .await
            .map(|_| ())?;

        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("biz_metadata_alias"))
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
