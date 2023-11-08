use crate::filter::OpVal;

#[derive(Debug)]
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

	ContainsIn(Vec<String>),
	NotContainsIn(Vec<String>),

	StartsWith(String),
	NotStartsWith(String),

	StartsWithIn(Vec<String>),
	NotStartsWithIn(Vec<String>),

	EndsWith(String),
	NotEndsWith(String),

	EndsWithIn(Vec<String>),
	NotEndsWithIn(Vec<String>),

	Empty(bool),
	Null(bool),
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

			// FIXME: Needs to do the In/Array patterns.
			let ov = match (op, value) {
				("$eq", Value::String(string_v)) => OpValString::Eq(string_v),
				("$in", value) => OpValString::NotIn(into_strings(value)?),

				("$not", Value::String(string_v)) => OpValString::Not(string_v),
				("$notIn", value) => OpValString::NotIn(into_strings(value)?),

				("$lt", Value::String(string_v)) => OpValString::Lt(string_v),
				("$lte", Value::String(string_v)) => OpValString::Lte(string_v),

				("$gt", Value::String(string_v)) => OpValString::Gt(string_v),
				("$gte", Value::String(string_v)) => OpValString::Gte(string_v),

				("$contains", Value::String(string_v)) => OpValString::Contains(string_v),
				("$containsIn", value) => OpValString::ContainsIn(into_strings(value)?),

				("$notContains", Value::String(string_v)) => OpValString::NotContains(string_v),
				("$notContainsIn", value) => OpValString::NotContainsIn(into_strings(value)?),

				("$startsWith", Value::String(string_v)) => OpValString::StartsWith(string_v),
				("$startsWithIn", value) => OpValString::StartsWithIn(into_strings(value)?),

				("$notStartsWith", Value::String(string_v)) => OpValString::NotStartsWith(string_v),
				("$notStartsWithIn", value) => OpValString::NotStartsWithIn(into_strings(value)?),

				("$endsWith", Value::String(string_v)) => OpValString::EndsWith(string_v),
				("$endsWithIn", value) => OpValString::EndsWithIn(into_strings(value)?),

				("$notEndsWith", Value::String(string_v)) => OpValString::NotEndsWith(string_v),
				("$notEndsWithIn", value) => OpValString::NotEndsWithIn(into_strings(value)?),

				("$empty", Value::Bool(v)) => OpValString::Empty(v),
				("$null", Value::Bool(v)) => OpValString::Null(v),

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
	use crate::filter::{sea_is_col_value_null, SeaResult};
	use sea_query::{BinOper, ColumnRef, Condition, ConditionExpression, SimpleExpr, Value};

	impl OpValString {
		pub fn into_sea_cond_expr(self, col: &ColumnRef) -> SeaResult<ConditionExpression> {
			let binary_fn = |op: BinOper, v: String| {
				let vxpr = SimpleExpr::Value(Value::from(v));
				ConditionExpression::SimpleExpr(SimpleExpr::binary(col.clone().into(), op, vxpr))
			};
			let binaries_fn = |op: BinOper, v: Vec<String>| {
				let vxpr = SimpleExpr::Values(v.into_iter().map(Value::from).collect());
				ConditionExpression::SimpleExpr(SimpleExpr::binary(col.clone().into(), op, vxpr))
			};

			let cond_any_of_fn = |op: BinOper, values: Vec<String>, val_prefix: &str, val_suffix: &str| {
				let mut cond = Condition::any();

				for value in values {
					let expr = binary_fn(op, format!("{val_prefix}{value}{val_suffix}"));
					cond = cond.add(expr);
				}

				ConditionExpression::Condition(cond)
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
				OpValString::ContainsIn(values) => cond_any_of_fn(BinOper::Like, values, "%", "%"),

				OpValString::NotContains(s) => binary_fn(BinOper::NotLike, format!("%{s}%")),
				OpValString::NotContainsIn(values) => cond_any_of_fn(BinOper::NotLike, values, "%", "%"),

				OpValString::StartsWith(s) => binary_fn(BinOper::Like, format!("{s}%")),
				OpValString::StartsWithIn(values) => cond_any_of_fn(BinOper::Like, values, "", "%"),

				OpValString::NotStartsWith(s) => binary_fn(BinOper::NotLike, format!("{s}%")),
				OpValString::NotStartsWithIn(values) => cond_any_of_fn(BinOper::NotLike, values, "", "%"),

				OpValString::EndsWith(s) => binary_fn(BinOper::Like, format!("%{s}")),
				OpValString::EndsWithIn(values) => cond_any_of_fn(BinOper::Like, values, "%", ""),

				OpValString::NotEndsWith(s) => binary_fn(BinOper::Like, format!("%{s}")),
				OpValString::NotEndsWithIn(values) => cond_any_of_fn(BinOper::Like, values, "%", ""),

				OpValString::Null(null) => sea_is_col_value_null(col.clone(), null),
				OpValString::Empty(empty) => {
					let op = if empty { BinOper::Equal } else { BinOper::NotEqual };
					Condition::any()
						.add(sea_is_col_value_null(col.clone(), empty))
						.add(binary_fn(op, "".to_string()))
						.into()
				}
			};

			Ok(cond)
		}
	}
}
// endregion: --- with-sea-query
