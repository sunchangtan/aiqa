# 金融 AI Agent 五类对象统一架构与建模规则（对标 Bloomberg）

> 本文档由两部分合并而成：  
> **A. 五类数据对象总体架构设计（战略与体系层）**  
> **B. 五类对象建模规则与判定规范（工程与执行层）**  
>
> 作为公司级长期数据与 AI 架构基石，本文件具有最高优先级，后续所有数据系统、数据湖、数仓、图谱与 AI Agent 均需遵循。

---

## 第一部分：总体架构设计（Architecture Blueprint）


# 金融 AI Agent 五类数据对象架构设计文档（对标 Bloomberg 风控与数据体系）

## 1. 文档目的与背景

本文件定义公司级 **金融 AI Agent 数据基石架构**，作为后续：
- 数据开发
- 数据湖（Data Lake）
- 数据仓库（DWH / Lakehouse）
- 特征库（Feature Store）
- 知识图谱（KG）
- 风控 / 合规 / 投研 AI Agent

的**统一语义与对象模型标准**。

该架构对标 **Bloomberg / Refinitiv 等大型金融数据与风控系统的底层建模思想**，
并结合 AI Agent 对可解释性、证据链与可计算性的要求进行工程化抽象。

> 本架构为 **长期稳定架构**，要求：  
> **正确性优先、强约束、可审计、可演进。**

---

## 2. 核心结论（Executive Summary）

1. 采用五类对象作为公司金融数据的统一抽象：

```
Entity / Event / Relation / Document / Feature
```

2. Risk Typology 不作为第六类对象，而是：
- 归属 **Taxonomy / Classification Layer**
- 作为 Event / Entity 的风险分类维度存在

3. 该架构：
- 能覆盖 >95% 金融系统数据
- 与 Bloomberg 的 Master Data + Event + Risk + Evidence 模型一致
- 可直接指导数据湖、数仓、AI Agent 工具链建设

---

## 3. 对标 Bloomberg 的整体架构视角

### 3.1 Bloomberg 的真实建模原则（事实层）

Bloomberg 并不在 UI 层显式暴露“实体/事件/关系”，但在其底层：

- **Master Entity**：公司、人、证券、发行人、国家
- **Relationship Network**：股权、控制、任职、担保
- **Event Model**：并购、处罚、违约、诉讼、制裁
- **Risk Typology**：合规/信用/法律/声誉/制裁风险
- **Evidence**：新闻、公告、监管文件、判决

本方案将该隐性模型 **显性化、标准化、工程化**，以支撑 AI Agent。

---

## 4. 五类对象模型（公司统一定义）

## 4.1 Entity（实体）

### 定义
在时间上相对稳定存在、可被多次事件引用、具备唯一标识的对象。

### 典型实体
- Company / Issuer
- Person
- Security（Stock / Bond / Fund）
- Index
- Industry
- Region / Economy
- Institution

### 设计要求（强制）
- 全局唯一 `entity_id`
- 多源映射（ticker / ISIN / 统一社会信用代码）
- 生命周期管理（生效/失效/合并拆分）

### 对标 Bloomberg
- Bloomberg Entity Master / PermID

---

## 4.2 Event（事件）

### 定义
在时间线上发生、会引起状态变化或风险暴露的事实单元。

### 典型事件
- 并购 / 投资 / 融资
- 财报发布
- 股权变动
- 违约 / 重组
- 诉讼 / 仲裁
- 监管处罚 / 调查
- 政策发布（作为“发布事件”）

### 设计要求
- event_type 受控枚举
- 多时间轴：发生/披露/生效
- 可去重（多源新闻 → 单一事件）

### 对标 Bloomberg
- Corporate Action Event
- Regulatory Event
- Credit Event

---

## 4.3 Relation（关系）

### 定义
实体↔实体、实体↔事件之间的结构性连接。

