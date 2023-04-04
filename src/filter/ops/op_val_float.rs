use crate::filter::OpVal;

#[derive(Debug)]
pub struct FloatOpVals(pub Vec<FloatOpVal>);

#[derive(Debug, Clone)]
pub enum FloatOpVal {
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
impl From<f64> for FloatOpVal {
	fn from(val: f64) -> Self {
		FloatOpVal::Eq(val)
	}
}

impl From<&f64> for FloatOpVal {
	fn from(val: &f64) -> Self {
		FloatOpVal::Eq(*val)
	}
}
// endregion: --- Primitive to FloatOpVal

// region:    --- FloatOpVal to OpVal
impl From<FloatOpVal> for OpVal {
	fn from(val: FloatOpVal) -> Self {
		OpVal::Float(val)
	}
}
// endregion: --- FloatOpVal to OpVal

// region:    --- Primitive to OpVal::Float(FloatOpVal::Eq)
impl From<f64> for OpVal {
	fn from(val: f64) -> Self {
		FloatOpVal::Eq(val).into()
	}
}

impl From<&f64> for OpVal {
	fn from(val: &f64) -> Self {
		FloatOpVal::Eq(*val).into()
	}
}
// endregion: --- Primitive to OpVal::Float(FloatOpVal::Eq)

// region:    --- is_match
impl FloatOpVal {
	/// Matches a target value (`t_val`) with the FloatOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: f64) -> bool {
		use FloatOpVal::*;

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
