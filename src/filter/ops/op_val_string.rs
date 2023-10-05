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

	Empty(bool),

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

// region:    --- is_match
impl OpValString {
	/// Matches a target value (`t_val`) with the StringOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: &str) -> bool {
		use OpValString::*;

		match self {
			Eq(p_val) => t_val == p_val,
			Not(p_val) => t_val != p_val,
			In(p_vals) => p_vals.iter().any(|p_val| t_val == p_val),
			NotIn(p_vals) => !p_vals.iter().any(|p_val| t_val == p_val),
			Lt(p_val) => t_val < p_val.as_str(),
			Lte(p_val) => t_val <= p_val.as_str(),
			Gt(p_val) => t_val > p_val.as_str(),
			Gte(p_val) => t_val >= p_val.as_str(),
			Empty(p_val) => p_val == &t_val.is_empty(),
			Contains(p_val) => t_val.contains(p_val),
			NotContains(p_val) => !t_val.contains(p_val),
			ContainsIn(p_vals) => p_vals.iter().any(|p_val| t_val.contains(p_val)),
			NotContainsIn(p_vals) => !p_vals.iter().any(|p_val| t_val.contains(p_val)),
			StartsWith(p_val) => t_val.starts_with(p_val),
			NotStartsWith(p_val) => !t_val.starts_with(p_val),
			StartsWithIn(p_vals) => p_vals.iter().any(|p_val| t_val.starts_with(p_val)),
			NotStartsWithIn(p_vals) => !p_vals.iter().any(|p_val| t_val.starts_with(p_val)),
			EndsWith(p_val) => t_val.starts_with(p_val),
			NotEndsWith(p_val) => !t_val.starts_with(p_val),
			EndsWithIn(p_vals) => p_vals.iter().any(|p_val| t_val.ends_with(p_val)),
			NotEndsWithIn(p_vals) => !p_vals.iter().any(|p_val| t_val.ends_with(p_val)),
		}
	}
}
// endregion: --- is_match

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use sea_query::{BinOper, ColumnRef, ConditionExpression, SimpleExpr, Value};

	impl OpValString {
		pub fn into_sea_cond_expr(self, col: &ColumnRef) -> ConditionExpression {
			let binary_fn = |op: BinOper, vxpr: SimpleExpr| {
				ConditionExpression::SimpleExpr(SimpleExpr::binary(col.clone().into(), op, vxpr))
			};
			match self {
				OpValString::Eq(s) => binary_fn(BinOper::Equal, Value::from(s).into()),
				OpValString::Not(s) => binary_fn(BinOper::NotEqual, Value::from(s).into()),
				OpValString::In(s) => binary_fn(
					BinOper::In,
					SimpleExpr::Values(s.into_iter().map(Value::from).collect()),
				),
				OpValString::NotIn(s) => binary_fn(
					BinOper::NotIn,
					SimpleExpr::Values(s.into_iter().map(Value::from).collect()),
				),
				OpValString::Lt(s) => binary_fn(BinOper::SmallerThan, Value::from(s).into()),
				OpValString::Lte(s) => binary_fn(BinOper::SmallerThanOrEqual, Value::from(s).into()),
				OpValString::Gt(s) => binary_fn(BinOper::GreaterThan, Value::from(s).into()),
				OpValString::Gte(s) => binary_fn(BinOper::GreaterThanOrEqual, Value::from(s).into()),
				OpValString::Empty(_s) => todo!("OpValString::Empty not implemented yet"),
				OpValString::Contains(s) => binary_fn(BinOper::Like, Value::from(format!("%{s}%")).into()),
				OpValString::NotContains(s) => binary_fn(BinOper::NotLike, Value::from(format!("%{s}%")).into()),
				OpValString::ContainsIn(_s) => todo!("OpValString::ContainsIn not implemented yet"),
				OpValString::NotContainsIn(_s) => todo!("OpValString::NotContainsIn not implemented yet"),
				OpValString::StartsWith(s) => binary_fn(BinOper::Like, Value::from(format!("{s}%")).into()),
				OpValString::NotStartsWith(s) => binary_fn(BinOper::NotLike, Value::from(format!("{s}%")).into()),
				OpValString::StartsWithIn(_s) => todo!("OpValString::StartsWithIn not implemented yet"),
				OpValString::NotStartsWithIn(_s) => todo!("OpValString::NotStartsWithIn not implemented yet"),
				OpValString::EndsWith(s) => binary_fn(BinOper::Like, Value::from(format!("%{s}")).into()),
				OpValString::NotEndsWith(s) => binary_fn(BinOper::Like, Value::from(format!("%{s}")).into()),
				OpValString::EndsWithIn(_s) => todo!("OpValString::EndsWithIn not implemented yet"),
				OpValString::NotEndsWithIn(_s) => todo!("OpValString::NotEndsWithIn not implemented yet"),
			}
		}
	}
}
// endregion: --- with-sea-query