### 典型关系
- 股东关系
- 控制关系
- 任职关系
- 投资关系
- 事件参与角色

### 设计要求
- relation_type 受控
- start_time / end_time
- role（事件中的角色）
- 可回溯历史

### 对标 Bloomberg
- Ownership Graph
- Control & Beneficial Ownership Network

---

## 4.4 Document（文档 / 证据）

### 定义
对实体、事件、关系进行描述、披露或证明的信息载体，本身不是事实。

### 包含类型
- 新闻
- 公告
- 法规文本
- 监管处罚文件
- 法院判决
- 研报 / 分析报告

### 设计要求
- source / credibility / publish_time
- 与 Entity / Event 通过 evidence 关系关联
- 全文与结构化元数据分离

### 对标 Bloomberg
- News
- Filings
- Regulatory Documents
- Adverse Media Evidence

---

## 4.5 Feature（属性 / 指标）

### 定义
用于描述实体或事件状态的特征集合，包含属性与可计算指标。

### 子分类
- Attribute：不需要计算窗口（成立日期、行业）
- Metric：需要公式/周期（营收TTM、PE、波动率）

### 设计要求
- 定义与取值分离
- 指标版本化、口径可追溯
- 币种/单位/周期标准化

### 对标 Bloomberg
- Financial Fields
- Calculated Metrics
- Analytics Factors

---

## 5. Risk Typology 的定位（重要）

### 定义
对风险“性质”的标准化分类体系。

### 位置
- 属于 **Taxonomy Layer**
- 挂载在 Event / Entity 的风险评估上

### 常见类型
- Regulatory Risk
- Legal Risk
- Credit Risk
- Governance Risk
- Sanctions / AML Risk
- Reputational Risk

### 对标 Bloomberg
- Risk Classification
- Compliance & AML Risk Types

---

## 6. 数据湖 / 数仓落地映射

### 6.1 数据湖分层

- Bronze：原始数据（按来源）
- Silver：五类对象标准化层
- Gold：业务/AI 服务层

### 6.2 数仓建模映射

| 架构对象 | 数仓角色 |
|------|------|
| Entity | 维表 / 主数据 |
| Event | 事实表 |
| Relation | 桥接表 |
| Document | 内容表 + 元数据 |
| Feature | 特征事实表 |

---

## 7. AI Agent 可解释性约束（强制）

任何 Agent 输出必须绑定：
- entity_id
- event_id（如适用）
- doc_id（证据）
- feature / metric_id
- risk_typology（如涉及风险判断）

---

## 8. 架构覆盖性评估

### 覆盖范围
- 行情、基本面、公告、新闻、法规、风控、研报、关系穿透

### 结论
- 覆盖金融核心数据 >95%
- 少数交易流水等可作为 Event/Feature 扩展

---

## 9. 结论

本五类对象架构：
- 对齐 Bloomberg 等头部系统底层逻辑
- 满足金融 AI Agent 对可解释、可追溯、可计算的要求
- 可作为公司长期数据与智能化建设的统一基石



---

## 第二部分：建模规则与执行规范（Modeling Rules & Governance）

# 金融 AI Agent 五类对象建模规则（对标 Bloomberg 的数据/风控建模思想）

> 目标：给出一套可长期执行的“像 Bloomberg 一样”的建模规则，用于统一公司 **数据开发 / 数据湖 / 数仓 / 大数据仓库 / 图谱 / RAG / Agent 工具路由**。  
> 本规则强调：**对象划分一致性、可追溯证据链、可去重、可版本化、可审计**。

---

## 1. 参考基准：Bloomberg 的关键建模约束（可核验公开材料）

Bloomberg 的参考数据/法律实体数据公开材料中明确强调两点：

