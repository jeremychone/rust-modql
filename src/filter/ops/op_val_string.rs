#![allow(deprecated)] // for now

use crate::filter::OpVal;

#[derive(Debug, Clone)]
pub struct OpValsString(pub Vec<OpValString>);

#[derive(Debug, Clone)]
pub enum OpValString {
	Eq(String),
	Not(String),

	In(Vec<String>),
	NotIn(Vec<String>),

	Lt(String),
	Lte(String),

	Gt(String),
	Gte(String),

	Contains(String),
	NotContains(String),

	ContainsAny(Vec<String>),
	NotContainsAny(Vec<String>),

	ContainsAll(Vec<String>),

	StartsWith(String),
	NotStartsWith(String),

	StartsWithAny(Vec<String>),
	NotStartsWithAny(Vec<String>),

	EndsWith(String),
	NotEndsWith(String),

	EndsWithAny(Vec<String>),
	NotEndsWithAny(Vec<String>),

	Empty(bool),
	Null(bool),

	ContainsCi(String),
	NotContainsCi(String),

	StartsWithCi(String),
	NotStartsWithCi(String),

	EndsWithCi(String),
	NotEndsWithCi(String),

	Ilike(String),
}

// region:    --- Simple value to Eq OpValString
impl From<String> for OpValString {
	fn from(val: String) -> Self {
		OpValString::Eq(val)
	}
}

impl From<&str> for OpValString {
	fn from(val: &str) -> Self {
		OpValString::Eq(val.to_string())
	}
}
// endregion: --- Simple value to Eq OpValString

// region:    --- Simple value to Eq OpValStrings
impl From<String> for OpValsString {
	fn from(val: String) -> Self {
		OpValString::from(val).into()
	}
}

impl From<&str> for OpValsString {
	fn from(val: &str) -> Self {
		OpValString::from(val).into()
	}
}
// endregion: --- Simple value to Eq OpValStrings

// region:    --- StringOpVal to OpVal
impl From<OpValString> for OpVal {
	fn from(val: OpValString) -> Self {
		OpVal::String(val)
	}
}
// endregion: --- StringOpVal to OpVal

// region:    --- Primitive to OpVal::String(StringOpVal::Eq)
impl From<String> for OpVal {
	fn from(val: String) -> Self {
		OpValString::Eq(val).into()
	}
}

impl From<&str> for OpVal {
	fn from(val: &str) -> Self {
		OpValString::Eq(val.to_string()).into()
	}
}
// endregion: --- Primitive to OpVal::String(StringOpVal::Eq)

mod json {
	use crate::filter::json::OpValueToOpValType;
	use crate::filter::OpValString;
	use crate::{Error, Result};
	use serde_json::Value;

