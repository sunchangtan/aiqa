# biz_metadata_linter（Python）

规范文档：
- `docs/金融语义字典（biz_metadata）模型与强门禁校验规范_v1.0.md`

## 适用场景
- Import Gate：CSV/MD 入库前校验（阻止脏元数据进入 DB）
- Publish Gate：将 status=active 前，对 object/array 完整性、TypeRef 可解析性做全量校验

该工具实现了规范中的核心门禁：
- scope：object_type != feature 禁止填写 data_class/value_type/unit；feature 必填 data_class/value_type
- types：value_type 语法（标量/Union/object/array）+ TypeRef（ref:<code>）
- identifier：value_type 仅允许 {string,int,int|string} + unit 为空 + code 命名匹配 *.id.<id_type>
- unit：仅 metric 可填 unit
- hierarchy：parent_code 存在且必须是 code 前缀
- uniqueness：同一输入批次 (tenant_id, code) 不得重复
- completeness（publish）：object 必须存在 S.* 子字段；json<array:object> 必须存在 xxx.item.*

说明：
- 若输入只包含部分对象文件（例如只校验 `company`，但未包含 `person`），且存在 `ref:person.*` 的 TypeRef，则会报 `TYPE_REF_NOT_FOUND`；这是预期行为，门禁需要在同一批次拿到被引用的定义才能完成解析。

## 门禁规则清单（实现状态）

说明：
- Gate：
  - Import：入库前门禁
  - Publish：发布前门禁（在 Import 基础上更严格）
- 状态：
  - ✅ 已实现：当前工具已覆盖并在 CI/门禁中可直接使用
  - ⚠️ 部分实现：仅覆盖部分子规则或覆盖方式与规范略有差异（会在备注说明）
  - ❌ 未实现：需要补充实现（建议在 DB Guardrails 或后续版本加入）

| 分类 | Gate | 规则（摘要） | 规范引用 | 规则/错误码（rule_id） | 状态 |
| --- | --- | --- | --- | --- | --- |
| 基础 | Import/Publish | 必填字段：tenant_id/code/name/object_type/status/source；version>0 | A.3.1/A.3.2/A.3.5；B.3.1 | BASIC_REQUIRED_MISSING / BASIC_VERSION_INVALID | ✅ |
| 枚举 | Import/Publish | object_type/status/source/data_class 取值合法 | A.一/1；A.3.4/A.3.5；B.2.2 | ENUM_OBJECT_TYPE / ENUM_STATUS / ENUM_SOURCE / ENUM_DATA_CLASS | ✅ |
| 命名 | Import/Publish | code 为点分 snake_case（如 company.base.name_cn） | A.3.2 | CODE_FORMAT | ✅ |
| 唯一性 | Import/Publish | 同一批次 (tenant_id, code) 不重复 | B.3.3（DB UNIQUE 的输入侧补充） | UNIQUE_TENANT_CODE | ✅ |
| Scope | Import/Publish | 非 feature 禁止填写 data_class/value_type/unit；feature 必填 data_class/value_type | A.一/2；第六章(1/2)；B.2.1 | SCOPE_NON_FEATURE_HAS_TYPE / SCOPE_FEATURE_MISSING_TYPE | ✅ |
| value_type 语法 | Import/Publish | 允许：标量 / Union / json<object:S> / json<array:T> / ref:<code> | A.5；B.2.3 | TYPE_SYNTAX_INVALID | ✅ |
| TypeRef 解析 | Import/Publish | ref:<code> 目标存在、禁止循环、限制深度、解析后仍需符合 value_type 语法 | A.5.6；第六章(6)；B.2.3 | TYPE_REF_NOT_FOUND / TYPE_REF_CYCLE / TYPE_REF_TOO_DEEP / TYPE_REF_RESOLVED_INVALID | ✅ |
| TypeRef 目标约束 | Import/Publish | ref 目标必须 object_type=feature 且 status=active | A.5.6；第六章(6) | TYPE_REF_TARGET_NOT_FEATURE / TYPE_REF_TARGET_NOT_ACTIVE | ✅ |
| unit | Import/Publish | unit 仅 metric 可填；identifier 必须为空 | A.一/3；B.2.5；A.5.5 | UNIT_NOT_ALLOWED / IDENTIFIER_UNIT_NOT_EMPTY | ✅ |
| identifier | Import/Publish | data_class=identifier：value_type∈{string,int,int\|string}（支持 ref 展开后判断）；code 匹配 *.id.<id_type> | A.5.5；B.2.4 | IDENTIFIER_VALUE_TYPE / IDENTIFIER_CODE_PATTERN | ✅ |
| 层级 | Import/Publish | parent_code 存在且必须为 code 前缀（同 tenant） | A.3.3（层级组织） | HIERARCHY_PARENT_MISSING / HIERARCHY_PARENT_PREFIX | ✅ |
| 完整性 | Publish | data_class=object 且 value_type=json<object:S> ⇒ 必须存在 S.* 子字段 | 第六章(3) | COMPLETENESS_OBJECT_CHILDREN_MISSING | ✅ |
| 完整性 | Publish | value_type=json<array:object> ⇒ 必须存在 xxx.item.* 子字段 | 第六章(4) | COMPLETENESS_ARRAY_OBJECT_ITEMS_MISSING | ✅ |
| 约束细节 | Import/Publish | value_type 长度/单位枚举/更细粒度语义治理（如 metric 周期、口径等） | A.字段字典补充项 | - | ❌ |
| DB 侧 | - | deleted_at 过滤唯一性、parent_id FK、tenant_id 一致性、乐观锁更新约束等 | B.3 | - | ❌（不属于文件门禁） |

## 安装
无外部依赖（Python 3.10+）。

## 使用

### 1) Import Gate
```bash
python biz_metadata_linter.py --mode import --input company.csv person.csv --report import_report.json
```

也支持直接传入目录（会递归扫描 `.csv` / `.md` / `.markdown`）：
```bash
python biz_metadata_linter.py --mode import --input ../../data/biz-metadata/ --report import_report.json
```

### 2) Publish Gate
```bash
python biz_metadata_linter.py --mode publish --input company.csv person.csv --report publish_report.json
```

### 3) 直接对 MD 进行校验
```bash
python biz_metadata_linter.py --mode import --input dictionary.md
```

## 输出
- `report.json` 包含：
  - row_count / error_count / warn_count / violation_count
  - resolved_inputs（目录输入会展开为实际文件列表）
  - violations（每条含 rule_id / message / tenant_id / code / field / value）

字段含义：
- `row_count`：本次参与校验的总行数（CSV 行 + MD 表格行）。
- `violation_count`：违规总数，等于 `error_count + warn_count`，也等于 `violations` 数组长度。

## 与 DB Guardrails 的边界
- DB 侧 UNIQUE/FK/乐观锁等约束依然建议保留；
- 该工具用于“写入前/发布前”的强门禁，与 DB 约束互补。