1) **证券（Security）必须先链接到发行人（Issuer）/法律实体（Legal Entity）才能进入数据库**：这是一种“先有实体主数据、再挂载金融工具”的约束，用于企业级风险暴露聚合。 citeturn0search4  
2) Bloomberg 的 Reference Data 覆盖 **legal entity、corporate actions、classifications、holdings、ownership** 等，并用于贯通前中后台工作流。 citeturn0search11  
3) Bloomberg 的 FIGI/BSYM 体系强调“唯一、持久、可映射”的金融工具标识模型（用于连接多源数据）。 citeturn0search5turn0search12turn0search9  
4) Bloomberg 的 Event-Driven Feeds 强调提供**机器可读的结构化事件数据**（而非仅文本）。 citeturn0search15  

> 结论：Bloomberg 的“像样数据模型”核心是：**主数据（Entity/Security）—关系（Ownership/Control）—事件（Corporate Action/Regulatory/Credit）—证据（News/Filings）—指标（Pricing/Financials）** 的贯通与约束。

---

## 2. 五类对象的权威定义（公司统一口径）

### 2.1 Entity（实体）
**定义**：时间上相对稳定存在、可被多次引用、拥有唯一标识的对象。  
**例子**：公司/发行人、自然人、证券（股票/债券/基金/衍生品）、指数、行业、地区经济体、机构、账户（如适用）。  
**必须具备**：`global_id` + `entity_type` + 生命周期（effective_from/to 或 status）。

### 2.2 Event（事件）
**定义**：在时间线上发生的、引起状态变化或风险暴露的事实单元。  
**例子**：并购/融资/重组、财报发布、评级调整、违约/展期、诉讼/仲裁、监管处罚/调查、停牌/复牌、政策发布（作为“发布事件”）。  
**必须具备**：`event_type`（受控）+ 多时间轴（发生/披露/生效）+ 参与方（通过 Relation）。

### 2.3 Relation（关系）
**定义**：实体↔实体、实体↔事件之间的结构性连接，具备类型、方向与时效。  
**例子**：持股/控制、任职、担保、投资、事件参与角色（收购方/标的/参投方/被处罚方）。  
**必须具备**：`relation_type`（受控）+ `from/to` + `start/end` +（可选）`source_event_id`。

### 2.4 Document（文档/证据）
**定义**：对实体/事件/关系进行描述、披露或证明的信息载体；**不是事实本身**。  
**例子**：新闻、公告、监管文件、法院判决、法规文本、研报。  
**必须具备**：`source`、`publish_time`、`doc_type`、`credibility`、可追溯 URI/Hash。

### 2.5 Feature（特征：属性/指标）
**定义**：描述实体或事件状态的字段与可计算指标集合。  
- **Attribute**：无窗口/公式（注册地、成立日、票面利率）  
- **Metric**：有口径/公式/周期（营收TTM、PE、波动率30D、市值）  
**必须具备**：定义与取值分离（`*_def` vs `*_value`），指标版本化（period/as_of/version）。

---

## 3. “像 Bloomberg 一样”的五类划分规则（可执行判定树）

下面给出一个**判定树（Decision Tree）**，用于把任何新数据对象落入五类之一。

### 3.1 判定树

1) **它是不是“信息载体/文件/文本/披露”？**  
- 是 → **Document**  
- 否 → 继续

2) **它是否是“长期存在的对象”，具有稳定 ID，可被多次引用？**  
- 是 → **Entity**  
- 否 → 继续

3) **它是否是“在时间线上发生的变化/动作/结果”，并有发生/披露/生效时间？**  
- 是 → **Event**  
- 否 → 继续

4) **它是否描述“两个对象之间（或对象与事件之间）的连接/角色/结构”，并且可回溯历史？**  
- 是 → **Relation**  
- 否 → 继续

5) **它是否是“对对象状态的字段/数值”，并可能带口径/周期/窗口？**  
- 是 → **Feature**  
- 否 → 返回建模委员会（通常为“新实体类型”或“新事件类型”）

---

## 4. 核心硬约束（Bloomberg 风格的“不可违反原则”）

