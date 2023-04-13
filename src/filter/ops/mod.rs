use crate::filter::*;

pub mod op_val_bool;
pub mod op_val_nums;
pub mod op_val_string;

// region:    --- OpVal
#[derive(Debug, Clone)]
pub enum OpVal {
	String(OpValString),
	Uint64(OpValUint64),
	Uint32(OpValUint32),
	Int64(OpValInt64),
	Int32(OpValInt32),
	Float64(OpValFloat64),
	Float32(OpValFloat32),
	Bool(OpValBool),
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
	// Uints
	OpValUint64,
	OpValsUint64,
	OpValUint32,
	OpValsUint32,
	// Floats
	OpValFloat64,
	OpValsFloat64,
	OpValFloat32,
	OpValsFloat32,
	// Bool
	OpValBool,
	OpValsBool
);

// endregion: --- From [Type]OpVal & Vec<[Type]OpVal> to [Type]OpVals
