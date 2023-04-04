use crate::filter::{BoolOpVal, BoolOpVals, FloatOpVal, FloatOpVals, IntOpVal, IntOpVals, StringOpVal, StringOpVals};

pub mod op_val_bool;
pub mod op_val_float;
pub mod op_val_int;
pub mod op_val_string;

// region:    --- OpVal
#[derive(Debug, Clone)]
pub enum OpVal {
	String(StringOpVal),
	Int(IntOpVal),
	Float(FloatOpVal),
	Bool(BoolOpVal),
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
	StringOpVal,
	StringOpVals,
	IntOpVal,
	IntOpVals,
	FloatOpVal,
	FloatOpVals,
	BoolOpVal,
	BoolOpVals
);

// endregion: --- From [Type]OpVal to [Type]OpVals
