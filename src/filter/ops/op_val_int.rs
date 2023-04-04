use crate::filter::OpVal;

#[derive(Debug)]
pub struct OpValsInt64(pub Vec<OpValInt64>);

#[derive(Debug, Clone)]
pub enum OpValInt64 {
	Eq(i64),
	Not(i64),
	In(Vec<i64>),
	NotIn(Vec<i64>),
	Lt(i64),
	Lte(i64),
	Gt(i64),
	Gte(i64),
	Empty(bool),
}

// region:    --- Primitive to IntOpVal
impl From<i64> for OpValInt64 {
	fn from(val: i64) -> Self {
		OpValInt64::Eq(val)
	}
}

impl From<&i64> for OpValInt64 {
	fn from(val: &i64) -> Self {
		OpValInt64::Eq(*val)
	}
}
// endregion: --- Primitive to IntOpVal

// region:    --- IntOpVal to OpVal
impl From<OpValInt64> for OpVal {
	fn from(val: OpValInt64) -> Self {
		OpVal::Int64(val)
	}
}
// endregion: --- IntOpVal to OpVal

// region:    --- Primitive to OpVal::Int(IntOpVal::Eq)
impl From<i64> for OpVal {
	fn from(val: i64) -> Self {
		OpValInt64::Eq(val).into()
	}
}

impl From<&i64> for OpVal {
	fn from(val: &i64) -> Self {
		OpValInt64::Eq(*val).into()
	}
}
// endregion: --- Primitive to OpVal::Int(IntOpVal::Eq)

// region:    --- is_match
impl OpValInt64 {
	/// Matches a target value (`t_val`) with the IntOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: i64) -> bool {
		use OpValInt64::*;

		match self {
			Eq(p_val) => &t_val == p_val,
			Not(p_val) => &t_val != p_val,
			In(p_vals) => p_vals.iter().any(|p_val| &t_val == p_val),
			NotIn(p_vals) => !p_vals.iter().any(|p_val| &t_val == p_val),
			Lt(p_val) => &t_val < p_val,
			Lte(p_val) => &t_val <= p_val,
			Gt(p_val) => &t_val > p_val,
			Gte(p_val) => &t_val >= p_val,
			Empty(_) => false, // always false per this function signature.
		}
	}
}
// endregion: --- is_match
