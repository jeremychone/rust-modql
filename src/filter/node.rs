use super::ops::OpVal;
use crate::filter::{OpValBool, OpValFloat64, OpValInt64, OpValString};

pub trait IntoFilterNodes {
	fn filter_nodes(self, context_path: Option<String>) -> Vec<FilterNode>;
}

// region:    --- FilterNode
#[derive(Debug, Clone)]
pub struct FilterNode {
	pub context_path: Option<String>, // would be for the project.title (project in this case)
	pub name: String,
	pub opvals: Vec<OpVal>,
}

impl FilterNode {
	pub fn new(name: impl Into<String>, opvals: impl Into<Vec<OpVal>>) -> FilterNode {
		FilterNode {
			context_path: None,
			name: name.into(),
			// opvals: vec![opval.into()],
			opvals: opvals.into(),
		}
	}
}

// Implements the From trait from tuples to FilterNode
macro_rules! impl_from_tuple {
	($($OV:ident),+) => {
		$(
			/// From trait from (prop_name, [Type]OpValue) for FilterNode
			/// (e.g., `let node: FilterNode = ("id", IntOpVal::Gt(1)).into()`)
			impl From<(&str, $OV)> for FilterNode {
				fn from((name, ov): (&str, $OV)) -> Self {
					Self {
						context_path: None,
						name: name.to_string(),
						opvals: vec![ov.into()],
					}
				}
			}

			/// From trait from (prop_name, Vec<[Type]OpValue>)  for FilterNode
			/// (e.g., `let node: FilterNode = (prop_name, Vec<[Type]OpValue>).into()`)
			impl From<(&str, Vec<$OV>)> for FilterNode {
				fn from((name, ovs): (&str, Vec<$OV>)) -> Self {
					Self {
						context_path: None,
						name: name.to_string(),
						opvals: ovs.into_iter().map(|v| v.into()).collect(),
					}
				}
			}
		)+
	};
}
impl_from_tuple!(OpValString, OpValInt64, OpValFloat64, OpValBool);

impl From<(&str, i64)> for FilterNode {
	fn from((name, ov): (&str, i64)) -> Self {
		Self {
			context_path: None,
			name: name.to_string(),
			opvals: vec![OpValInt64::Eq(ov).into()],
		}
	}
}

impl From<(&str, f64)> for FilterNode {
	fn from((name, ov): (&str, f64)) -> Self {
		Self {
			context_path: None,
			name: name.to_string(),
			opvals: vec![OpValFloat64::Eq(ov).into()],
		}
	}
}

impl From<(&str, bool)> for FilterNode {
	fn from((name, ov): (&str, bool)) -> Self {
		Self {
			context_path: None,
			name: name.to_string(),
			opvals: vec![OpValBool::Eq(ov).into()],
		}
	}
}

impl From<(&str, &str)> for FilterNode {
	fn from((name, ov): (&str, &str)) -> Self {
		Self {
			context_path: None,
			name: name.to_string(),
			opvals: vec![OpValString::Eq(ov.to_string()).into()],
		}
	}
}

impl From<(&str, &String)> for FilterNode {
	fn from((name, ov): (&str, &String)) -> Self {
		Self {
			context_path: None,
			name: name.to_string(),
			opvals: vec![OpValString::Eq(ov.to_string()).into()],
		}
	}
}

impl From<(&str, String)> for FilterNode {
	fn from((name, ov): (&str, String)) -> Self {
		Self {
			context_path: None,
			name: name.to_string(),
			opvals: vec![OpValString::Eq(ov).into()],
		}
	}
}

// endregion: --- FilterNode

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