	impl OpValueToOpValType for OpValString {
		fn op_value_to_op_val_type(op: &str, value: Value) -> Result<Self>
		where
			Self: Sized,
		{
			fn into_strings(value: Value) -> Result<Vec<String>> {
				let mut values = Vec::new();

				let Value::Array(array) = value else {
					return Err(Error::JsonValArrayWrongType { actual_value: value });
				};

				for item in array.into_iter() {
					if let Value::String(item) = item {
						values.push(item);
					} else {
						return Err(Error::JsonValArrayItemNotOfType {
							expected_type: "String",
							actual_value: item,
						});
					}
				}

				Ok(values)
			}

			let ov = match (op, value) {
				("$eq", Value::String(string_v)) => OpValString::Eq(string_v),
				("$in", value) => OpValString::In(into_strings(value)?),

				("$not", Value::String(string_v)) => OpValString::Not(string_v),
				("$notIn", value) => OpValString::NotIn(into_strings(value)?),

				("$lt", Value::String(string_v)) => OpValString::Lt(string_v),
				("$lte", Value::String(string_v)) => OpValString::Lte(string_v),

				("$gt", Value::String(string_v)) => OpValString::Gt(string_v),
				("$gte", Value::String(string_v)) => OpValString::Gte(string_v),

				("$contains", Value::String(string_v)) => OpValString::Contains(string_v),
				("$containsAny", value) => OpValString::ContainsAny(into_strings(value)?),

				("$containsAll", value) => OpValString::ContainsAll(into_strings(value)?),

				("$notContains", Value::String(string_v)) => OpValString::NotContains(string_v),
				("$notContainsAny", value) => OpValString::NotContainsAny(into_strings(value)?),

				("$startsWith", Value::String(string_v)) => OpValString::StartsWith(string_v),
				("$startsWithAny", value) => OpValString::StartsWithAny(into_strings(value)?),

				("$notStartsWith", Value::String(string_v)) => OpValString::NotStartsWith(string_v),
				("$notStartsWithAny", value) => OpValString::NotStartsWithAny(into_strings(value)?),

				("$endsWith", Value::String(string_v)) => OpValString::EndsWith(string_v),
				("$endsWithAny", value) => OpValString::EndsWithAny(into_strings(value)?),

				("$notEndsWith", Value::String(string_v)) => OpValString::NotEndsWith(string_v),
				("$notEndsWithAny", value) => OpValString::NotEndsWithAny(into_strings(value)?),

				("$empty", Value::Bool(v)) => OpValString::Empty(v),
				("$null", Value::Bool(v)) => OpValString::Null(v),

				("$containsCi", Value::String(string_v)) => OpValString::ContainsCi(string_v),
				("$notContainsCi", Value::String(string_v)) => OpValString::NotContainsCi(string_v),

				("$startsWithCi", Value::String(string_v)) => OpValString::StartsWithCi(string_v),
				("$notStartsWithCi", Value::String(string_v)) => OpValString::NotStartsWithCi(string_v),

				("$endsWithCi", Value::String(string_v)) => OpValString::EndsWithCi(string_v),
				("$notEndsWithCi", Value::String(string_v)) => OpValString::NotEndsWithCi(string_v),

				// Postgres optimized case insensitive like
				("$ilike", Value::String(string_v)) => OpValString::Ilike(string_v),

				(_, v) => {
					return Err(Error::JsonOpValNotSupported {
						operator: op.to_string(),
						value: v,
					})
				}
			};
			Ok(ov)
		}
	}
}

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use crate::filter::{sea_is_col_value_null, FilterNodeOptions, SeaResult};
	use crate::{into_node_column_expr, into_node_value_expr};
	use sea_query::{BinOper, ColumnRef, Condition, ConditionExpression, Expr, Func, SimpleExpr};

	#[cfg(feature = "with-ilike")]
	use sea_query::extension::postgres::PgBinOper;

	impl OpValString {
		pub fn into_sea_cond_expr(
			self,
			col: &ColumnRef,
			node_options: &FilterNodeOptions,
		) -> SeaResult<ConditionExpression> {
			let binary_fn = |op: BinOper, v: String| {
				let vxpr = into_node_value_expr(v, node_options);
				let column = into_node_column_expr(col.clone(), node_options);
				ConditionExpression::SimpleExpr(SimpleExpr::binary(column.into(), op, vxpr))
			};

			#[cfg(feature = "with-ilike")]
			let pg_binary_fn = |op: PgBinOper, v: String| {
				let vxpr = into_node_value_expr(v, node_options);
				let column = into_node_column_expr(col.clone(), node_options);
				ConditionExpression::SimpleExpr(SimpleExpr::binary(column.into(), BinOper::PgOperator(op), vxpr))
			};

			let binaries_fn = |op: BinOper, v: Vec<String>| {
				let vxpr_list: Vec<SimpleExpr> = v.into_iter().map(|v| into_node_value_expr(v, node_options)).collect();
				let vxpr = SimpleExpr::Tuple(vxpr_list);
				let column = into_node_column_expr(col.clone(), node_options);
				ConditionExpression::SimpleExpr(SimpleExpr::binary(column.into(), op, vxpr))
			};

			let cond_any_of_fn = |op: BinOper, values: Vec<String>, val_prefix: &str, val_suffix: &str| {
				let mut cond = Condition::any();

				for value in values {
					let expr = binary_fn(op, format!("{val_prefix}{value}{val_suffix}"));
					cond = cond.add(expr);
				}

				ConditionExpression::Condition(cond)
			};

			let case_insensitive_fn = |op: BinOper, v: String| {
				let vxpr = SimpleExpr::Value(v.into());
				let col_expr = SimpleExpr::FunctionCall(Func::lower(Expr::col(col.clone())).into());
				let value_expr = SimpleExpr::FunctionCall(Func::lower(vxpr).into());
				ConditionExpression::SimpleExpr(SimpleExpr::binary(col_expr, op, value_expr))
			};

			let cond = match self {
				OpValString::Eq(s) => binary_fn(BinOper::Equal, s),
				OpValString::Not(s) => binary_fn(BinOper::NotEqual, s),
				OpValString::In(s) => binaries_fn(BinOper::In, s),
				OpValString::NotIn(s) => binaries_fn(BinOper::NotIn, s),
				OpValString::Lt(s) => binary_fn(BinOper::SmallerThan, s),
				OpValString::Lte(s) => binary_fn(BinOper::SmallerThanOrEqual, s),
				OpValString::Gt(s) => binary_fn(BinOper::GreaterThan, s),
				OpValString::Gte(s) => binary_fn(BinOper::GreaterThanOrEqual, s),

				OpValString::Contains(s) => binary_fn(BinOper::Like, format!("%{s}%")),

				OpValString::NotContains(s) => binary_fn(BinOper::NotLike, format!("%{s}%")),

				OpValString::ContainsAll(values) => {
					let mut cond = Condition::all();

					for value in values {
						let expr = binary_fn(BinOper::Like, format!("%{value}%"));
						cond = cond.add(expr);
					}

					ConditionExpression::Condition(cond)
				}

				OpValString::ContainsAny(values) => cond_any_of_fn(BinOper::Like, values, "%", "%"),
				OpValString::NotContainsAny(values) => cond_any_of_fn(BinOper::NotLike, values, "%", "%"),

				OpValString::StartsWith(s) => binary_fn(BinOper::Like, format!("{s}%")),
				OpValString::StartsWithAny(values) => cond_any_of_fn(BinOper::Like, values, "", "%"),

				OpValString::NotStartsWith(s) => binary_fn(BinOper::NotLike, format!("{s}%")),
				OpValString::NotStartsWithAny(values) => cond_any_of_fn(BinOper::NotLike, values, "", "%"),

				OpValString::EndsWith(s) => binary_fn(BinOper::Like, format!("%{s}")),
				OpValString::EndsWithAny(values) => cond_any_of_fn(BinOper::Like, values, "%", ""),

				OpValString::NotEndsWith(s) => binary_fn(BinOper::Like, format!("%{s}")),
				OpValString::NotEndsWithAny(values) => cond_any_of_fn(BinOper::NotLike, values, "%", ""),

				OpValString::Null(null) => sea_is_col_value_null(col.clone(), null),
				OpValString::Empty(empty) => {
					let op = if empty { BinOper::Equal } else { BinOper::NotEqual };
					Condition::any()
						.add(sea_is_col_value_null(col.clone(), empty))
						.add(binary_fn(op, "".to_string()))
						.into()
				}

				OpValString::ContainsCi(s) => case_insensitive_fn(BinOper::Like, format!("%{s}%")),
				OpValString::NotContainsCi(s) => case_insensitive_fn(BinOper::NotLike, format!("%{s}%")),

				OpValString::StartsWithCi(s) => case_insensitive_fn(BinOper::Like, format!("{s}%")),
				OpValString::NotStartsWithCi(s) => case_insensitive_fn(BinOper::NotLike, format!("{s}%")),

				OpValString::EndsWithCi(s) => case_insensitive_fn(BinOper::Like, format!("%{s}")),
				OpValString::NotEndsWithCi(s) => case_insensitive_fn(BinOper::NotLike, format!("%{s}")),

				OpValString::Ilike(s) => {
					#[cfg(feature = "with-ilike")]
					{
						pg_binary_fn(PgBinOper::ILike, format!("%{s}%"))
					}
					#[cfg(not(feature = "with-ilike"))]
					{
						case_insensitive_fn(BinOper::Like, format!("%{s}%"))
					}
				}
			};

			Ok(cond)
		}
	}
}
// endregion: --- with-sea-query
