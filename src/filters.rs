use crate::OpValue;

// region:    --- FilterNode
pub struct FilterNode {
	pub context_name: Option<String>, // would be for the project.title (project in this case)
	pub name: String,
	pub value: FilterNodeValue,
}

pub enum FilterNodeValue {
	/// e.g., [StartsWith("test-"), EndsWith("-suffix")]
	Conds(Vec<OpValue>),
	/// e.g., {"project": [{"$startsWith": "test-"}, ...]}
	Nodes(Vec<FilterNode>),
}
// endregion: --- FilterNode
