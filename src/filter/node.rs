use super::ops::OpVal;

pub trait IntoFilterNodes {
	fn filter_nodes(self, context_path: Option<String>) -> Vec<FilterNode>;
}

// region:    --- FilterNode
#[derive(Debug)]
pub struct FilterNode {
	pub context_path: Option<String>, // would be for the project.title (project in this case)
	pub name: String,
	pub opvals: Vec<OpVal>,
}

impl FilterNode {
	pub fn new(name: impl Into<String>, opval: impl Into<OpVal>) -> FilterNode {
		FilterNode {
			context_path: None,
			name: name.into(),
			opvals: vec![opval.into()],
		}
	}
}
// endregion: --- FilterNode

// region:    --- OrGroups

/// Represent a Vector of Vector of Filter nodes.
/// The top Vector are intended to be used as ORs,
/// and the leaf FilterNode vectors used as ANDs.
pub struct OrGroups(Vec<Vec<FilterNode>>);

impl OrGroups {
	/// Add a new or group (Vec<FilterNode>)
	pub fn add_group(&mut self, group: Vec<FilterNode>) -> &mut Self {
		self.0.push(group);
		self
	}

	pub fn groups(self) -> Vec<Vec<FilterNode>> {
		self.0
	}
}

/// Create a OrGroups of single FilterNode vector (group of one)
impl From<Vec<Vec<FilterNode>>> for OrGroups {
	fn from(val: Vec<Vec<FilterNode>>) -> Self {
		OrGroups(val)
	}
}

/// Create a OrGroups of single FilterNode vector (group of one)
impl From<Vec<FilterNode>> for OrGroups {
	fn from(val: Vec<FilterNode>) -> Self {
		OrGroups(vec![val])
	}
}

/// Create a OrGroups from a single FilterNode
impl From<FilterNode> for OrGroups {
	fn from(val: FilterNode) -> Self {
		OrGroups(vec![vec![val]])
	}
}
// endregion: --- OrGroups
