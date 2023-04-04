use crate::filter::{
	OpValBool, OpValFloat64, OpValInt64, OpValString, OpValsBool, OpValsFloat64, OpValsInt64, OpValsString,
};

pub mod op_val_bool;
pub mod op_val_float;
pub mod op_val_int;
pub mod op_val_string;

// region:    --- OpVal
#[derive(Debug, Clone)]
pub enum OpVal {
	String(OpValString),
	Int64(OpValInt64),
	Float64(OpValFloat64),
	Bool(OpValBool),
}

// endregion: --- OpVal

// region:    --- From [Type]OpVal to [Type]OpVals

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
		)*
	};
}

// For all opvals (must specified the pair as macro rules are hygienic)
impl_from_for_opvals!(
	OpValString,
	OpValsString,
	OpValInt64,
	OpValsInt64,
	OpValFloat64,
	OpValsFloat64,
	OpValBool,
	OpValsBool
);

// endregion: --- From [Type]OpVal to [Type]OpVals
