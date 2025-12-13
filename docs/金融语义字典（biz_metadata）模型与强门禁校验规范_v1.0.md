# 金融语义字典（biz_metadata）模型与强门禁校验规范（v1.0）

---

## A. biz_metadata 表结构（字段字典 / 对象字典定义表）

> **定位**
>
> - 本表仅存放**定义（metadata）**，不存放任何业务事实数据。
> - 作为金融 AI Agent、数据湖 / 数仓 / 特征平台的**统一语义字典与治理基石**。
> - 设计原则对齐 Bloomberg / LSEG 等行业标杆：字段定义（Feature Dictionary）与对象语义分离、强约束、可演进。

---

## 一、核心设计原则（定稿）

1. **五类核心对象**
   - `entity | event | relation | document | feature`
2. **data_class / value_type 只服务于 feature**
   - 当 `object_type != 'feature'`：
     **`data_class / value_type / unit` 必须为空**
   - 当 `object_type = 'feature'`：
     **`data_class / value_type` 必须非空**
3. **unit 使用规则（补充定稿）**
   - `unit` **仅对 `metric` 有业务意义**
   - 对 `identifier / text / object / array`：通常为空
   - 对 `identifier`：**必须为空**
4. **结构统一表达**
   - object：`json<object:S>`（S 为 schema_ref / 命名空间）
   - array：`json<array:T>`（T 为元素类型）
5. **value_type 支持 Union（联合类型）**
   - 标量联合：`int|string`
   - 实体联合：`person|company`
   - 数组联合：`json<array:company|person>`
6. **value_type 支持 TypeRef（类型引用）**
   - 语法：`ref:<code>`
   - 用途：复用已定义字段的类型定义（例如 `ref:person.base.name`）


---

## 二、表名

- **biz_metadata**（单数表名）

---

## 三、字段清单（全量）

### 3.1 主键、版本与多租户并发保护

| 字段      | 类型        | 允许空 | 说明                                                                                   |
| --------- | ----------- | -----: | -------------------------------------------------------------------------------------- |
| id        | bigint      |     否 | 内部主键，稳定锚点，用于层级/外键/迁移。                                               |
| tenant_id | varchar(64) |     否 | **多租户隔离字段**。所有唯一性与写入需带 tenant_id。                                   |
| version   | int         |     否 | **版本号 / 乐观锁**。更新时必须 `WHERE id + tenant_id + version`，成功后 `version+1`。 |

---

### 3.2 语义标识与描述

| 字段        | 类型         | 允许空 | 说明                                                           |
| ----------- | ------------ | -----: | -------------------------------------------------------------- |
| code        | varchar(255) |     否 | 语义路径（如 `company.base.name_cn`），全局唯一（tenant 内）。 |
| name        | varchar(255) |     否 | 中文名称（展示用）。                                           |
| description | text         |     是 | 业务口径说明（来源、边界、周期、替代字段等）。                 |

---

### 3.3 对象分类与层级

| 字段        | 类型   | 允许空 | 说明                                                         |
| ----------- | ------ | -----: | ------------------------------------------------------------ |
| object_type | enum   |     否 | `entity / event / relation / document / feature`             |
| parent_id   | bigint |     是 | 层级父节点（FK → biz_metadata.id），用于字段组与子字段组织。 |

---

### 3.4 Feature 专属字段（仅当 object_type=feature）

| 字段       | 类型        |                    允许空 | 说明                                                      |
| ---------- | ----------- | ------------------------: | --------------------------------------------------------- |
| data_class | enum        | 是（非 feature 必须为空） | `attribute / metric / text / object / array / identifier` |
| value_type | varchar(64) | 是（非 feature 必须为空） | 类型表达，见第 4 节                                       |
| unit       | varchar(64) |                        是 | 单位（**仅 metric 有意义**；identifier 必须为空）         |

---

### 3.5 状态、来源与审计

