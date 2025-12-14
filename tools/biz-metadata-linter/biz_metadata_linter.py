#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""biz_metadata 强门禁校验器（Python 版本）

本工具用于对 `data/biz-metadata` 下的 CSV/Markdown 表格数据执行“入库前/发布前”的
强门禁校验，避免脏元数据进入数据库。

规范来源：
- `docs/金融语义字典（biz_metadata）模型与强门禁校验规范_v1.0.md`
  - 关键治理规则：第六章（非 feature 约束、feature 必填、object/array 完整性、identifier、TypeRef）
  - 规则摘要：B 章第 2 节（2.1~2.5）

目标：
- 对 CSV / Markdown 表格导出的 biz_metadata 进行 Import Gate / Publish Gate 校验
- 覆盖 v1.0 规范：scope、types、identifier、unit、hierarchy、completeness、uniqueness
- 支持 TypeRef：value_type = ref:<code>，并在校验时展开后继续套用原门禁

输入：
- 默认列：tenant_id, version, code, name, description, object_type, parent_code, data_class, value_type, unit, status, source
- 可选列：id, parent_id, created_at, updated_at, deleted_at

输出：
- JSON 报告（violations 列表）
- 退出码：0=通过；2=存在 ERROR；1=运行错误

注意：
- DB 侧约束（UNIQUE/FK/乐观锁等）不在本校验器内执行；本校验器用于“文件/批次入库前”或“发布前”。
- 本工具的 rule_id 以“可读且稳定”为目标；可在后续与规范文档的错误码表做一一映射。
"""

from __future__ import annotations

from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Dict, List, Optional, Sequence, Tuple, Set, Any
import argparse
import csv
import json
import re


# ----------------------------
# Models
# ----------------------------

@dataclass(frozen=True)
class Row:
    """输入行的标准化视图。

    说明：
    - `raw` 保留原始字典（CSV/MD 解析结果），用于调试/定位问题。
    - 其余字段经过 `_norm()` 标准化：去空白、将 "null/none/nan" 视为空字符串等。
    """

    tenant_id: str
    version: int
    code: str
    name: str
    description: str
    object_type: str
    parent_code: str
    data_class: str
    value_type: str
    unit: str
    status: str
    source: str
    raw: Dict[str, Any]


@dataclass(frozen=True)
class Violation:
    """一条门禁违规记录。

    - severity: 严重级别（ERROR/WARN）
    - rule_id: 稳定的规则标识（便于在 CI/门禁系统中做聚合统计）
    - message: 面向人类的解释，尽量包含“为何不合规、如何修复”的信息
    """

    severity: str  # ERROR | WARN
    rule_id: str
    message: str
    tenant_id: str
    code: str
    field: str = ""
    value: str = ""

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


# ----------------------------
# Constants (align with spec)
# ----------------------------

# 规范：A.一/1 + B.2.1（五类核心对象）
OBJECT_TYPES = {"entity", "event", "relation", "document", "feature"}
# 规范：A.四 + B.2.2（feature 专属 data_class）
DATA_CLASSES = {"attribute", "metric", "text", "object", "array", "identifier"}
# 规范：A.3.5（active / deprecated）
STATUS = {"active", "deprecated"}
# 规范：A.3.5（manual / auto_mine / api_sync）
SOURCE = {"manual", "auto_mine", "api_sync"}

# 规范：A.5.1（标量）
SCALAR_TYPES = {"string", "int", "decimal", "boolean", "date", "datetime"}
# 规范：A.5.5（identifier value_type 允许集合）
IDENTIFIER_ALLOWED = {"string", "int", "int|string"}

# code 规范：dot-separated snake_case（示例：company.base.name_cn）
RE_CODE = re.compile(r"^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)*$")
# Union：使用 `|`（允许两侧出现空白字符，例如 "int | string"）
RE_UNION = re.compile(r"^[a-z][a-z0-9_]*(\s*\|\s*[a-z][a-z0-9_]*)+$")
# object：json<object:S>（S 为 schema_ref/命名空间，例如 company.base）
RE_OBJECT = re.compile(r"^json<object:([a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)*)>$")
# array：json<array:T>（T 为标量/实体/Union/object，详见规范 A.5.4；Union 允许空白）
RE_ARRAY = re.compile(
    r"^json<array:([a-z][a-z0-9_]*(\s*\|\s*[a-z][a-z0-9_]*)+|[a-z][a-z0-9_]*|object)>$"
)

# 单个“实体类型名”/“类型标记”（不含 dot），用于 Union / array element 校验。
RE_TYPE_ATOM = re.compile(r"^[a-z][a-z0-9_]*$")
# TypeRef：ref:<code>（规范 A.5.6）
RE_TYPEREF = re.compile(r"^ref:([a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)*)$")

# 规范：A.5.5（identifier 命名约定：... .id.<id_type>）
RE_IDENTIFIER_CODE = re.compile(r"^.+\.id\.[a-z][a-z0-9_]*$")  # *.id.<id_type>


# ----------------------------
# Parsing
# ----------------------------

def _norm(v: Any) -> str:
    """字段规范化：将各种“空值表达”统一成空字符串。

    目的：
    - CSV/MD 里经常出现 "null"/"None"/"NaN" 等文本；本工具将其视为未填写。
    - 统一空白处理可减少下游规则分支。
    """
    if v is None:
        return ""
    s = str(v)
    if s.strip().lower() in {"none", "null", "nan"}:
        return ""
    return s.strip()

def load_rows(paths: Sequence[str]) -> List[Row]:
    """加载多个输入文件并合并为统一 Row 列表。

    - 支持 CSV 与 Markdown（表格）
    - 支持多文件合并：用于“多对象分文件维护”的字典仓库结构
    - 支持目录：会递归扫描目录下的 `.csv` / `.md` / `.markdown` 文件（按路径排序）
    """
    merged: List[Row] = []
    for path in _expand_inputs(paths):
        if path.suffix.lower() == ".csv":
            merged.extend(_load_csv(path))
        elif path.suffix.lower() in {".md", ".markdown"}:
            merged.extend(_load_md_table(path))
        else:
            raise ValueError(f"Unsupported input format: {path}")
    return merged


def _expand_inputs(inputs: Sequence[str]) -> List[Path]:
    """将 CLI 输入展开为文件列表（支持文件与目录）。

    规则：
    - 文件：原样返回（必须存在且为文件）
    - 目录：递归扫描该目录下所有 `.csv` / `.md` / `.markdown` 文件
    - 输出按路径字典序排序，保证门禁结果可复现
    """
    supported = {".csv", ".md", ".markdown"}
    files: List[Path] = []

    for raw in inputs:
        path = Path(raw)
        if not path.exists():
            raise FileNotFoundError(str(path))

        if path.is_dir():
            for child in sorted(path.rglob("*")):
                if child.is_file() and child.suffix.lower() in supported:
                    files.append(child)
            continue

        if path.is_file():
            files.append(path)
            continue

        raise ValueError(f"Unsupported input path: {path}")

    if not files:
        raise ValueError("No supported input files found (expect .csv/.md/.markdown)")

    return sorted(files)

def _load_csv(path: Path) -> List[Row]:
    """读取 CSV（UTF-8 或带 BOM 的 UTF-8-SIG）。"""
    rows: List[Row] = []
    with path.open("r", encoding="utf-8-sig", newline="") as f:
        reader = csv.DictReader(f)
        for raw in reader:
            rows.append(_row_from_raw(raw))
    return rows

def _load_md_table(path: Path) -> List[Row]:
    """读取 Markdown 表格。

    约定：
    - 通过找到包含 `code` 与 `object_type` 的表头行定位表格开始。
    - 跳过分隔行（| --- | --- |）。
    """
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()

    header_idx = None
    header_line = None
    for i, line in enumerate(lines):
        if line.strip().startswith("|") and "code" in line and "object_type" in line:
            header_idx = i
            header_line = line
            break
    if header_idx is None or header_line is None:
        raise ValueError(f"No markdown table found in: {path}")

    headers = [h.strip() for h in header_line.strip().strip("|").split("|")]
    data_lines = []
    for l in lines[header_idx + 2:]:
        if not l.strip().startswith("|"):
            break
        if re.match(r"^\|\s*:?-{2,}", l.strip()):
            continue
        data_lines.append(l)

    rows: List[Row] = []
    for l in data_lines:
        cells = [c.strip() for c in l.strip().strip("|").split("|")]
        if len(cells) != len(headers):
            continue
        raw = dict(zip(headers, cells))
        rows.append(_row_from_raw(raw))
    return rows

def _row_from_raw(raw: Dict[str, Any]) -> Row:
    """将原始行字典转换为 Row。

    字段缺失时会落为 "" 或 0（version），并由后续 `_check_required_and_enums` 给出 ERROR。
    """
    parent_code = _norm(raw.get("parent_code"))
    tenant_id = _norm(raw.get("tenant_id"))
    version = _norm(raw.get("version"))
    code = _norm(raw.get("code"))
    name = _norm(raw.get("name"))
    description = _norm(raw.get("description"))
    object_type = _norm(raw.get("object_type"))
    data_class = _norm(raw.get("data_class"))
    value_type = _norm(raw.get("value_type"))
    unit = _norm(raw.get("unit"))
    status = _norm(raw.get("status"))
    source = _norm(raw.get("source"))

    try:
        version_i = int(version) if version != "" else 0
    except ValueError:
        version_i = 0

    return Row(
        tenant_id=tenant_id,
        version=version_i,
        code=code,
        name=name,
        description=description,
        object_type=object_type,
        parent_code=parent_code,
        data_class=data_class,
        value_type=value_type,
        unit=unit,
        status=status,
        source=source,
        raw=raw,
    )


# ----------------------------
# Type parsing & ref resolution
# ----------------------------

from dataclasses import dataclass as _dataclass  # avoid shadow in docs

@_dataclass(frozen=True)
class ResolvedType:
    """TypeRef 展开结果。

    - raw: 原始 value_type（可能是 ref:xxx）
    - resolved: 解析后的最终 value_type（非 ref 形式）
    - canonical_code: 最终落到的“实体定义 code”（用于错误定位/引用约束）
    - chain: 展开链路（用于循环/深度问题定位）
    """
    raw: str
    resolved: str
    canonical_code: str
    chain: Tuple[str, ...]


class TypeResolver:
    """TypeRef 解析器（ref:<code> → 最终 value_type）。

    规范：A.5.6 + 第六章/6
    - 引用必须可解析、禁止循环、限制展开深度
    - 展开后的最终类型仍要继续执行既有门禁（identifier/unit/object/array 完整性等）
    """

    def __init__(self, rows_by_tenant_code: Dict[Tuple[str, str], Row], max_depth: int = 5):
        self.rows_by_tenant_code = rows_by_tenant_code
        self.max_depth = max_depth

    @staticmethod
    def split_union_terms(expr: str) -> Optional[List[str]]:
        """将 Union 表达式拆分为 term 列表（允许 `|` 两侧存在空白）。"""
        if "|" not in expr:
            return None
        parts = [p.strip() for p in expr.split("|")]
        if any(p == "" for p in parts):
            return None
        return parts

    @staticmethod
    def canonical_union(expr: str) -> str:
        """对 Union 做“仅内部规范化”（不作为强制格式约束）。"""
        terms = TypeResolver.split_union_terms(expr)
        return expr if terms is None else "|".join(terms)

    def _resolve_single_ref(
        self, tenant_id: str, value_type: str
    ) -> Tuple[Optional[ResolvedType], Optional[Violation]]:
        """解析单个 TypeRef（value_type 必须是 `ref:<code>`）。"""
        vt = value_type.strip()
        m = RE_TYPEREF.match(vt)
        if not m:
            return ResolvedType(raw=vt, resolved=vt, canonical_code="", chain=()), None

        target = m.group(1)
        seen: List[str] = []
        current = target
        depth = 0
        while True:
            depth += 1
            if depth > self.max_depth:
                return None, Violation(
                    severity="ERROR",
                    rule_id="TYPE_REF_TOO_DEEP",
                    message=f"type_ref depth exceeded (>{self.max_depth}): {' -> '.join(seen + [current])}",
                    tenant_id=tenant_id,
                    code=current,
                    field="value_type",
                    value=vt,
                )
            if current in seen:
                return None, Violation(
                    severity="ERROR",
                    rule_id="TYPE_REF_CYCLE",
                    message=f"type_ref cycle detected: {' -> '.join(seen + [current])}",
                    tenant_id=tenant_id,
                    code=current,
                    field="value_type",
                    value=vt,
                )
            seen.append(current)

            row = self.rows_by_tenant_code.get((tenant_id, current))
            if row is None:
                return None, Violation(
                    severity="ERROR",
                    rule_id="TYPE_REF_NOT_FOUND",
                    message=f"type_ref target not found: {current}",
                    tenant_id=tenant_id,
                    code=current,
                    field="value_type",
                    value=vt,
                )

            # 规范：TypeRef 目标必须是 feature 且为 active（见 v1.0 规范 A.5.6 / 第六章-6 / B.2.3）
            if row.object_type != "feature":
                return None, Violation(
                    severity="ERROR",
                    rule_id="TYPE_REF_TARGET_NOT_FEATURE",
                    message=f"type_ref target must be object_type=feature: {current} ({row.object_type})",
                    tenant_id=tenant_id,
                    code=current,
                    field="value_type",
                    value=vt,
                )

            if row.status != "active":
                return None, Violation(
                    severity="ERROR",
                    rule_id="TYPE_REF_TARGET_NOT_ACTIVE",
                    message=f"type_ref target must be status=active: {current} ({row.status})",
                    tenant_id=tenant_id,
                    code=current,
                    field="value_type",
                    value=vt,
                )

            inner = row.value_type.strip()
            if inner == "":
                return None, Violation(
                    severity="ERROR",
                    rule_id="TYPE_REF_TARGET_NO_TYPE",
                    message=f"type_ref target has empty value_type: {current}",
                    tenant_id=tenant_id,
                    code=current,
                    field="value_type",
                    value=vt,
                )

            m2 = RE_TYPEREF.match(inner)
            if m2:
                current = m2.group(1)
                continue

            return ResolvedType(raw=vt, resolved=inner, canonical_code=row.code, chain=tuple(seen)), None

        raise AssertionError("unreachable")

    def resolve(self, tenant_id: str, value_type: str) -> Tuple[Optional[ResolvedType], Optional[Violation]]:
        """解析 value_type。

        返回：
        - (ResolvedType, None): 解析成功（含“非 ref”的直接返回）
        - (None, Violation): 解析失败（目标不存在、循环、超深度等）
        - (None, None): value_type 为空，交由上游 scope/required 规则处理
        """
        vt = value_type.strip()
        if vt == "":
            return None, None

        # 1) 单个 TypeRef：ref:<code>
        if RE_TYPEREF.match(vt):
            return self._resolve_single_ref(tenant_id, vt)

        # 2) Union of TypeRef / scalar / entity：ref:a | ref:b
        union_terms = TypeResolver.split_union_terms(vt)
        if union_terms is not None:
            resolved_terms: List[str] = []
            for term in union_terms:
                if RE_TYPEREF.match(term):
                    resolved, vio = self._resolve_single_ref(tenant_id, term)
                    if vio:
                        # 创建新的 Violation 对象，使用原始的 vt
                        return None, Violation(
                            severity=vio.severity,
                            rule_id=vio.rule_id,
                            message=vio.message,
                            tenant_id=vio.tenant_id,
                            code=vio.code,
                            field=vio.field,
                            value=vt,
                        )
                    assert resolved is not None
                    resolved_terms.append(resolved.resolved.strip())
                else:
                    resolved_terms.append(term)

            flattened: List[str] = []
            for term in resolved_terms:
                inner_terms = TypeResolver.split_union_terms(term)
                if inner_terms is None:
                    flattened.append(term.strip())
                else:
                    flattened.extend([t.strip() for t in inner_terms])

            # 去重（保序）：避免解析后出现 "string|string" 影响后续规则判断。
            deduped: List[str] = []
            seen_terms: Set[str] = set()
            for t in flattened:
                t = t.strip()
                if t == "" or t in seen_terms:
                    continue
                seen_terms.add(t)
                deduped.append(t)

            resolved_expr = "|".join(deduped)
            return ResolvedType(
                raw=vt,
                resolved=TypeResolver.canonical_union(resolved_expr),
                canonical_code="",
                chain=(),
            ), None

        # 3) 其他表达：保持原样，但对顶层 Union 做内部规范化（不作为强制格式）
        return ResolvedType(raw=vt, resolved=TypeResolver.canonical_union(vt), canonical_code="", chain=()), None


def is_valid_value_type_expr(vt: str) -> bool:
    """判断 value_type 表达式语法是否符合规范（不做语义完整性校验）。

    规范：B.2.3
    - 标量 / Union / object / array / TypeRef
    - 注意：此处仅做语法层面判断；TypeRef 的存在性/循环/深度由 `TypeResolver` 处理。
    """
    vt = vt.strip()
    vt_canon = TypeResolver.canonical_union(vt)

    if vt_canon in SCALAR_TYPES:
        return True
    if vt_canon in IDENTIFIER_ALLOWED:
        return True
    if RE_OBJECT.match(vt):
        return True
    if RE_ARRAY.match(vt):
        return True
    if RE_TYPEREF.match(vt):
        return True
    # Union：支持 term 为 scalar / entity(atom) / TypeRef，允许空白
    if "|" in vt:
        terms = TypeResolver.split_union_terms(vt)
        if terms is None:
            return False
        for term in terms:
            t = term.strip()
            if t in SCALAR_TYPES:
                continue
            if RE_TYPE_ATOM.match(t):
                continue
            if RE_TYPEREF.match(t):
                continue
            return False
        return True
    return False


# ----------------------------
# Linter
# ----------------------------

class BizMetadataLinter:
    def __init__(self, rows: Sequence[Row], mode: str = "import", max_ref_depth: int = 5):
        """构造门禁校验器。

        参数：
        - mode:
          - import: 入库前门禁（更偏向“结构与字段合法性”，不要求所有结构完整性）
          - publish: 发布前门禁（更偏向“语义完整性”，会额外校验 object/array 完整性与 ref 目标状态）
        - max_ref_depth: TypeRef 最大展开深度（规范建议 5，可通过 CLI 覆盖）
        """
        if mode not in {"import", "publish"}:
            raise ValueError("mode must be one of: import, publish")
        self.mode = mode
        self.rows = list(rows)

        self.rows_by_tenant_code: Dict[Tuple[str, str], Row] = {}
        for r in self.rows:
            if r.tenant_id and r.code:
                self.rows_by_tenant_code[(r.tenant_id, r.code)] = r

        self.resolver = TypeResolver(self.rows_by_tenant_code, max_depth=max_ref_depth)

    def lint(self) -> List[Violation]:
        """执行全量规则校验并返回违规列表。"""
        vios: List[Violation] = []
        vios.extend(self._check_required_and_enums())
        vios.extend(self._check_code_format())
        vios.extend(self._check_uniqueness())
        vios.extend(self._check_scope_and_feature_fields())
        vios.extend(self._check_types_and_ref())
        vios.extend(self._check_unit_rules())
        vios.extend(self._check_identifier_rules())
        vios.extend(self._check_hierarchy())
        if self.mode == "publish":
            vios.extend(self._check_completeness())
        return vios

    def _check_required_and_enums(self) -> List[Violation]:
        """基础必填与枚举合法性校验。

        规范：A.3.x 字段字典 + B.3.1（NOT NULL / ENUM）
        - required: tenant_id/code/name/object_type/status/source/version
        - enum: object_type/status/source/data_class（data_class 允许空，由 scope 规则进一步约束）
        """
        out: List[Violation] = []
        for r in self.rows:
            for field in ["tenant_id", "code", "name", "object_type", "status", "source"]:
                if getattr(r, field) == "":
                    out.append(Violation("ERROR", "BASIC_REQUIRED_MISSING", f"missing required field: {field}",
                                         r.tenant_id, r.code, field, getattr(r, field)))
            if r.version <= 0:
                out.append(Violation("ERROR", "BASIC_VERSION_INVALID", "version must be positive integer",
                                     r.tenant_id, r.code, "version", str(r.version)))

            if r.object_type and r.object_type not in OBJECT_TYPES:
                out.append(Violation("ERROR", "ENUM_OBJECT_TYPE", f"invalid object_type: {r.object_type}",
                                     r.tenant_id, r.code, "object_type", r.object_type))
            if r.status and r.status not in STATUS:
                out.append(Violation("ERROR", "ENUM_STATUS", f"invalid status: {r.status}",
                                     r.tenant_id, r.code, "status", r.status))
            if r.source and r.source not in SOURCE:
                out.append(Violation("ERROR", "ENUM_SOURCE", f"invalid source: {r.source}",
                                     r.tenant_id, r.code, "source", r.source))

            if r.data_class and r.data_class not in DATA_CLASSES:
                out.append(Violation("ERROR", "ENUM_DATA_CLASS", f"invalid data_class: {r.data_class}",
                                     r.tenant_id, r.code, "data_class", r.data_class))
        return out

    def _check_code_format(self) -> List[Violation]:
        """code 命名格式校验（dot-separated snake_case）。

        规范：A.3.2（语义路径 code）
        """
        out: List[Violation] = []
        for r in self.rows:
            if r.code and not RE_CODE.match(r.code):
                out.append(Violation("ERROR", "CODE_FORMAT", "code must be dot-separated snake_case",
                                     r.tenant_id, r.code, "code", r.code))
        return out

    def _check_uniqueness(self) -> List[Violation]:
        """输入批次内的唯一性校验：同一 tenant 下 code 不得重复。

        规范：A.3.1 + B.3.3（UNIQUE (tenant_id, code)）
        - DB 侧通常还会加 `WHERE deleted_at IS NULL`，本工具只针对“当前输入批次”做去重。
        """
        out: List[Violation] = []
        seen: Set[Tuple[str, str]] = set()
        for r in self.rows:
            key = (r.tenant_id, r.code)
            if r.tenant_id and r.code:
                if key in seen:
                    out.append(Violation("ERROR", "UNIQUE_TENANT_CODE",
                                         "duplicate (tenant_id, code) in input batch",
                                         r.tenant_id, r.code, "code", r.code))
                else:
                    seen.add(key)
        return out

    def _check_scope_and_feature_fields(self) -> List[Violation]:
        """对象作用域（scope）门禁：只有 feature 才能携带类型字段。

        规范：A.一/2 + 第六章/1~2 + B.2.1
        - object_type != feature => data_class/value_type/unit 必须为空
        - object_type = feature  => data_class/value_type 必须非空
        """
        out: List[Violation] = []
        for r in self.rows:
            if r.object_type != "feature":
                if r.data_class or r.value_type or r.unit:
                    out.append(Violation("ERROR", "SCOPE_NON_FEATURE_HAS_TYPE",
                                         "object_type != feature must keep data_class/value_type/unit empty",
                                         r.tenant_id, r.code, "data_class/value_type/unit",
                                         f"{r.data_class}|{r.value_type}|{r.unit}"))
            else:
                if not r.data_class or not r.value_type:
                    out.append(Violation("ERROR", "SCOPE_FEATURE_MISSING_TYPE",
                                         "object_type=feature must have non-empty data_class and value_type",
                                         r.tenant_id, r.code, "data_class/value_type",
                                         f"{r.data_class}|{r.value_type}"))
        return out

    def _check_types_and_ref(self) -> List[Violation]:
        """value_type 表达式语法与 TypeRef 可解析性校验。

        规范：A.五 + A.5.6 + 第六章/6 + B.2.3
        - 先检查 value_type 字符串表达式是否符合允许的语法集合
        - 若为 TypeRef，则进行解析（存在性/循环/深度），并对解析后的最终 value_type 再做一次语法校验
        """
        out: List[Violation] = []
        for r in self.rows:
            if r.object_type != "feature":
                continue
            if r.value_type and not is_valid_value_type_expr(r.value_type):
                out.append(Violation("ERROR", "TYPE_SYNTAX_INVALID",
                                     f"invalid value_type expression: {r.value_type}",
                                     r.tenant_id, r.code, "value_type", r.value_type))
                continue

            resolved, vio = self.resolver.resolve(r.tenant_id, r.value_type)
            if vio:
                out.append(Violation(vio.severity, vio.rule_id, vio.message, r.tenant_id, r.code, vio.field, vio.value))
                continue
            if resolved and not is_valid_value_type_expr(resolved.resolved):
                out.append(Violation("ERROR", "TYPE_REF_RESOLVED_INVALID",
                                     f"resolved value_type is invalid: {resolved.resolved}",
                                     r.tenant_id, r.code, "value_type", r.value_type))
        return out

    def _check_unit_rules(self) -> List[Violation]:
        """unit 门禁：仅 metric 可填写 unit。

        规范：A.一/3 + B.2.5
        - identifier/text/object/array 通常应为空；对 identifier 本工具在 `_check_identifier_rules` 中强制为空。
        """
        out: List[Violation] = []
        for r in self.rows:
            if r.object_type != "feature":
                continue
            if r.unit and r.data_class != "metric":
                out.append(Violation("ERROR", "UNIT_NOT_ALLOWED",
                                     "unit can be filled only when data_class=metric",
                                     r.tenant_id, r.code, "unit", r.unit))
        return out

    def _check_identifier_rules(self) -> List[Violation]:
        """identifier 门禁（强约束）。

        规范：A.5.5 + 第六章/5 + B.2.4
        - value_type（解析 ref 后）必须为 {string,int,int|string}
        - unit 必须为空
        - code 命名必须符合 *.id.<id_type>
        """
        out: List[Violation] = []
        for r in self.rows:
            if r.object_type != "feature" or r.data_class != "identifier":
                continue

            resolved, vio = self.resolver.resolve(r.tenant_id, r.value_type)
            if vio:
                out.append(Violation(vio.severity, vio.rule_id, vio.message, r.tenant_id, r.code, "value_type", r.value_type))
                continue
            resolved_vt = resolved.resolved if resolved else r.value_type
            resolved_vt = TypeResolver.canonical_union(resolved_vt.strip())
            # identifier 的允许集合对 Union 不区分顺序，统一做一次规范化：
            # - "string|string" -> "string"
            # - "string|int" / "int|string" -> "int|string"
            union_terms = TypeResolver.split_union_terms(resolved_vt)
            if union_terms is not None:
                term_set = {t.strip() for t in union_terms}
                if term_set.issubset({"int", "string"}) and term_set:
                    if term_set == {"int", "string"}:
                        resolved_vt = "int|string"
                    elif term_set == {"int"}:
                        resolved_vt = "int"
                    elif term_set == {"string"}:
                        resolved_vt = "string"

            if resolved_vt not in IDENTIFIER_ALLOWED:
                out.append(Violation("ERROR", "IDENTIFIER_VALUE_TYPE",
                                     f"identifier value_type must be one of {sorted(IDENTIFIER_ALLOWED)} (after ref resolution)",
                                     r.tenant_id, r.code, "value_type", r.value_type))
            if r.unit:
                out.append(Violation("ERROR", "IDENTIFIER_UNIT_NOT_EMPTY",
                                     "identifier unit must be empty",
                                     r.tenant_id, r.code, "unit", r.unit))
            if not RE_IDENTIFIER_CODE.match(r.code):
                out.append(Violation("ERROR", "IDENTIFIER_CODE_PATTERN",
                                     "identifier code must match *.id.<id_type>",
                                     r.tenant_id, r.code, "code", r.code))
        return out

    def _check_hierarchy(self) -> List[Violation]:
        """层级一致性：parent_code 必须存在且必须为 code 前缀。

        规范：A.3.3（层级组织）
        - 规范中 DB 存 parent_id，但很多数据源以 parent_code 形式表达；此处按 parent_code 做静态校验。
        """
        out: List[Violation] = []
        for r in self.rows:
            if not r.parent_code:
                continue
            parent = self.rows_by_tenant_code.get((r.tenant_id, r.parent_code))
            if parent is None:
                out.append(Violation("ERROR", "HIERARCHY_PARENT_MISSING",
                                     f"parent_code not found in same tenant: {r.parent_code}",
                                     r.tenant_id, r.code, "parent_code", r.parent_code))
                continue
            if not r.code.startswith(r.parent_code + "."):
                out.append(Violation("ERROR", "HIERARCHY_PARENT_PREFIX",
                                     "child code must start with parent_code + '.'",
                                     r.tenant_id, r.code, "parent_code", r.parent_code))
        return out

    def _check_completeness(self) -> List[Violation]:
        """发布前完整性校验（Publish Gate）。

        规范：第六章/3~4 + B.3（建议仅作为发布门禁）
        - object：data_class=object 且 value_type=json<object:S> => 必须存在 S.* 子字段
        - array：value_type=json<array:object> => 必须存在 xxx.item.* 子字段
        """
        out: List[Violation] = []
        tenant_codes: Dict[str, Set[str]] = {}
        for r in self.rows:
            tenant_codes.setdefault(r.tenant_id, set()).add(r.code)

        for r in self.rows:
            if r.object_type != "feature":
                continue

            # 完整性校验仅对 object/array 生效：
            # - object：由 data_class=object 决定是否启用
            # - array(object)：仅当 data_class=array 时才可能触发（避免对 attribute 等无关字段重复报错）
            needs_object_check = r.data_class == "object"
            needs_array_check = r.data_class == "array"
            if not (needs_object_check or needs_array_check):
                continue

            resolved, vio = self.resolver.resolve(r.tenant_id, r.value_type)
            if vio:
                out.append(
                    Violation(
                        vio.severity,
                        vio.rule_id,
                        vio.message,
                        r.tenant_id,
                        r.code,
                        "value_type",
                        r.value_type,
                    )
                )
                continue
            vt = resolved.resolved if resolved else r.value_type

            if needs_object_check:
                m = RE_OBJECT.match(vt)
                if m:
                    schema_ref = m.group(1)
                    exists_child = any(
                        c.startswith(schema_ref + ".")
                        for c in tenant_codes.get(r.tenant_id, set())
                    )
                    if not exists_child:
                        out.append(
                            Violation(
                                "ERROR",
                                "COMPLETENESS_OBJECT_CHILDREN_MISSING",
                                f"object schema_ref {schema_ref} must have S.* child fields",
                                r.tenant_id,
                                r.code,
                                "value_type",
                                r.value_type,
                            )
                        )

            if needs_array_check and vt == "json<array:object>":
                prefix = r.code + ".item."
                exists_item = any(
                    c.startswith(prefix) for c in tenant_codes.get(r.tenant_id, set())
                )
                if not exists_item:
                    out.append(
                        Violation(
                            "ERROR",
                            "COMPLETENESS_ARRAY_OBJECT_ITEMS_MISSING",
                            "json<array:object> must have xxx.item.* child fields",
                            r.tenant_id,
                            r.code,
                            "value_type",
                            r.value_type,
                        )
                    )
        return out


# ----------------------------
# CLI
# ----------------------------

def main() -> int:
    """CLI 入口。

    用途：
    - import: 入库前门禁（适合 CI/门禁）
    - publish: 发布前门禁（对 object/array 完整性更严格）
    """
    ap = argparse.ArgumentParser(description="biz_metadata linter (import/publish gate) with TypeRef support")
    ap.add_argument(
        "--input",
        "-i",
        nargs="+",
        required=True,
        help="Input files or directories (.csv/.md/.markdown). Support multiple; directories are scanned recursively.",
    )
    ap.add_argument("--mode", choices=["import", "publish"], default="import", help="Gate mode")
    ap.add_argument("--max-ref-depth", type=int, default=5, help="Max TypeRef chain depth")
    ap.add_argument("--report", "-o", default="", help="Output JSON report path; if empty, print to stdout")
    ap.add_argument("--fail-on-warn", action="store_true", help="Exit non-zero if WARN exists")
    args = ap.parse_args()

    resolved_inputs = [str(p) for p in _expand_inputs(args.input)]
    rows = load_rows(resolved_inputs)
    linter = BizMetadataLinter(rows, mode=args.mode, max_ref_depth=args.max_ref_depth)
    violations = linter.lint()

    report = {
        "mode": args.mode,
        # 用户传入的参数（可能包含目录）
        "inputs": args.input,
        # 实际参与校验的文件列表（目录会被递归展开）
        "resolved_inputs": resolved_inputs,
        # 解析得到的数据行数（CSV 行 + MD 表格行）
        "row_count": len(rows),
        # 违规总数（= errors + warns），即 violations 列表长度
        "violation_count": len(violations),
        "error_count": sum(1 for v in violations if v.severity == "ERROR"),
        "warn_count": sum(1 for v in violations if v.severity == "WARN"),
        "violations": [v.to_dict() for v in violations],
    }

    if args.report:
        Path(args.report).write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
    else:
        print(json.dumps(report, ensure_ascii=False, indent=2))

    if report["error_count"] > 0:
        return 2
    if args.fail_on_warn and report["warn_count"] > 0:
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
