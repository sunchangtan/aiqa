
# 金融问答系统元数据与数据源映射体系 — 表结构设计（含软删除标记）

> 说明：本文件给出与“统一元数据 + 数据源映射”方案一致的一期表结构设计（MySQL 风格 DDL），并已在所有表中统一增加软删除标记字段 `deleted_at`（NULL 表示未删除）。表名、字段名与管理层方案说明文档保持一致。

---

## 1. 业务语义层

### 1.1 `biz_metadata` — 统一元数据中心

```sql
CREATE TABLE biz_metadata (
  id              BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '元数据ID',
  code            VARCHAR(255) NOT NULL UNIQUE COMMENT '元数据编码，如 company.name / company_finance.revenue',
  name            VARCHAR(255) NOT NULL COMMENT '显示名称，如 企业名称、营业收入',

  node_type       VARCHAR(20) NOT NULL COMMENT '节点类型: entity/field/event/relation/view',

  owner_id        BIGINT NULL COMMENT '归属节点ID: 实体/事件/关系/视图等, 关联 biz_metadata.id',

  value_type      VARCHAR(50) NULL COMMENT '值类型: 标量类型包括 string/int/decimal/boolean/date/datetime; 列表类型采用 list<标量类型> 表达, 如 list<string>/list<int>; 对于关系/复杂对象节点, 使用 list 或 object, 具体字段结构由子元数据(owner_id 关联)描述, 非结构化使用 json',


  semantic_type   VARCHAR(20) NULL COMMENT '分析语义: dimension=维度/属性(如行业、地区、评级, 用于过滤/分组); metric=度量/指标(如收入、金额、次数, 可聚合统计); text=文本字段(如企业简介、公告正文等, 不参与数值聚合)',


  is_identifier   TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否标识字段(如企业ID/债券代码)',
  is_status       TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否状态字段(如是否违约、是否存续)',

  unit            VARCHAR(50) NULL COMMENT '单位: 元/万元/亿/%/人民币 等',

  status          VARCHAR(20) NOT NULL DEFAULT 'active' COMMENT '状态: active/inactive/deprecated',

  description     TEXT NULL COMMENT '业务说明/口径描述',

  extra_json      JSON NULL COMMENT '预留扩展字段(JSON)，如枚举取值、业务标签等',

  created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at      TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  KEY idx_biz_metadata_owner_id (owner_id),
  KEY idx_biz_metadata_node_type (node_type),
  KEY idx_biz_metadata_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='统一业务语义元数据中心';
```

---

### 1.2 `biz_metadata_alias` — 元数据别名（自然语言同义词）

```sql
CREATE TABLE biz_metadata_alias (
  id            BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '别名ID',
  metadata_id   BIGINT NOT NULL COMMENT '关联 biz_metadata.id',
  source        VARCHAR(64) NOT NULL DEFAULT 'manual' COMMENT '别名来源: manual=人工配置; auto_mine=通过算法/模型自动挖掘; log_mine=从用户查询/日志中挖掘; 预留 system/third_party 等扩展值',

  language      VARCHAR(16) NOT NULL DEFAULT 'zh-CN' COMMENT '语言标识，如 zh-CN/en-US',
  weight        INT NOT NULL DEFAULT 0 COMMENT '匹配权重，越大优先级越高',
  is_primary    TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否主要别名',
  source        VARCHAR(64) NOT NULL DEFAULT 'manual' COMMENT '来源: manual/auto_mine/log_mine 等',

  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at    TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  KEY idx_metadata_alias_mid (metadata_id),
  KEY idx_metadata_alias_alias (alias)
  -- 可选外键:
  -- ,CONSTRAINT fk_metadata_alias_metadata
  --   FOREIGN KEY (metadata_id) REFERENCES biz_metadata(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='业务元数据别名/同义词配置';
```

---

### 1.3 `metadata_category` — 业务分类定义

```sql
CREATE TABLE metadata_category (
  id           BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '分类ID',
  code         VARCHAR(100) NOT NULL UNIQUE COMMENT '分类编码，如 company_basic/company_finance/company_risk',
  name         VARCHAR(200) NOT NULL COMMENT '分类名称，如 企业基本信息',

  parent_id    BIGINT NULL COMMENT '父分类ID，顶级为NULL',

  sort_order   INT NOT NULL DEFAULT 0 COMMENT '同一父分类下的排序',

  status       VARCHAR(20) NOT NULL DEFAULT 'active' COMMENT '状态: active/inactive/deprecated',

  icon         VARCHAR(100) NULL COMMENT '前端图标标识(可选)',
  description  TEXT NULL COMMENT '分类说明：业务含义、使用范围等',

  created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at   TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  KEY idx_metadata_category_parent (parent_id),
  KEY idx_metadata_category_status (status)
  -- 可选:
  -- ,CONSTRAINT fk_metadata_category_parent
  --   FOREIGN KEY (parent_id) REFERENCES metadata_category(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='元数据业务分类定义';
```

