use crate::filter::OpValsValue;
use crate::filter::*;

pub mod op_val_bool;
pub mod op_val_nums;
pub mod op_val_string;
pub mod op_val_value;

// region:    --- OpVal
#[derive(Debug, Clone)]
pub enum OpVal {
	String(OpValString),

	Int64(OpValInt64),
	Int32(OpValInt32),

	Float64(OpValFloat64),

	Bool(OpValBool),
	Value(OpValValue),
}

// endregion: --- OpVal

// region:    --- From [Type]OpVal & Vec<[Type]OpVal> to [Type]OpVals

// Convenient implementation when single constraints.
// Common implementation
macro_rules! impl_from_for_opvals {
	($($ov:ident, $ovs:ident),*) => {
		$(
			impl From<$ov> for $ovs {
				fn from(val: $ov) -> Self {
					$ovs(vec![val])
				}
			}

			impl From<Vec<$ov>> for $ovs {
				fn from(val: Vec<$ov>) -> Self {
					$ovs(val)
				}
			}
		)*
	};
}

// For all opvals (must specified the pair as macro rules are hygienic)
impl_from_for_opvals!(
	// String
	OpValString,
	OpValsString,
	// Ints
	OpValInt64,
	OpValsInt64,
	OpValInt32,
	OpValsInt32,
	// Floats
	OpValFloat64,
	OpValsFloat64,
	// Bool
	OpValBool,
	OpValsBool,
	// OpValJson
	OpValValue,
	OpValsValue
);

// endregion: --- From [Type]OpVal & Vec<[Type]OpVal> to [Type]OpVals

#[cfg(feature = "with-sea-query")]
pub use self::with_sea_query::*;

#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use sea_query::{ColumnRef, ConditionExpression, Expr, ExprTrait as _};

	pub fn sea_is_col_value_null(col: ColumnRef, null: bool) -> ConditionExpression {
		if null {
			Expr::col(col).is_null().into()
		} else {
			Expr::col(col).is_not_null().into()
		}
	}
}
