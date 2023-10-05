use crate::filter::OpVal;

/// - `ovs` OpValsType, e.g., `OpValsUint64`
/// - `ov` OpValType, e.g., `OpValUint64`
/// - `nt` Number type, e.g., `u64`
/// - `vr` Opval Variant e.g., `OpVal::Uint64`
macro_rules! impl_op_val {
	($(($ovs:ident, $ov:ident,$nt:ty, $vr:expr)),+) => {
		$(

#[derive(Debug)]
pub struct $ovs(pub Vec<$ov>);

#[derive(Debug, Clone)]
pub enum $ov {
	Eq($nt),
	Not($nt),
	In(Vec<$nt>),
	NotIn(Vec<$nt>),
	Lt($nt),
	Lte($nt),
	Gt($nt),
	Gte($nt),
	Empty(bool),
}

// region:    --- Simple value to Eq e.g., OpValUint64
impl From<$nt> for $ov {
	fn from(val: $nt) -> Self {
		$ov::Eq(val)
	}
}

impl From<&$nt> for $ov {
	fn from(val: &$nt) -> Self {
		$ov::Eq(*val)
	}
}
// endregion: --- Simple value to Eq e.g., OpValUint64

// region:    --- Simple value to Eq e.g., OpValsUint64
impl From<$nt> for $ovs {
	fn from(val: $nt) -> Self {
		$ov::from(val).into()
	}
}

impl From<&$nt> for $ovs {
	fn from(val: &$nt) -> Self {
		$ov::from(*val).into()
	}
}
// endregion: --- Simple value to Eq e.g., OpValsUint64

// region:    --- e.g., OpValUint64 to OpVal
impl From<$ov> for OpVal {
	fn from(val: $ov) -> Self {
		$vr(val)
	}
}
// endregion: --- e.g., OpValUint64 to OpVal

// region:    --- Primitive to OpVal::Int(IntOpVal::Eq)
impl From<$nt> for OpVal {
	fn from(val: $nt) -> Self {
		$ov::Eq(val).into()
	}
}

impl From<&$nt> for OpVal {
	fn from(val: &$nt) -> Self {
		$ov::Eq(*val).into()
	}
}
// endregion: --- Primitive to OpVal::Int(IntOpVal::Eq)


// region:    --- is_match
impl $ov {
	/// Matches a target value (`t_val`) with the IntOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: $nt) -> bool {
		use $ov::*;

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

		)+
	};
}

impl_op_val!(
	(OpValsUint64, OpValUint64, u64, OpVal::Uint64),
	(OpValsUint32, OpValUint32, u32, OpVal::Uint32),
	(OpValsInt64, OpValInt64, i64, OpVal::Int64),
	(OpValsInt32, OpValInt32, i32, OpVal::Int32),
	(OpValsFloat64, OpValFloat64, f64, OpVal::Float64),
	(OpValsFloat32, OpValFloat32, f32, OpVal::Float32)
);

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use sea_query::{BinOper, ColumnRef, ConditionExpression, SimpleExpr, Value};

	macro_rules! impl_into_sea_op_val {
		($($ov:ident),+) => {
			$(
	impl $ov {
		pub fn into_sea_cond_expr(self, col: &ColumnRef) -> ConditionExpression {
			let binary_fn = |op: BinOper, vxpr: SimpleExpr| {
				ConditionExpression::SimpleExpr(SimpleExpr::binary(col.clone().into(), op, vxpr))
			};
			match self {
				$ov::Eq(s) => binary_fn(BinOper::Equal, Value::from(s).into()),
				$ov::Not(s) => binary_fn(BinOper::NotEqual, Value::from(s).into()),
				$ov::In(s) => binary_fn(
					BinOper::In,
					SimpleExpr::Values(s.into_iter().map(Value::from).collect()),
				),
				$ov::NotIn(s) => binary_fn(
					BinOper::NotIn,
					SimpleExpr::Values(s.into_iter().map(Value::from).collect()),
				),
				$ov::Lt(s) => binary_fn(BinOper::SmallerThan, Value::from(s).into()),
				$ov::Lte(s) => binary_fn(BinOper::SmallerThanOrEqual, Value::from(s).into()),
				$ov::Gt(s) => binary_fn(BinOper::GreaterThan, Value::from(s).into()),
				$ov::Gte(s) => binary_fn(BinOper::GreaterThanOrEqual, Value::from(s).into()),
				$ov::Empty(_s) => todo!("$ov::Empty not implemented yet"),
			}
		}
	}
			)+
		};
	}

	impl_into_sea_op_val!(
		OpValUint64,
		OpValUint32,
		OpValInt64,
		OpValInt32,
		OpValFloat64,
		OpValFloat32
	);
}
// endregion: --- with-sea-query
