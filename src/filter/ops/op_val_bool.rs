use crate::filter::OpVal;

#[derive(Debug)]
pub struct BoolOpVals(pub Vec<BoolOpVal>);

#[derive(Debug, Clone)]
pub enum BoolOpVal {
	Eq(bool),
	Not(bool),
	Empty(bool),
}

// region:    --- Primitive to BoolOpVal
impl From<bool> for BoolOpVal {
	fn from(val: bool) -> Self {
		BoolOpVal::Eq(val)
	}
}

impl From<&bool> for BoolOpVal {
	fn from(val: &bool) -> Self {
		BoolOpVal::Eq(*val)
	}
}
// endregion: --- Primitive to BoolOpVal

// region:    --- BoolOpVal to OpVal
impl From<BoolOpVal> for OpVal {
	fn from(val: BoolOpVal) -> Self {
		OpVal::Bool(val)
	}
}
// endregion: --- BoolOpVal to OpVal

// region:    --- Primitive to OpVal::Bool(BoolOpVal::Eq)
impl From<bool> for OpVal {
	fn from(val: bool) -> Self {
		BoolOpVal::Eq(val).into()
	}
}

impl From<&bool> for OpVal {
	fn from(val: &bool) -> Self {
		BoolOpVal::Eq(*val).into()
	}
}
// endregion: --- Primitive to OpVal::Bool(BoolOpVal::Eq)

// region:    --- is_match
impl BoolOpVal {
	/// Matches a target value (`t_val`) with the BoolOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: bool) -> bool {
		use BoolOpVal::*;

		match self {
			Eq(p_val) => &t_val == p_val,
			Not(p_val) => &t_val != p_val,
			Empty(_) => false, // always false per this function signature.
		}
	}
}
// endregion: --- is_match
