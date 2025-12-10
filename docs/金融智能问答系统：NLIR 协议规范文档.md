# 金融智能问答系统：NLIR 协议规范文档 (V1.0)

> **版本**: V1.0
> **最后更新**: 2025-12-10
> **适用模块**: 意图识别 (Intent Router) → 语义链接 (Schema Linker)
> **核心理念**: 语义层与物理层解耦。NLIR 仅描述“业务意图”与“逻辑流转”，不包含物理表名或 SQL 语法。

---

## 1. 协议整体结构 (JSON Schema)

```json
{
  "rewritten_content": "用户查询的规范化重写（补全主语、去除废话）",
  "ir": {
    "natural_ir": {
      "steps": [
        {
          "id": "step1",
          "action": "query",
          "outputs": [
            {
              "name": "营业收入",
              "node_type": "field",
              "key": "revenue_val"
            }
          ],
          "filters": [
            {
              "name": "年份",
              "node_type": "field",
              "op": "eq",
              "value": "2024"
            }
          ],
          "sort": { "name": "...", "node_type": "field", "order": "desc" },
          "limit": 10
        }
      ],
      "final_step_id": "step1"
    }
  }
}
```

---

## 2. 字段详细说明

### 2.1 根节点 (Root)

| 字段名             | 类型   | 必填 | 说明                                                                 |
| ------------------ | ------ | ---- | -------------------------------------------------------------------- |
| `rewritten_content` | String | 是   | 经过 NLP 预处理后的用户查询文本，用于审计、调试及回答生成。          |
| `ir`               | Object | 是   | 中间表示 (IR) 的容器对象。                                          |
| `ir.natural_ir`    | Object | 是   | 自然语言 IR 的核心逻辑体。                                          |
| `final_step_id`    | String | 是   | 最终结果由哪一步产出（通常是最后一个 step 的 ID）。                 |

---

### 2.2 Step 对象 (步骤)

描述一个独立的原子查询操作。

> 核心约束：同一个 Step 内的所有 `outputs` 必须共享相同的 `filters` 上下文。

| 字段名   | 类型   | 必填 | 说明                                                                                 |
| -------- | ------ | ---- | ------------------------------------------------------------------------------------ |
| `id`     | String | 是   | 步骤唯一标识，如 `step1`, `step2`。                                                 |
| `action` | String | 是   | 动作类型。枚举：`query`(查询), `count`(计数), `rank`(排序), `compare`(对比)。        |
| `outputs` | Array | 是   | 本步骤的产出物列表。支持一次查询返回多个指标（如同时查营收和利润）。                |
| `filters` | Array | 否   | 筛选条件列表。                                                                       |
| `sort`   | Object | 否   | 排序规则，结构：`{ "name": "...", "node_type": "field", "order": "asc/desc" }`。    |
| `limit`  | Integer| 否   | 限制返回条数。                                                                       |

---

### 2.3 Output 对象 (产出定义)

定义本步骤“要查什么业务对象”，以及“查出来存为什么变量”。

