use crate::filter::OpVal;

#[derive(Debug)]
pub struct OpValsBool(pub Vec<OpValBool>);

#[derive(Debug, Clone)]
pub enum OpValBool {
	Eq(bool),
	Not(bool),
	Empty(bool),
}

// region:    --- Simple Value to Eq BoolOpVal
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
// endregion: --- Simple Value to Eq BoolOpVal

// region:    --- Simple Value to Eq BoolOpVals
impl From<bool> for OpValsBool {
	fn from(val: bool) -> Self {
		OpValBool::from(val).into()
	}
}

impl From<&bool> for OpValsBool {
	fn from(val: &bool) -> Self {
		OpValBool::from(*val).into()
	}
}
// endregion: --- Simple Value to Eq BoolOpVals

// region:    --- BoolOpVal to OpVal
impl From<OpValBool> for OpVal {
	fn from(val: OpValBool) -> Self {
		OpVal::Bool(val)
	}
}
// endregion: --- BoolOpVal to OpVal

// region:    --- Simple Value to Eq OpVal::Bool(BoolOpVal::Eq)
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
// endregion: --- Simple Value to Eq OpVal::Bool(BoolOpVal::Eq)

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

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use sea_query::{BinOper, ColumnRef, ConditionExpression, SimpleExpr, Value};

	impl OpValBool {
		pub fn into_sea_cond_expr(self, col: &ColumnRef) -> ConditionExpression {
			let binary_fn = |op: BinOper, vxpr: SimpleExpr| {
				ConditionExpression::SimpleExpr(SimpleExpr::binary(col.clone().into(), op, vxpr))
			};
			match self {
				OpValBool::Eq(s) => binary_fn(BinOper::Equal, Value::from(s).into()),
				OpValBool::Not(s) => binary_fn(BinOper::NotEqual, Value::from(s).into()),
				OpValBool::Empty(_) => todo!("into_sea_op_val for OpValBool::Empty"),
			}
		}
	}
}
// endregion: --- with-sea-query