---

### 1.4 `metadata_category_mapping` — 分类与元数据多对多映射

```sql
CREATE TABLE metadata_category_mapping (
  metadata_id   BIGINT NOT NULL COMMENT '元数据ID, 关联 biz_metadata.id',
  category_id   BIGINT NOT NULL COMMENT '分类ID, 关联 metadata_category.id',

  is_primary    TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否该元数据的主分类',
  weight        INT NOT NULL DEFAULT 0 COMMENT '在该分类下的排序/权重',

  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  deleted_at    TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  PRIMARY KEY (metadata_id, category_id),

  KEY idx_mcm_category (category_id)
  -- 可选外键:
  -- ,CONSTRAINT fk_mcm_metadata
  --   FOREIGN KEY (metadata_id) REFERENCES biz_metadata(id)
  -- ,CONSTRAINT fk_mcm_category
  --   FOREIGN KEY (category_id) REFERENCES metadata_category(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='元数据与业务分类映射(支持一字段多归属)';
```

---

### 1.5 `metadata_view_field`（可选）— 视图与字段关系

```sql
CREATE TABLE metadata_view_field (
  view_metadata_id   BIGINT NOT NULL COMMENT '视图元数据ID, 要求 node_type=view',
  field_metadata_id  BIGINT NOT NULL COMMENT '字段元数据ID, 一般 node_type=field',

  is_default         TINYINT(1) NOT NULL DEFAULT 1 COMMENT '是否视图默认返回该字段',
  display_order      INT NOT NULL DEFAULT 0 COMMENT '在该视图中的展示顺序',

  created_at         TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  deleted_at         TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  PRIMARY KEY (view_metadata_id, field_metadata_id),

  KEY idx_mvf_field (field_metadata_id)
  -- 可选外键:
  -- ,CONSTRAINT fk_mvf_view
  --   FOREIGN KEY (view_metadata_id) REFERENCES biz_metadata(id)
  -- ,CONSTRAINT fk_mvf_field
  --   FOREIGN KEY (field_metadata_id) REFERENCES biz_metadata(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='视图与字段包含关系定义';
```

---

### 1.6 `biz_metadata_relation` — 元数据之间的业务关系

```sql
CREATE TABLE biz_metadata_relation (
  id               BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '关系ID',

  from_metadata_id BIGINT NOT NULL COMMENT '起点元数据ID(引用端/外键端)，如 company_finance.company_id',
  to_metadata_id   BIGINT NOT NULL COMMENT '终点元数据ID(被引用端/主键端)，如 company.company_id',

  relation_type    VARCHAR(50) NOT NULL COMMENT '关系类型: foreign_key/same_as/join_key/derived_from 等',
  cardinality      VARCHAR(20) NULL COMMENT '基数: 1-1/1-N/N-1/N-N (可选)',
  is_bidirectional TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否可看作双向同义关系(如 same_as)',

  description      TEXT NULL COMMENT '关系说明',

  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at       TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  UNIQUE KEY uk_metadata_relation (from_metadata_id, to_metadata_id, relation_type),

  KEY idx_metadata_relation_from (from_metadata_id),
  KEY idx_metadata_relation_to (to_metadata_id)
  -- 可选外键:
  -- ,CONSTRAINT fk_metadata_relation_from
  --   FOREIGN KEY (from_metadata_id) REFERENCES biz_metadata(id)
  -- ,CONSTRAINT fk_metadata_relation_to
  --   FOREIGN KEY (to_metadata_id)   REFERENCES biz_metadata(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='元数据之间的外键/同义/join/派生关系';
```

---

## 2. 数据源层

### 2.1 `biz_data_source` — 数据源登记

```sql
CREATE TABLE biz_data_source (
  id            BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '数据源ID',
  code          VARCHAR(64) NOT NULL UNIQUE COMMENT '数据源编码，如 pevc_event_api_source',
  name          VARCHAR(256) NOT NULL COMMENT '数据源名称，如 PEVC事件服务',

  source_type   VARCHAR(32) NOT NULL COMMENT '类型: api/database/file/mq/cache',
  engine        VARCHAR(64) NOT NULL COMMENT '具体引擎: rest/grpc/mysql/postgres/kafka 等',

  conn_config   JSON NOT NULL COMMENT '连接/鉴权配置，如 base_url/连接串/认证信息等',

  owner_team    VARCHAR(64) NULL COMMENT '责任团队，如 RiskDataTeam',
  status        VARCHAR(20) NOT NULL DEFAULT 'active' COMMENT '状态: active/inactive/deprecated',

  description   TEXT NULL COMMENT '数据源用途描述',

  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at    TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='数据源登记表(API/DB/文件等)';
```

---

### 2.2 `biz_api` — 接口登记

