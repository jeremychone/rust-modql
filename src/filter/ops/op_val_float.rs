use crate::filter::OpVal;

#[derive(Debug)]
pub struct OpValsFloat64(pub Vec<OpValFloat64>);

#[derive(Debug, Clone)]
pub enum OpValFloat64 {
	Eq(f64),
	Not(f64),
	In(Vec<f64>),
	NotIn(Vec<f64>),
	Lt(f64),
	Lte(f64),
	Gt(f64),
	Gte(f64),
	Empty(bool),
}

// region:    --- Primitive to FloatOpVal
impl From<f64> for OpValFloat64 {
	fn from(val: f64) -> Self {
		OpValFloat64::Eq(val)
	}
}

impl From<&f64> for OpValFloat64 {
	fn from(val: &f64) -> Self {
		OpValFloat64::Eq(*val)
	}
}
// endregion: --- Primitive to FloatOpVal

// region:    --- FloatOpVal to OpVal
impl From<OpValFloat64> for OpVal {
	fn from(val: OpValFloat64) -> Self {
		OpVal::Float64(val)
	}
}
// endregion: --- FloatOpVal to OpVal

// region:    --- Primitive to OpVal::Float(FloatOpVal::Eq)
impl From<f64> for OpVal {
	fn from(val: f64) -> Self {
		OpValFloat64::Eq(val).into()
	}
}

impl From<&f64> for OpVal {
	fn from(val: &f64) -> Self {
		OpValFloat64::Eq(*val).into()
	}
}
// endregion: --- Primitive to OpVal::Float(FloatOpVal::Eq)

// region:    --- is_match
impl OpValFloat64 {
	/// Matches a target value (`t_val`) with the FloatOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: f64) -> bool {
		use OpValFloat64::*;

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
