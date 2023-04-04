use crate::filter::OpVal;

#[derive(Debug)]
pub struct OpValsBool(pub Vec<OpValBool>);

#[derive(Debug, Clone)]
pub enum OpValBool {
	Eq(bool),
	Not(bool),
	Empty(bool),
}

// region:    --- Primitive to BoolOpVal
impl From<bool> for OpValBool {
	fn from(val: bool) -> Self {
		OpValBool::Eq(val)
	}
}

impl From<&bool> for OpValBool {
	fn from(val: &bool) -> Self {
		OpValBool::Eq(*val)
	}
}
// endregion: --- Primitive to BoolOpVal

// region:    --- BoolOpVal to OpVal
impl From<OpValBool> for OpVal {
	fn from(val: OpValBool) -> Self {
		OpVal::Bool(val)
	}
}
// endregion: --- BoolOpVal to OpVal

// region:    --- Primitive to OpVal::Bool(BoolOpVal::Eq)
impl From<bool> for OpVal {
	fn from(val: bool) -> Self {
		OpValBool::Eq(val).into()
	}
}

impl From<&bool> for OpVal {
	fn from(val: &bool) -> Self {
		OpValBool::Eq(*val).into()
	}
}
// endregion: --- Primitive to OpVal::Bool(BoolOpVal::Eq)

// region:    --- is_match
impl OpValBool {
	/// Matches a target value (`t_val`) with the BoolOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: bool) -> bool {
		use OpValBool::*;

		match self {
			Eq(p_val) => &t_val == p_val,
			Not(p_val) => &t_val != p_val,
			Empty(_) => false, // always false per this function signature.
		}
	}
}
// endregion: --- is_match