### P0：实体优先（Entity-First）
> **任何可交易工具（Security）必须先链接发行人/法律实体（Issuer / Legal Entity）后才允许入库。**  
这与 Bloomberg “securities linked to issuers prior to entry” 一致。 citeturn0search4

**工程化表达**：
- `security_entity.issuer_entity_id NOT NULL`
- 入库校验：无 issuer 的 security 拒绝进入 Silver/Gold 层（可留在 Bronze 原始层）

---

### P0：证据链强制（Evidence-First for AI）
> 文档永远是证据，不是事实；事实必须由 Event/Relation 表达。

**工程化表达**：
- Document 只能通过 `document_link` 连接到 Entity/Event/Relation
- Event/Relation 必须能回溯到至少一个 doc_id（或内部系统凭证）

---

### P0：事件去重（One Fact, One Event）
> 同一现实事件，多源新闻/公告应合并为单一事件对象（多个证据）。

**工程化表达**：
- `event_fingerprint`（指纹）+ 合并策略
- `event_id` 唯一；`doc_id` 可多对一

---

### P1：关系必须时态化（Temporal Relation）
> 关系没有 start/end 就无法回溯“当时的真实结构”（股权穿透、关联方风险会失真）。

**工程化表达**：
- `relation.start_time` 必填（至少为披露时间）
- 终止关系必须写 `end_time`

---

### P1：指标必须口径化与版本化（Metric Definition Governance）
> 不版本化指标会导致“同名不同义”，数仓与 AI 输出会互相打架。

**工程化表达**：
- `metric_def`（公式、单位、币种、频率、是否TTM等）
- `metric_value(period, as_of, version)` 唯一约束

---

## 5. Document 与 Entity/Event/Relation 的关系：两种关系必须分开

### 5.1 业务事实关系（Business Relation）
进入 `relation`：持股/控制/任职/担保/投资/事件参与角色。

### 5.2 证据关系（Evidence Link）
进入 `document_link`：mention/disclose/prove/apply_to 等。

**关键规则**：  
- `document_link` 不参与风险传播与网络推理  
- 风险传播只基于 `relation`（业务关系）与 `event`（事实变化）

---

## 6. Risk Typology 在五类中的位置（Bloomberg 风格）

Risk Typology **不是对象类型**，而是 **分类维度（Taxonomy Layer）**，用于：
- 给 Event 打风险性质标签（多对多）
- 给 Entity 做风险评估（score/severity/confidence）

与 Bloomberg 的合规/制裁筛查与风险聚合产品逻辑一致：其核心是对实体进行暴露聚合与分类管理，而不是把“风险类型”当实体节点。 citeturn0search21turn0search13

---

## 7. 典型数据如何落入五类（对照表）

| 数据项 | 五类归属 | 说明 |
|---|---|---|
| 公司、发行人、自然人 | Entity | 主数据对象 |
| 股票/债券/基金（工具） | Entity（Security） | 必须链接 issuer citeturn0search4 |
| 股东名单（A持股B 30%） | Relation | 业务结构关系（时态化） |
| 并购公告 | Document | 披露载体 |
| 并购行为本身 | Event | 一个现实事件（可被多 doc 支撑） |
| 并购参与方（收购方/标的） | Relation | Event ↔ Entity 角色关系 |
| 新闻报道“传闻将并购” | Document | 不自动生成 Event（除非规则满足） |
| 财报发布 | Event | 发布事件 |
| 财报全文 | Document | 证据与载体 |
| 营收TTM、PE、波动率30D | Feature（Metric） | 必须口径/周期/版本化 |
| 注册地、成立日、票息 | Feature（Attribute） | 轻量字段，仍需 as_of |
| 处罚决定书 | Document | 证据 |
| 处罚行为（处罚事件） | Event | 事实变化 |
| 风险类型（合规/信用/声誉） | Taxonomy | 挂在 Event/Entity，不是对象 |