| 字段名  | 类型   | 必填 | 说明                                                         | 对应数据库字段              |
| ------- | ------ | ---- | ------------------------------------------------------------ | --------------------------- |
| `name`  | String | 是   | 标准业务名称。如“营业收入”、“供应商关系”。后端将据此在元数据表中查找定义。 | `biz_metadata.name`         |
| `node_type` | String | 是 | 节点元类型。枚举见下方 [3.1 节](#31-node_type-元数据类型)。 | `biz_metadata.node_type`    |
| `key`   | String | 是   | 变量名/句柄。供后续 Step 的 dependency 引用。建议命名：`revenue_val`, `supplier_list`。 | -                           |

---

### 2.4 Filter 对象 (筛选条件)

定义本步骤的过滤逻辑（静态值过滤或动态依赖过滤）。

| 字段名  | 类型   | 必填 | 说明                                                                 | 对应数据库字段           |
| ------- | ------ | ---- | -------------------------------------------------------------------- | ------------------------ |
| `name`  | String | 是   | 标准业务名称。如“发生日期”、“所属行业”。                             | `biz_metadata.name`      |
| `node_type` | String | 是 | 通常为 `field` 或 `entity`。                                       | `biz_metadata.node_type` |
| `op`    | String | 是   | 操作符。枚举：`eq`(=), `neq`(!=), `gt`(>), `lt`(<), `gte`(>=), `lte`(<=), `in`, `like`, `between`。 | - |
| `value` | Any    | 否   | 静态筛选值。如 `"2024"`, `["上海", "北京"]`。与 `dependency` 二选一。 | - |
| `dependency` | Object | 否 | 动态依赖引用。用于多步推理。与 `value` 二选一。                      | - |

---

### 2.5 Dependency 对象 (依赖引用)

用于将前序步骤的输出结果，作为当前步骤的过滤条件。

| 字段名     | 类型   | 必填 | 说明                                           |
| ---------- | ------ | ---- | ---------------------------------------------- |
| `step_id`  | String | 是   | 依赖的前序步骤 ID。                           |
| `output_key` | String | 是 | 引用前序步骤 `outputs` 数组中定义的具体 `key`。 |

---

## 3. 标准枚举值定义

### 3.1 `node_type` (元数据类型)

严格对应数据库 `biz_metadata.node_type` 字段。

- `entity`：客观主体。例如：公司、自然人、基金、债券。
- `field`：数值指标或静态属性。例如：营业收入、收盘价、注册城市、成立日期。
- `event`：动态事实/事件。例如：违约、诉讼、IPO、高管变动。
- `relation`：图谱关系类型。例如：供应商关系、担保关系、子公司关系。

### 3.2 `action` (动作类型)

- `query`：基础检索（默认）。适用于查数、查图谱。
- `aggregate`：聚合计算。需配合后端逻辑（如 Sum, Avg），通常由 `op` 或特定指标属性触发。
- `rank`：排序并取 Top N。
- `compare`：两个实体间的对比（如：对比 A 公司和 B 公司的营收）。

> 说明：在 Step 对象定义中也可使用 `count` 作为计数语义，视后端实现进行统一映射（如映射为特定 `aggregate` 类型）。

---

## 4. 典型场景示例

### 4.1 场景 A：单步多指标查询 (Arrays in Outputs)

**用户问题**: “查一下宁德时代 2024 年的营业收入和净利润。”

```json
{
  "steps": [
    {
      "id": "step1",
      "action": "query",
      "outputs": [
        {
          "name": "营业收入",
          "node_type": "field",
          "key": "revenue_val"
        },
        {
          "name": "净利润",
          "node_type": "field",
          "key": "net_profit_val"
        }
      ],
      "filters": [
        {
          "name": "公司名称",
          "node_type": "field",
          "op": "eq",
          "value": "宁德时代"
        },
        {
          "name": "报告期",
          "node_type": "field",
          "op": "eq",
          "value": "2024"
        }
      ]
    }
  ],
  "final_step_id": "step1"
}
```

---

### 4.2 场景 B：多步依赖推理 (Dependency Injection)

**用户问题**: “查询宁德时代的供应商中，有哪些发生过环保处罚？”

```json
{
  "steps": [
    {
      "id": "step1",
      "action": "query",
      "outputs": [
        {
          "name": "供应商关系",
          "node_type": "relation",
          "key": "supplier_list"
        }
      ],
      "filters": [
        {
          "name": "公司名称",
          "node_type": "field",
          "op": "eq",
          "value": "宁德时代"
        }
      ]
    },
    {
      "id": "step2",
      "action": "query",
      "outputs": [
        {
          "name": "环保处罚",
          "node_type": "event",
          "key": "penalty_events"
        }
      ],
      "filters": [
        {
          "name": "公司ID",
          "node_type": "field",
          "op": "in",
          "dependency": {
            "step_id": "step1",
            "output_key": "supplier_list"
          }
        }
      ]
    }
  ],
  "final_step_id": "step2"
}
```
