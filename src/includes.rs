//! PLACEHOLDER for now. Not used yet.

// region:    --- Includes
#[derive(Debug)]
pub struct Includes {
	pub value: IncludeValue,
}

impl Includes {
	pub fn new(value: IncludeValue) -> Includes {
		Includes { value }
	}
}
// endregion: --- Includes

// region:    --- IncludeNode
#[derive(Debug)]
pub struct IncludeNode {
	pub name: String,
	pub value: IncludeValue,
}

impl From<(&str, bool)> for IncludeNode {
	fn from(val: (&str, bool)) -> Self {
		IncludeNode {
			name: val.0.to_owned(),
			value: IncludeValue::Value(val.1),
		}
	}
}

impl From<(String, bool)> for IncludeNode {
	fn from(val: (String, bool)) -> Self {
		IncludeNode {
			name: val.0,
			value: IncludeValue::Value(val.1),
		}
	}
}
// endregion: --- IncludeNode

// region:    --- IncludeValue
#[derive(Debug)]
pub enum IncludeValue {
	Value(bool),
	Nodes(Vec<IncludeNode>),
}
// endregion: --- IncludeValue
