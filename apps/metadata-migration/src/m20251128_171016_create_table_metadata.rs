use sea_orm_migration::{
    prelude::{sea_query::extension::postgres::Type, *},
    schema::*,
};

use crate::metadata_type::MetadataType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(MetadataType::Enum)
                    .values([
                        MetadataType::Attribute,
                        MetadataType::Entity,
                        MetadataType::Event,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("metadata")
                    .comment("元数据表")
                    .if_not_exists()
                    .col(
                        big_pk_auto("id").comment("元数据节点 ID，自增主键"),
                    )
                    .col(
                        string_len("code", 128)
                            .not_null()
                            .unique_key()
                            .comment("元数据编码，供 Planner/配置/接口映射作稳定标识"),
                    )
                    .col(
                        string_len("name", 256)
                            .not_null()
                            .comment("元数据名称/展示名称"),
                    )
                    .col(
                        ColumnDef::new(Alias::new("metadata_type"))
                            .enumeration(
                                MetadataType::Enum,
                                [
                                    MetadataType::Attribute,
                                    MetadataType::Entity,
                                    MetadataType::Event,
                                ],
                            )
                            .not_null()
                            .comment(
                                "元数据类别：attribute/entity/event",
                            ),
                    )
                    .col(
                        string_len("value_type", 256)
                            .not_null()
                            .comment(
                                "属性值类型: 基础 int/decimal/string/enum/boolean/date/list; 自定义可引用其他元数据类型(如 list<int>/object); 联合类型示例: person | company",
                            ),
                    )
                    .col(
                        boolean("is_chainable")
                            .default(false)
                            .comment(
                                "是否为可链式的实体/实体列表属性，仅 value_type = 'entity' 时有效",
                            ),
                    )
                    .col(
                        boolean("is_filterable")
                            .default(false)
                            .comment("是否允许作为筛选条件(WHERE/FILTER)"),
                    )
                    .col(
                        boolean("is_sortable")
                            .default(false)
                            .comment("是否允许作为排序字段(ORDER BY)"),
                    )
                    .col(
                        boolean("is_groupable")
                            .default(false)
                            .comment("是否允许作为分组字段(GROUP BY)"),
                    )
                    .col(
                        boolean("is_relation_derived")
                            .default(false)
                            .comment("是否为关系衍生指标"),
                    )
                    .col(
                        json_binary("extra")
                            .null()
                            .comment(
                                "预留扩展信息：物理表/字段映射、取值约束、实体型属性限定等",
                            ),
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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("metadata").to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(MetadataType::Enum).to_owned())
            .await
    }
}