---

## 8. 数据湖/数仓落地的“必须遵守”映射规则

### 8.1 Bronze（原始层）
- 按来源入湖，保留原始 JSON/PDF/HTML
- 不做语义合并（便于审计）

### 8.2 Silver（标准化对象层）
- 全部映射为五类对象
- 建立 `document_link`
- 事件去重、关系时态化、指标口径化

### 8.3 Gold（服务层）
- 面向场景的数据集市（投研/风控/合规/行情）
- 特征服务（Feature API）
- Agent Tool API（带证据链）

---

## 9. 组织与治理：确保长期一致的“制度化”机制

### 9.1 字典治理（必须）
- `entity_type` / `event_type` / `relation_type` / `doc_type` / `metric_def` / `risk_typology_def`  
统一由数据治理委员会维护，变更需 ADR。

### 9.2 建模评审门禁（必须）
任何新数据源接入 Silver 层必须通过：
- 五类归属评审
- ID 映射覆盖率门槛
- 事件去重策略
- 指标口径与版本策略
- 证据链可追溯

---

## 10. 最小可落地实现（MVP）建议

1) 先建立 Entity 主数据与 ID 映射（Security → Issuer 强约束） citeturn0search4turn0search9  
2) 建立 Event 基础类型 + 指纹去重  
3) 关系先覆盖股权/任职/担保/事件角色  
4) Document 入湖 + `document_link`  
5) Feature 先覆盖行情与财务的 Top 50 指标，建立 `metric_def`

---

## 11. 附：常用的判定示例（可直接培训团队）

### 示例 A：新闻“公司A疑似被调查”
- Document：新闻本身
- 若无监管文件/多源确认 → 不建处罚事件，仅建 `document_link(mention)`
- 若有处罚决定书或可靠来源 → 建 Event（监管调查/处罚），并用 doc 作为证据

### 示例 B：公告“公司A收购公司B 51%”
- Document：公告全文
- Event：并购事件（1个）
- Relation：E ↔ A（收购方），E ↔ B（标的）；事件生效后可产生 A→B 控制关系（时态化）
- Feature：收购金额、股权比例（可作为事件属性）

---

# 12. 一句话总结（给领导的）

> 这套五类对象建模规则，将 Bloomberg 的“主数据—关系—事件—证据—指标”思想显性化，并以证据链、去重、时态、口径治理为硬约束，保证公司未来所有数据开发与数据平台建设在同一语义基座上可持续演进。

---

# 附录 A：biz_metadata（元数据目录）建模规范（基于五类对象架构）

## A.1 目标与适用范围
biz_metadata 用于承载公司级“语义字典/对象目录”，服务于：
- 数据湖 Silver 层对象标准化与字段映射
- 数仓口径统一、字段血缘与审计
- AI Agent 路由（字段解析、工具选择）与可解释输出

本附录聚焦 **Entity + Feature**（公司基本信息等主数据与字段）场景；Event/Relation/Document 的扩展规则沿用主文档的五类对象定义。

---

## A.2 五类对象与 biz_metadata 的映射原则（强制）

1) biz_metadata **描述“定义”**，不存放业务数据“取值”。  
2) `node_type` 用于表达“五类对象类型”；字段/指标一律归入 `feature`。  
3) 字段组（如 `company.base`）不是第六类对象，属于 **feature 的对象形态**。  
4) Document 与对象之间的连接属于 **证据链接（document_link）**，不得混入 feature/relation。

---

## A.3 解决“feature_group 与 group 重复”的标准口径

### 推荐做法（最干净、可长期执行）
- `node_type` 仅使用五类：`entity | event | relation | document | feature`
- `data_class` 用于描述 **feature 的形态**：
  - `attribute`：普通属性（维度类）
  - `metric`：指标（口径/周期/版本化）
  - `text`：长文本
  - `object`：对象结构（JSON 子对象/字段组）
  - `array`：数组结构（JSON 数组）

