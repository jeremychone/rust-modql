use crate::OpVal;

/// filters: {
///   "task": {
///      "type": {"$eq", "bug"},
///      "ctime": {"$gte", 2022-10-02}
///   }
/// }
///
///
/// FilterNode {
///   name: "task"
///   node: FilterNode {
///      name: "type",
///      opval: OpValue::String(StringOpValue::Eq("bug")),
///   }
/// }

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
