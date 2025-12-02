//! 轻量表达式 DSL，用于跨仓储的筛选、排序和分页需求。

use std::fmt::Debug;

/// 基础的筛选值类型，覆盖常见标量场景。
#[derive(Clone, Debug, PartialEq)]
pub enum FilterValue {
    String(String),
    I64(i64),
    F64(f64),
    Bool(bool),
}

impl FilterValue {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            FilterValue::I64(v) => Some(*v),
            FilterValue::F64(v) => Some(*v as i64),
            FilterValue::String(v) => v.parse().ok(),
            FilterValue::Bool(_) => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            FilterValue::String(v) => Some(v.clone()),
            FilterValue::I64(v) => Some(v.to_string()),
            FilterValue::F64(v) => Some(v.to_string()),
            FilterValue::Bool(v) => Some(v.to_string()),
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FilterValue::Bool(v) => Some(*v),
            FilterValue::String(v) => v.parse().ok(),
            FilterValue::I64(_) | FilterValue::F64(_) => None,
        }
    }
}

impl From<&str> for FilterValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for FilterValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i64> for FilterValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<f64> for FilterValue {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl From<bool> for FilterValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// 单字段条件表达式。
#[derive(Clone, Debug, PartialEq)]
pub enum Comparison {
    Eq {
        field: String,
        value: FilterValue,
    },
    Ne {
        field: String,
        value: FilterValue,
    },
    Gt {
        field: String,
        value: FilterValue,
    },
    Ge {
        field: String,
        value: FilterValue,
    },
    Lt {
        field: String,
        value: FilterValue,
    },
    Le {
        field: String,
        value: FilterValue,
    },
    Between {
        field: String,
        start: FilterValue,
        end: FilterValue,
    },
    In {
        field: String,
        values: Vec<FilterValue>,
    },
    Contains {
        field: String,
        value: FilterValue,
    },
}

/// 组合表达式，支持 AND / OR / NOT。
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Comparison(Comparison),
    And(Vec<Expression>),
    Or(Vec<Expression>),
    Not(Box<Expression>),
    True,
    False,
}

impl Expression {
    /// 按逻辑与组合多个表达式；若集合为空则直接返回恒真。
    pub fn and<T: Into<Vec<Expression>>>(exprs: T) -> Self {
        let list = exprs.into();
        if list.is_empty() {
            return Expression::True;
        }
        Expression::And(list)
    }

    /// 按逻辑或组合多个表达式；若集合为空则返回恒假。
    pub fn or<T: Into<Vec<Expression>>>(exprs: T) -> Self {
        let list = exprs.into();
        if list.is_empty() {
            return Expression::False;
        }
        Expression::Or(list)
    }

    /// 对单个表达式取逻辑非。
    pub fn negate(expr: Expression) -> Self {
        Expression::Not(Box::new(expr))
    }

    /// 将原子比较包装为表达式节点，便于链式构造。
    pub fn cmp(comparison: Comparison) -> Self {
        Expression::Comparison(comparison)
    }
}

/// 排序方向。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// 排序字段表达式。
#[derive(Clone, Debug, PartialEq)]
pub struct OrderBy {
    pub field: String,
    pub direction: SortDirection,
}

impl OrderBy {
    pub fn asc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: SortDirection::Asc,
        }
    }

    pub fn desc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: SortDirection::Desc,
        }
    }
}

/// 查询附加选项：分页与排序。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct QueryOptions {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub order_bys: Vec<OrderBy>,
}

impl QueryOptions {
    /// 创建一个新的查询选项，可指定 limit/offset。
    pub fn new(limit: Option<u64>, offset: Option<u64>) -> Self {
        Self {
            limit,
            offset,
            order_bys: Vec::new(),
        }
    }

    /// 附加一个排序字段，支持链式调用。
    pub fn with_order_by(mut self, order: OrderBy) -> Self {
        self.order_bys.push(order);
        self
    }
}

/// 构造一个字段等于目标值的比较表达式。
pub fn eq(field: impl Into<String>, value: impl Into<FilterValue>) -> Comparison {
    Comparison::Eq {
        field: field.into(),
        value: value.into(),
    }
}

/// 构造一个字段不等于目标值的比较表达式。
pub fn ne(field: impl Into<String>, value: impl Into<FilterValue>) -> Comparison {
    Comparison::Ne {
        field: field.into(),
        value: value.into(),
    }
}

/// 构造一个字段大于目标值的比较表达式。
pub fn gt(field: impl Into<String>, value: impl Into<FilterValue>) -> Comparison {
    Comparison::Gt {
        field: field.into(),
        value: value.into(),
    }
}

/// 构造一个字段大于等于目标值的比较表达式。
pub fn ge(field: impl Into<String>, value: impl Into<FilterValue>) -> Comparison {
    Comparison::Ge {
        field: field.into(),
        value: value.into(),
    }
}

/// 构造一个字段小于目标值的比较表达式。
pub fn lt(field: impl Into<String>, value: impl Into<FilterValue>) -> Comparison {
    Comparison::Lt {
        field: field.into(),
        value: value.into(),
    }
}

/// 构造一个字段小于等于目标值的比较表达式。
pub fn le(field: impl Into<String>, value: impl Into<FilterValue>) -> Comparison {
    Comparison::Le {
        field: field.into(),
        value: value.into(),
    }
}

/// 构造一个字段值在给定集合中的比较表达式。
pub fn r#in(field: impl Into<String>, values: Vec<impl Into<FilterValue>>) -> Comparison {
    Comparison::In {
        field: field.into(),
        values: values.into_iter().map(Into::into).collect(),
    }
}

/// 构造一个字段包含目标值的比较表达式。
pub fn contains(field: impl Into<String>, value: impl Into<FilterValue>) -> Comparison {
    Comparison::Contains {
        field: field.into(),
        value: value.into(),
    }
}

/// 构造一个字段处于闭区间 [start, end] 的比较表达式。
pub fn between(
    field: impl Into<String>,
    start: impl Into<FilterValue>,
    end: impl Into<FilterValue>,
) -> Comparison {
    Comparison::Between {
        field: field.into(),
        start: start.into(),
        end: end.into(),
    }
}

impl std::ops::Not for Expression {
    type Output = Expression;

    fn not(self) -> Self::Output {
        Expression::Not(Box::new(self))
    }
}
