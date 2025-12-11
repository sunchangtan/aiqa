use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("biz_metadata"))
                    .comment("统一业务语义元数据定义表")
                    .if_not_exists()
                    .col(big_pk_auto("id").comment("唯一标识，自增主键"))
                    .col(
                        string_len("code", 255)
                            .not_null()
                            .unique_key()
                            .comment("业务编码 (全局唯一)，建议格式：domain.entity.field，如 company.finance.revenue"),
                    )
                    .col(
                        string_len("name", 255)
                            .not_null()
                            .comment("标准业务名称 (中文)，如 \"营业收入\"。对应 NLIR 协议中 name 字段。"),
                    )
                    .col(
                        text("description")
                            .null()
                            .comment("业务含义/口径描述。例如：\"指企业在从事主要业务活动中取得的收入\"。"),
                    )
                    .col(
                        string_len("meta_type", 20)
                            .not_null()
                            .comment("节点类型枚举：entity(实体), event(事件), field(字段), relation(关系)。"),
                    )
                    .col(
                        big_integer("owner_id")
                            .null()
                            .comment("归属父节点 ID，用于强从属关系"),
                    )
                    .col(
                        string_len("data_class", 20)
                            .not_null()
                            .comment("数据核心分类：metric(指标-数值可聚合), dimension(维度-筛选/分组/日期/ID), text(文本-描述/检索), group(分组/容器)。"),
                    )
                    .col(
                        string_len("value_type", 50)
                            .null()
                            .comment("数据类型: string, int, decimal, date, boolean, jsonb"),
                    )
                    .col(
                        string_len("unit", 50)
                            .null()
                            .comment("单位: CNY, USD, %, 次 (仅 metric 有效)"),
                    )
                    .col(
                        boolean("is_identifier")
                            .not_null()
                            .default(false)
                            .comment("是否为唯一标识符，用于唯一确定一个实体。"),
                    )
                    .col(
                        string_len("status", 20)
                            .not_null()
                            .default("active")
                            .comment("生命周期状态: active/deprecated/draft"),
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
                    .name("idx_biz_metadata_owner_id")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("owner_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_biz_metadata_meta_type")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("meta_type"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_biz_metadata_status")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("status"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_biz_metadata_name")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("name"))
                    .to_owned(),
            )
            .await?;

        // 公共更新时间戳函数
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION update_timestamp_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ LANGUAGE 'plpgsql';
                "#,
            )
            .await
            .map(|_| ())?;

        // 针对 biz_metadata 的更新触发器
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_biz_metadata_modtime
                    BEFORE UPDATE ON biz_metadata
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
                DROP TRIGGER IF EXISTS update_biz_metadata_modtime ON biz_metadata;
                DROP FUNCTION IF EXISTS update_timestamp_column;
                "#,
            )
            .await
            .map(|_| ())?;

        manager
            .drop_table(Table::drop().table(Alias::new("biz_metadata")).to_owned())
            .await
    }
}
