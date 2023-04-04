use crate::OpVal;

#[derive(Debug)]
pub struct StringOpVals(pub Vec<StringOpVal>);

#[derive(Debug, Clone)]
pub enum StringOpVal {
	Eq(String),
	Not(String),

	In(Vec<String>),
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

// region:    --- Primitive to StringOpVal
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
// endregion: --- Primitive to StringOpVal

// region:    --- StringOpVal to OpVal
impl From<StringOpVal> for OpVal {
	fn from(val: StringOpVal) -> Self {
		OpVal::String(val)
	}
}
// endregion: --- StringOpVal to OpVal

// region:    --- Primitive to OpVal::String(StringOpVal::Eq)
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
// endregion: --- Primitive to OpVal::String(StringOpVal::Eq)

// region:    --- is_match
impl StringOpVal {
	/// Matches a target value (`t_val`) with the StringOpVal pattern value (`p_val`)
	pub fn is_match(&self, t_val: &str) -> bool {
		use StringOpVal::*;

		match (self) {
			Eq(p_val) => t_val == p_val,
			Not(p_val) => t_val != p_val,
			In(p_vals) => p_vals.iter().any(|p_val| t_val == p_val),
			NotIn(p_vals) => !p_vals.iter().any(|p_val| t_val == p_val),
			Lt(p_val) => t_val < p_val.as_str(),
			Lte(p_val) => t_val <= p_val.as_str(),
			Gt(p_val) => t_val > p_val.as_str(),
			Gte(p_val) => t_val >= p_val.as_str(),
			Empty(p_val) => p_val == &t_val.is_empty(),
			Contains(p_val) => t_val.contains(p_val),
			NotContains(p_val) => !t_val.contains(p_val),
			ContainsIn(p_vals) => p_vals.iter().any(|p_val| t_val.contains(p_val)),
			NotContainsIn(p_vals) => !p_vals.iter().any(|p_val| t_val.contains(p_val)),
			StartsWith(p_val) => t_val.starts_with(p_val),
			NotStartsWith(p_val) => !t_val.starts_with(p_val),
			StartsWithIn(p_vals) => p_vals.iter().any(|p_val| t_val.starts_with(p_val)),
			NotStartsWithIn(p_vals) => !p_vals.iter().any(|p_val| t_val.starts_with(p_val)),
			EndsWith(p_val) => t_val.starts_with(p_val),
			NotEndsWith(p_val) => !t_val.starts_with(p_val),
			EndsWithIn(p_vals) => p_vals.iter().any(|p_val| t_val.ends_with(p_val)),
			NotEndsWithIn(p_vals) => !p_vals.iter().any(|p_val| t_val.ends_with(p_val)),
		}
	}
}
// endregion: --- is_match
