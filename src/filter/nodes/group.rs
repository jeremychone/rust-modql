use crate::filter::FilterNode;

// region:    --- Filter Group
/// A FilterGroup is a vector of FilterNode that are intended to be interpreted as AND.
#[derive(Debug)]
pub struct FilterGroup(Vec<FilterNode>);

impl FilterGroup {
	pub fn nodes(&self) -> &Vec<FilterNode> {
		&self.0
	}
}

impl From<Vec<FilterNode>> for FilterGroup {
	fn from(val: Vec<FilterNode>) -> Self {
		FilterGroup(val)
	}
}

impl From<FilterNode> for FilterGroup {
	fn from(val: FilterNode) -> Self {
		FilterGroup(vec![val])
	}
}

// endregion: --- Filter Group

// region:    --- FilterGroups

/// A FilterGroups is a vector of FilterGroup, and each groups are intended to be inter
#[derive(Debug)]
pub struct FilterGroups(Vec<FilterGroup>);

impl FilterGroups {
	/// Add a new or group (`Vec<FilterNode>`).
	/// It will be OR with its peer groups, and the content of the vector should interpreted as AND.
	pub fn add_group(&mut self, group: Vec<FilterNode>) -> &mut Self {
		self.0.push(FilterGroup(group));
		self
	}

	pub fn groups(&self) -> &Vec<FilterGroup> {
		&self.0
	}
}

/// Create a FilterGroups from a vec or vec of filternode
impl From<Vec<Vec<FilterNode>>> for FilterGroups {
	fn from(val: Vec<Vec<FilterNode>>) -> Self {
		FilterGroups(val.into_iter().map(FilterGroup::from).collect())
	}
}

/// Create a FilterGroups of single FilterNode vector (group of one)
impl From<Vec<FilterNode>> for FilterGroups {
	fn from(val: Vec<FilterNode>) -> Self {
		FilterGroups(vec![val.into()])
	}
}

/// Create a FilterGroups from a single FilterNode
impl From<FilterNode> for FilterGroups {
	fn from(val: FilterNode) -> Self {
		FilterGroups(vec![val.into()])
	}
}

impl From<FilterNode> for Option<FilterGroups> {
	fn from(val: FilterNode) -> Self {
		Some(val.into())
	}
}

impl From<FilterGroup> for FilterGroups {
	fn from(val: FilterGroup) -> Self {
		FilterGroups(vec![val])
	}
}

impl From<FilterGroup> for Option<FilterGroups> {
	fn from(val: FilterGroup) -> Self {
		Some(val.into())
	}
}

// endregion: --- FilterGroups
