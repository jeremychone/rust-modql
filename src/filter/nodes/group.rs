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

impl IntoIterator for FilterGroup {
	type Item = FilterNode;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
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

/// A FilterGroups is a vector of FilterGroup, and each groups are intended to be OR between them,
///  and inside the group, that will be the And
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

	pub fn into_vec(self) -> Vec<FilterGroup> {
		self.0
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

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use sea_query::Condition;

	impl FilterGroup {
		// pub fn into_sea_expr_iter(self) -> impl Iterator<Item = SimpleExpr> {
		// 	self.into_iter().flat_map(|node| node.into_sea_expr())
		// }
		pub fn into_sea_condition(self) -> Condition {
			let exprs = self.into_iter().flat_map(|node| node.into_sea_expr());
			let mut cond = Condition::all();
			for expr in exprs {
				cond = cond.add(expr);
			}
			cond
		}
	}

	impl FilterGroups {
		pub fn into_sea_condition(self) -> Condition {
			let mut cond = Condition::any();

			for group in self.0.into_iter() {
				cond = cond.add(group.into_sea_condition());
			}

			cond
		}
	}
}
// endregion: --- with-sea-query
