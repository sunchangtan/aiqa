use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 说明：
        // - 本迁移为“方案A”：直接定义最终形态的 biz_metadata 表结构。
        // - 对齐 `docs/金融语义字典（biz_metadata）模型与强门禁校验规范_v1.0.md`。
        // - 与 `tools/biz-metadata-linter` 的分工：
        //   - DB 侧：枚举/作用域/unit/identifier/code 格式等可用 CHECK 表达的硬约束；
        //   - Linter 侧：TypeRef 的跨行递归解析（存在性/循环/深度/目标 active）与发布前完整性校验。

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("biz_metadata"))
                    .comment("统一业务语义元数据定义表")
                    .if_not_exists()
                    .col(big_pk_auto("id").comment("唯一标识，自增主键"))
                    .col(
                        string_len("tenant_id", 64)
                            .not_null()
                            .default("default")
                            .comment("多租户隔离字段，当前阶段固定 default，后续可扩展。"),
                    )
                    .col(
                        integer("version")
                            .not_null()
                            .default(1)
                            .comment("版本号/乐观锁，更新/删除必须携带并匹配 version。"),
                    )
                    .col(
                        string_len("code", 255)
                            .not_null()
                            .comment("业务编码 (tenant 内唯一)，建议格式：domain.entity.field，如 company.finance.revenue"),
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
                        string_len("object_type", 16)
                            .not_null()
                            .comment("对象类型：entity/event/relation/document/feature。"),
                    )
                    .col(
                        big_integer("parent_id")
                            .null()
                            .comment("层级父节点 ID（FK -> biz_metadata.id）。"),
                    )
                    .col(
                        string_len("data_class", 16)
                            .null()
                            .comment(
                                "Feature 专属字段：attribute/metric/text/object/array/identifier（非 feature 必须为空）。",
                            ),
                    )
                    .col(
                        string_len("value_type", 64)
                            .null()
                            .comment(
                                "Feature 专属字段：类型表达（标量/Union/object/array/TypeRef）。",
                            ),
                    )
                    .col(
                        string_len("unit", 64)
                            .null()
                            .comment("单位（仅 metric 有业务意义；identifier 必须为空）。"),
                    )
                    .col(
                        string_len("status", 16)
                            .not_null()
                            .default("active")
                            .comment("生命周期状态：active/deprecated"),
                    )
                    .col(
                        string_len("source", 16)
                            .not_null()
                            .default("manual")
                            .comment("来源：manual/auto_mine/api_sync"),
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

        // parent_id -> biz_metadata.id（同表外键）
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_biz_metadata_parent_id")
                    .from(Alias::new("biz_metadata"), Alias::new("parent_id"))
                    .to(Alias::new("biz_metadata"), Alias::new("id"))
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_biz_metadata_tenant_parent_id")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("tenant_id"))
                    .col(Alias::new("parent_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_biz_metadata_tenant_object_type")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("tenant_id"))
                    .col(Alias::new("object_type"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_biz_metadata_tenant_status")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("tenant_id"))
                    .col(Alias::new("status"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_biz_metadata_tenant_name")
                    .table(Alias::new("biz_metadata"))
                    .col(Alias::new("tenant_id"))
                    .col(Alias::new("name"))
                    .to_owned(),
            )
            .await?;

        // PostgreSQL：软删场景下的 tenant 内唯一性（仅对 deleted_at IS NULL 生效）
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE UNIQUE INDEX IF NOT EXISTS ux_biz_metadata_tenant_code_alive
                ON biz_metadata (tenant_id, code)
                WHERE deleted_at IS NULL;
                "#,
            )
            .await
            .map(|_| ())?;

        // PostgreSQL：CHECK 约束（与 biz_metadata_linter 的门禁保持一致）
        // 注：ADD CONSTRAINT 无 IF NOT EXISTS，使用 DO 块避免重复执行时报错。
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_object_type
                    CHECK (object_type IN ('entity','event','relation','document','feature'));
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;

                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_status
                    CHECK (status IN ('active','deprecated'));
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;

                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_source
                    CHECK (source IN ('manual','auto_mine','api_sync'));
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;

                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_code_format
                    CHECK (code ~ '^[a-z][a-z0-9_]*(\\.[a-z][a-z0-9_]*)*$');
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;

                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_scope_feature
                    CHECK (
                        (object_type = 'feature' AND data_class IS NOT NULL AND value_type IS NOT NULL)
                        OR
                        (object_type <> 'feature' AND data_class IS NULL AND value_type IS NULL AND unit IS NULL)
                    );
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;

                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_data_class
                    CHECK (
                        data_class IS NULL
                        OR data_class IN ('attribute','metric','text','object','array','identifier')
                    );
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;

                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_unit_scope
                    CHECK (unit IS NULL OR data_class = 'metric');
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;

                DO $$
                BEGIN
                    ALTER TABLE biz_metadata
                    ADD CONSTRAINT ck_biz_metadata_identifier_rules
                    CHECK (
                        data_class <> 'identifier'
                        OR (unit IS NULL AND value_type IN ('string','int','int|string'))
                    );
                EXCEPTION WHEN duplicate_object THEN
                    NULL;
                END $$;
                "#,
            )
            .await
            .map(|_| ())?;

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
                DROP FUNCTION IF EXISTS update_timestamp_column();
                "#,
            )
            .await
            .map(|_| ())?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                DROP INDEX IF EXISTS ux_biz_metadata_tenant_code_alive;
                "#,
            )
            .await
            .map(|_| ())?;

        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("biz_metadata"))
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
