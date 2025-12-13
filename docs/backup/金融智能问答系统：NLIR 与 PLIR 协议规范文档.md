# 金融智能问答系统：NLIR 与 PLIR 协议规范文档 (V14.0)

**版本**: V14.0 (Metadata Code Enforced)
**日期**: 2025-12-10
**核心修复**: PLIR 中显式使用 **`code`** 字段，替代之前的简写 `meta`，确保与数据库 `biz_metadata.code` 字段严格对应。

---

## 1. 协议核心定义

### 1.1 映射流转 (Mapping Flow)

* **NLIR (语义层)**: 持有 **`name`** (中文业务名)。
  * *作用*: 作为向量检索的 Query。
* **PLIR (物理层)**: 持有 **`code`** (系统唯一编码)。
  * *作用*: 链接到 `biz_metadata` 表，确立数据的精确类型与计算逻辑。

### 1.2 统一 Slot 对象 (Unified Slot)

输入 (`args`) 和输出 (`outputs`) 均使用统一结构。

| 字段名 | 类型 | NLIR (语义层) | PLIR (物理层) | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| **key** | String | 必填 | 必填 | **槽位/参数名** (英文 Snake Case)。 |
| **name** | String | **必填** | 选填 | **中文名称** (如 "营业收入")。NLIR 的核心。 |
| **value** | Any | 必填 (Arg) | 必填 (Arg) | **值**。NLIR 存自然语言，PLIR 存物理 ID/Code。 |
| **code**| String | - | **必填** | **元数据编码** (如 `company.finance.revenue`)。PLIR 的核心。 |
| **extractor**| String| - | 必填 (Output)| **取值规则** (如 `$.data.val`, `column_name`)。 |
| **type** | Enum | 必填 | 选填 | 数据类型 (`entity`, `field`)。 |

---

## 2. 完整 JSON 结构 (Schema)

```json
{
  "session_id": "sess_001",
  "turns": [
    {
      "turn_id": "turn_01",
      "user_query": "查一下茅台的营收",
      "nlir": { "nodes": [...] },
      "plir": { "nodes": [...] }
    }
  ]
}
```

---

## 3. 完整场景演练 (Trace)

**场景**:

1. **Turn 1**: "查一下**茅台**的**营收**。"
2. **Turn 2**: "那**五粮液**呢？"

### 3.1 第一轮 (Turn 01)

```json
{
  "turn_id": "turn_01",
  "user_query": "查一下茅台的营收",

  // ==========================================
  // NLIR: 只有 Name (自然语言)
  // ==========================================
  "nlir": {
    "nodes": [
      {
        "id": "node_1",
        "action": "query",

        // 1. Outputs
        "outputs": [
          {
            "key": "var_rev",
            "name": "营业收入",       // [语义锚点] -> 等待映射
            "type": "field"
          }
        ],

        // 2. Args
        "args": [
          {
            "key": "entity",
            "name": "公司名称",       // [语义锚点]
            "value": "贵州茅台",      // [原始值]
            "type": "entity"
          },
          {
            "key": "period",
            "name": "报告期",
            "value": "最新",
            "type": "field"
          }
        ]
      }
    ]
  },

  // ==========================================
  // PLIR: 拥有 Metadata Code (映射完成)
  // ==========================================
  "plir": {
    "nodes": [
      {
        "id": "node_1",
        "action": "api_get_financial",

        // 1. Outputs
        "outputs": [
          {
            "key": "var_rev",
            "name": "营业收入",
            // [映射结果] 对应 biz_metadata.code
            "code": "company.finance.revenue",
            "extractor": "$.data.value"
          }
        ],

        // 2. Args
        "args": [
          {
            "key": "ticker",            // 物理参数名
            "name": "公司名称",
            "value": "600519.SH",       // 物理值
            // [映射结果] 参数定义的元数据
            "code": "company.basic.ticker"
          },
          {
            "key": "period",
            "name": "报告期",
            "value": "2024Q3",
            "code": "common.date.report_period"
          },
          // 隐含参数 (Template 补全)
          {
            "key": "metric",
            "name": "指标类型",
            "value": "total_revenue",
            "code": "company.finance.revenue"
          }
        ]
      }
    ]
  }
}
```

### 3.2 第二轮 (Turn 02): 复用

```json
{
  "turn_id": "turn_02",
  "user_query": "那五粮液呢？",

  "nlir": {
    "nodes": [
      {
        "id": "node_2a",
        "action": "query",
        "ref_id": "turn_01.node_1", // 语义复用

        "outputs": [
          { "key": "var_rev_wu", "name": "营业收入", "type": "field" }
        ],
        "args": [
          {
            "key": "entity",
            "name": "公司名称",
            "value": "五粮液",
            "type": "entity"
          },
          {
            "key": "period",
            "name": "报告期",
            "value": "最新",
            "type": "field"
          }
        ]
      }
    ]
  },

  "plir": {
    "nodes": [
      {
        "id": "node_2a",
        "action": "api_get_financial",
        "ref_id": "turn_01.node_1", // 物理血缘

        "outputs": [
          {
            "key": "var_rev_wu",
            "name": "营业收入",
            "code": "company.finance.revenue",
            "extractor": "$.data.value"
          }
        ],
        "args": [
          {
            "key": "ticker",
            "name": "公司名称",
            "value": "000858.SZ",
            "code": "company.basic.ticker"
          },
          {
            "key": "period",
            "name": "报告期",
            "value": "2024Q3",
            "code": "common.date.report_period"
          },
          {
            "key": "metric",
            "name": "指标类型",
            "value": "total_revenue",
            "code": "company.finance.revenue"
          }
        ]
      }
    ]
  }
}
```