| 字段       | 类型        | 允许空 | 说明                            |
| ---------- | ----------- | -----: | ------------------------------- |
| status     | enum        |     否 | `active / deprecated`           |
| source     | enum        |     否 | `manual / auto_mine / api_sync` |
| created_at | timestamptz |     否 | 创建时间                        |
| updated_at | timestamptz |     否 | 更新时间                        |
| deleted_at | timestamptz |     是 | 软删时间                        |

---

## 四、data_class 定义（定稿）

| data_class | 含义                             |
| ---------- | -------------------------------- |
| attribute  | 普通属性 / 维度字段              |
| metric     | 指标 / KPI（需口径与周期治理）   |
| text       | 长文本（检索 / RAG）             |
| object     | 对象结构 / 字段组                |
| array      | 列表 / 多值结构                  |
| identifier | 标识符字段（替代 is_identifier） |

---

## 五、value_type 规范（定稿）

### 5.1 标量类型

- `string`
- `int`
- `decimal`
- `boolean`
- `date`
- `datetime`

### 5.2 Union（联合类型）支持

- **标量联合**：`int|string`
- **实体联合**：`person|company`
- **数组联合**：`json<array:company|person>`

> Union 使用 `|`，不加空格；顺序建议固定（如 `company|person`）。

---

### 5.3 object（对象结构）

- 统一语法：`json<object:S>`
- `S` 为 **schema_ref / 命名空间**（不是递归类型）

示例：

- `company.base` → `data_class=object`，`value_type=json<object:company.base>`

---

### 5.4 array（列表结构）

- 统一语法：`json<array:T>`
- 物理载体默认 jsonb

#### T 的含义

- 标量：`string / int / decimal / ...`
- 实体引用：`company / person / bond / ...`
- 嵌套对象：`object`

#### 关键规则

- `json<array:company>`：**Entity 引用数组**（不是嵌套 company 对象）
- `json<array:object>`：嵌套对象数组，**必须**定义 `xxx.item.*` 子字段体系
- 不建议混用：`company|object`（落库与治理复杂）

---

### 5.5 identifier（标识符）强约束

当 `data_class = identifier`：

1. `value_type` 仅允许：
   - `string`
   - `int`
   - `int|string`
2. `unit` **必须为空**
3. 标识类型通过 **code 命名约定**表达（无 tags、无额外字段）：

#### 命名规范

- `... .id.<id_type>`

示例：

- `company.base.id.uscc`
- `company.base.id.reg_no`
- `bond.id.isin`
- `equity.id.ticker`

### 5.6 TypeRef（类型引用）

- 统一语法：`ref:<code>`
- `<code>` 必须是同一 tenant 下已存在的 `biz_metadata.code`，且其 `object_type=feature`、`status=active`。
- Import Gate / Publish Gate 需要对 `ref:` 做解析（可递归展开），并在**展开后的最终类型**上继续执行既有门禁（identifier/unit/object/array 完整性等）。
- 禁止循环引用；建议最大展开深度 5（超限视为配置错误）。

示例：

- `company.base.legal_rep_name` → `data_class=attribute`，`value_type=ref:person.base.name`

---


## 六、关键治理规则（必须写进门禁）

1. **非 feature 禁止填类型**
   - `object_type != feature` → `data_class / value_type / unit` 必须为空
2. **feature 必填类型**
   - `object_type = feature` → `data_class / value_type` 必须非空
3. **object 完整性**
   - `data_class=object` 且 `value_type=json<object:S>` → 必须存在 `S.*` 子字段
4. **array 完整性**
   - `value_type=json<array:object>` → 必须存在 `xxx.item.*`
5. **identifier 约束**
   - 仅标量类型
   - unit 必须为空

6. **TypeRef 一致性**
   - `value_type` 以 `ref:` 开头时：必须可解析到同 tenant 的 `status=active` 且 `object_type=feature` 的目标节点
   - 解析后再执行既有门禁（特别是 identifier / unit / object/array 完整性）；禁止循环引用

---

## 七、示例（快速对照）

- `company`

  - object_type=entity

- `company.base`

  - object_type=feature
  - data_class=object
  - value_type=`json<object:company.base>`

- `company.base.name_cn`

  - data_class=attribute
  - value_type=string