因此：
- `company.base` → `node_type=feature` + `data_class=object` + `value_type=jsonb`
- `company.base.name_cn` → `node_type=feature` + `data_class=attribute` + `value_type=string`

---

## A.4 字段含义纠偏（必须避免的三类错误）

### 错误 1：把实体（entity）当成 json 值
- `company` 是实体定义，不应强行填写 `value_type=jsonb` 或 `data_class=group`。
- 实体行仅需要：`code/name/description/node_type/entity_type/status` 等定义信息。

### 错误 2：把“文档提及”当成事实关系
- 新闻/公告属于 document；其与 company 的连接是 evidence link（mention/disclose），不是业务 relation。

### 错误 3：把所有数值都当“metric”
- 指标（metric）需要治理：口径/周期/版本；否则数仓与 AI 输出会冲突。
- 像“注册资本 reg_cap_amt”更接近“金额属性”（attribute），除非你明确把 metric 定义为“任何数值型字段”。公司级建议保持区分。

---

## A.5 推荐的 biz_metadata 最小字段集（可落地）

### A.5.1 biz_metadata（定义表）
- `id`：bigint PK
- `code`：varchar，语义路径（如 `company.base.name_cn`）
- `name`：varchar，中文名
- `description`：text
- `node_type`：enum（`entity/event/relation/document/feature`）
- `parent_id`：bigint（层级结构，替代 owner_code/owner_id 混用）
- `data_class`：enum（`attribute/metric/text/object/array`）仅当 node_type=feature 时使用
- `value_type`：enum（`string/int/decimal/boolean/date/datetime/jsonb`）仅当 node_type=feature 时使用
- `unit`：varchar nullable（单位）
- `is_identifier`：bool default false（是否可做唯一识别/主键候选）
- `status`：enum（`active/deprecated`）
- `source`：enum（`manual/auto_mine/api_sync`）
- `created_at/updated_at/deleted_at`

### A.5.2 external_id_map（实体外部标识映射，强烈建议独立）
- `entity_id`
- `id_type`（`uscc/reg_no/tax_id/company_code/ticker/isin/figi/...`）
- `id_value`
- `source`
- `effective_from/effective_to`（可选）

> 说明：你样例中的 `company.id`（CompanyCode）属于 external_id_map 的典型用例；不要把它误认为全局 entity_id。

---

## A.6 基于“公司基本信息”样例的推荐落位（示意）

| code | node_type | data_class | value_type | is_identifier | 说明 |
|---|---|---|---|---|---|
| company | entity |  |  |  | 公司实体定义 |
| company.id | feature | attribute | int | true | 外部源 company_code（建议映射表管理） |
| company.base | feature | object | jsonb | false | 基本档案字段组 |
| company.base.name_cn | feature | attribute | string | false/（可选 true） | 公司中文名（是否唯一视业务规则） |
| company.base.uscc | feature | attribute | string | true | 统一社会信用代码 |
| company.base.biz_scope | feature | text | string | false | 经营范围 |
| company.base.reg_cap_amt | feature | attribute（或 metric） | decimal | false | 注册资本金额（非 period 指标） |

---

## A.7 面向数据湖/数仓的落地规则（与主文档一致）
- Bronze：原始 API JSON / 文档全文落盘
- Silver：根据 biz_metadata 将 JSON 映射为规范化 feature（含 object/array 结构）
- Gold：面向场景生成宽表/主题数据集，并保持 `code → column` 的稳定映射

---

# 附录 B：接入门禁（Checklist，建议纳入制度）

新数据源进入 Silver/Gold 前必须通过：
1) 五类对象归属评审（node_type）
2) feature 形态评审（data_class）
3) 标识字段与映射策略（is_identifier + external_id_map）
4) 指标口径与版本策略（若 data_class=metric）
5) 文档证据链可追溯（doc_id/uri/hash）