```sql
CREATE TABLE biz_api (
  id               BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT 'API ID',
  code             VARCHAR(64) NOT NULL UNIQUE COMMENT '接口编码，如 pevc_event_list',
  name             VARCHAR(256) NOT NULL COMMENT '接口名称，如 PEVC事件列表查询',

  data_source_id   BIGINT NOT NULL COMMENT '数据源ID，关联 biz_data_source.id',

  endpoint         VARCHAR(512) NOT NULL COMMENT '接口路径/URL 或逻辑表名',
  http_method      VARCHAR(8) NULL COMMENT 'HTTP 方法: GET/POST/...，protocol=rest 时使用',
  protocol         VARCHAR(16) NOT NULL COMMENT '协议: rest/grpc/sql/...',

  req_schema       JSON NULL COMMENT '请求参数schema(可选)',
  resp_schema      JSON NULL COMMENT '响应结构schema(可选)',

  default_limit    INT NULL COMMENT '默认分页大小(如适用)',

  status           VARCHAR(20) NOT NULL DEFAULT 'active' COMMENT '状态: active/inactive/deprecated',
  description      TEXT NULL COMMENT '接口用途说明',
  is_stateless     TINYINT(1) NOT NULL DEFAULT 1 COMMENT '是否无状态/幂等',

  created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at       TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  KEY idx_biz_api_datasource (data_source_id),
  KEY idx_biz_api_status (status)
  -- 可选外键:
  -- ,CONSTRAINT fk_biz_api_datasource
  --   FOREIGN KEY (data_source_id) REFERENCES biz_data_source(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='可用接口/逻辑表登记';
```

---

## 3. 映射与过滤层

### 3.1 `biz_api_output_mapping` — 接口响应字段 → 语义字段映射

```sql
CREATE TABLE biz_api_output_mapping (
  id            BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '映射ID',

  api_id        BIGINT NOT NULL COMMENT '接口ID，关联 biz_api.id',
  metadata_id   BIGINT NOT NULL COMMENT '语义元数据ID，关联 biz_metadata.id',

  response_path VARCHAR(512) NOT NULL COMMENT '响应JSON路径(或列名)，如 $.data[*].companyId',

  is_primary    TINYINT(1) NOT NULL DEFAULT 1 COMMENT '是否该元数据在此API中的主映射',
  agg_rule      VARCHAR(64) NULL COMMENT '若该API返回聚合结果，可指定sum/count等规则(可选)',

  description   TEXT NULL COMMENT '说明(字段口径、注意事项)',

  created_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at    TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  UNIQUE KEY uk_api_output_mapping (api_id, metadata_id, response_path),

  KEY idx_aom_api (api_id),
  KEY idx_aom_metadata (metadata_id)
  -- 可选外键:
  -- ,CONSTRAINT fk_aom_api
  --   FOREIGN KEY (api_id) REFERENCES biz_api(id)
  -- ,CONSTRAINT fk_aom_metadata
  --   FOREIGN KEY (metadata_id) REFERENCES biz_metadata(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='接口响应字段到语义元数据的映射';
```

---

### 3.2 `biz_api_filter_mapping` — 接口筛选条件 → 语义字段映射

```sql
CREATE TABLE biz_api_filter_mapping (
  id             BIGINT PRIMARY KEY AUTO_INCREMENT COMMENT '筛选映射ID',

  api_id         BIGINT NOT NULL COMMENT '接口ID，关联 biz_api.id',
  metadata_id    BIGINT NOT NULL COMMENT '语义元数据ID，作为筛选条件字段',

  param_name     VARCHAR(128) NOT NULL COMMENT '请求参数名，如 eventType/companyId/dateFrom',
  supported_ops  VARCHAR(128) NOT NULL COMMENT '支持操作符列表，如 eq,in,gt,lt,between,like',

  param_type     VARCHAR(32) NOT NULL COMMENT '参数类型: string/int/date/datetime 等',
  value_template VARCHAR(256) NULL COMMENT '可选，值转换模板或枚举映射描述',

  required       TINYINT(1) NOT NULL DEFAULT 0 COMMENT '是否为必填条件',

  description    TEXT NULL COMMENT '说明(参数含义、限制)',

  created_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  updated_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  deleted_at     TIMESTAMP NULL DEFAULT NULL COMMENT '软删除时间，NULL 表示未删除',

  UNIQUE KEY uk_api_filter_mapping (api_id, metadata_id, param_name),

  KEY idx_afm_api (api_id),
  KEY idx_afm_metadata (metadata_id)
  -- 可选外键:
  -- ,CONSTRAINT fk_afm_api
  --   FOREIGN KEY (api_id) REFERENCES biz_api(id)
  -- ,CONSTRAINT fk_afm_metadata
  --   FOREIGN KEY (metadata_id) REFERENCES biz_metadata(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='接口筛选参数到语义元数据的映射';
```