- `company.base.name_hist`

  - data_class=array
  - value_type=`json<array:string>`

- `company.base.id.uscc`

  - data_class=identifier
  - value_type=string

- `event.penalty.party_ref_list`
  - data_class=array
  - value_type=`json<array:company|person>`

---

## 八、结论

该版本在保持**单表、强语义、易落地**的同时：

- 支持多租户与并发安全
- 明确 feature 与非 feature 的职责边界
- 支持 entity / scalar 的联合类型表达
- 为未来拆表（Field Definition / Scope / Metric Definition）预留演进空间

> **可作为公司级架构文档直接发布。**

---

## B. 语义字典强门禁与数据库校验规范（v1）
> 本文档用于金融级 Semantic OS 的 **biz_metadata v3** 元数据治理，覆盖：
> - 语义字典强门禁规则（规则表 + 错误码）
> - 数据库侧硬约束设计（DB Guardrails）
> - 校验器实现骨架（Rust / Python）

---

## 一、设计目标
- 防止脏元数据进入生产（Hard Guardrails）
- 保证多租户并发一致性（Optimistic Locking）
- 支持长期演进（Schema & Semantic Governance）

---

## 二、语义门禁规则（摘要）

### 2.1 对象作用域规则
- object_type != feature → data_class / value_type / unit **必须为空**
- object_type = feature → data_class / value_type **必须非空**

### 2.2 data_class 合法取值
attribute | metric | text | object | array | identifier

### 2.3 value_type 表达规范
- 标量：string / int / decimal / boolean / date / datetime
- Union：int|string / company|person
- Object：json<object:S>
- Array：json<array:T>
- TypeRef：ref:<code>（类型引用）

### 2.4 identifier 规则
- value_type ∈ {string, int, int|string}
- unit 必须为空
- code 必须匹配 *.id.<id_type>

### 2.5 unit 规则
- 仅 metric 可填
- identifier / text / object / array 必须为空

---

## 三、数据库侧硬门禁（PostgreSQL 设计清单）

### 3.1 NOT NULL / ENUM
- tenant_id, version, code, name, object_type, status, source

### 3.2 CHECK 约束（示意）
- feature 才能填 data_class/value_type/unit
- identifier 限定 value_type + unit
- unit 仅 metric 可用
- value_type 允许 TypeRef：ref:<code>（存在性/循环/展开校验在 Import Gate / Publish Gate 完成）

### 3.3 唯一性
- UNIQUE (tenant_id, code) WHERE deleted_at IS NULL

### 3.4 父子一致性
- parent_id 外键指向自身
- 触发器校验 parent.tenant_id = child.tenant_id

### 3.5 版本并发
- version NOT NULL DEFAULT 1
- 更新必须 WHERE version = ?

> object / array 完整性建议作为“发布门禁”，不做每次写入触发器。

---

## 四、校验器实现骨架

### 4.1 Python 校验器（本仓库实现）

本规范的“语义字典强门禁（Import Gate / Publish Gate）”由仓库内的 Python 工具直接落地实现：

- 工具目录：`tools/biz-metadata-linter/`
- 实现文件：`tools/biz-metadata-linter/biz_metadata_linter.py`
- 使用说明与规则清单：`tools/biz-metadata-linter/README_biz_metadata_linter.md`

该工具支持：
- `--mode import`：入库前门禁（结构与字段合法性）
- `--mode publish`：发布前门禁（在 import 基础上增加 object/array 完整性）

建议在 CI/门禁中以 `data/biz-metadata/` 为输入目录执行全量校验，并将 JSON 报告作为构建产物留存。

---

## 五、推荐落地顺序
1. CSV/MD → 校验器（Import Gate）
2. DB CHECK / UNIQUE / FK（Write Gate）
3. status=active 前完整性校验（Publish Gate）

---

## 六、结论
这套“语义字典 + 强门禁 + 数据库硬约束”设计，
等价于 Bloomberg/LSEG 的 Field Dictionary + Governance Core，
是金融 AI Agent、风控系统与数据湖/数仓协同的底座能力。
