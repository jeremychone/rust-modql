//! Module for all the Operator Value types.
//!
//! - `OpVal` is the unit Operator Value enum, for a single operator and single value.
//!    The Variant are represent the [Type]OvVal
//!
//! - `[Type]OpVal` (e.g., `StringOpVal`) is the Operator and Value for a specific main type (String, Int, Float, Bool for now)
//!
//! -
//!

// region:    --- OpVal and From<[Type]OpVal>
#[derive(Debug)]
pub enum OpVal {
	String(StringOpVal),
	Int(IntOpVal),
	Float(FloatOpVal),
	Bool(BoolOpVal),
}

impl From<IntOpVal> for OpVal {
	fn from(val: IntOpVal) -> Self {
		OpVal::Int(val)
	}
}
impl From<FloatOpVal> for OpVal {
	fn from(val: FloatOpVal) -> Self {
		OpVal::Float(val)
	}
}
impl From<StringOpVal> for OpVal {
	fn from(val: StringOpVal) -> Self {
		OpVal::String(val)
	}
}
impl From<BoolOpVal> for OpVal {
	fn from(val: BoolOpVal) -> Self {
		OpVal::Bool(val)
	}
}

// endregion: --- OpVal and From<[Type]OpVal>

// region:    --- From<scalar> to OpVal
impl From<String> for OpVal {
	fn from(val: String) -> Self {
		StringOpVal::Eq(val).into()
	}
}

impl From<&str> for OpVal {
	fn from(val: &str) -> Self {
		StringOpVal::Eq(val.to_string()).into()
	}
}

impl From<i64> for OpVal {
	fn from(val: i64) -> Self {
		IntOpVal::Eq(val).into()
	}
}

impl From<&i64> for OpVal {
	fn from(val: &i64) -> Self {
		IntOpVal::Eq(*val).into()
	}
}

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
// endregion: --- From<scalar> to OpVal

// region:    --- [Type]OpVal and [Type]OpVals
#[derive(Debug)]
pub struct StringOpVals(pub Vec<StringOpVal>);

#[derive(Debug)]
pub enum StringOpVal {
	Eq(String),
	In(Vec<String>),
	Not(String),
	NotIn(Vec<String>),

	Lt(String),
	Lte(String),

	Gt(String),
	Gte(String),

	Empty(bool),

	Contains(String),
	NotContains(String),

	ContainsIn(Vec<String>),
	NotContainsIn(Vec<String>),

	StartsWith(String),
	NotStartsWith(String),

	StartsWithIn(Vec<String>),
	NotStartsWithIn(Vec<String>),

	EndsWith(String),
	NotEndsWith(String),

	EndsWithIn(Vec<String>),
	NotEndsWithIn(Vec<String>),
}

impl From<String> for StringOpVal {
	fn from(val: String) -> Self {
		StringOpVal::Eq(val)
	}
}

impl From<&str> for StringOpVal {
	fn from(val: &str) -> Self {
		StringOpVal::Eq(val.to_string())
	}
}

#[derive(Debug)]
pub struct IntOpVals(pub Vec<IntOpVal>);

#[derive(Debug)]
pub enum IntOpVal {
	Eq(i64),
	Not(i64),
	In(Vec<i64>),
	NotIn(Vec<i64>),
	Lt(i64),
	Lte(i64),
	Gt(i64),
	Gte(i64),
	Empty(bool),
}

impl From<i64> for IntOpVal {
	fn from(val: i64) -> Self {
		IntOpVal::Eq(val)
	}
}

impl From<&i64> for IntOpVal {
	fn from(val: &i64) -> Self {
		IntOpVal::Eq(*val)
	}
}

#[derive(Debug)]
pub struct FloatOpVals(pub Vec<FloatOpVal>);

#[derive(Debug)]
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

#[derive(Debug)]
pub struct BoolOpVals(pub Vec<BoolOpVal>);

#[derive(Debug)]
pub enum BoolOpVal {
	Eq(bool),
	Not(bool),
}

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
// endregion: --- [Type]OpVal and [Type]OpVals

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
