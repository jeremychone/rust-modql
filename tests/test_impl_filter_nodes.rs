//! Should compile. No test functions yet.

use modql::filter::{FilterNode, IntOpVal, IntoFilterNodes, StringOpVal, StringOpVals};

pub struct ProjectFilter {
	id: Option<Vec<IntOpVal>>,
	name: Option<Vec<StringOpVal>>,
}

impl IntoFilterNodes for ProjectFilter {
	fn filter_nodes(self, context: Option<String>) -> Vec<FilterNode> {
		let mut nodes = Vec::new();

		if let Some(id) = self.id {
			let node = FilterNode {
				context_path: context.clone(),
				name: "id".to_string(),
				opvals: id.into_iter().map(|n| n.into()).collect(),
			};
			nodes.push(node)
		}

		if let Some(name) = self.name {
			let node = FilterNode {
				context_path: context,
				name: "name".to_string(),
				opvals: name.into_iter().map(|n| n.into()).collect(),
			};
			nodes.push(node)
		}

		nodes
	}
}

#[allow(unused)]
pub struct TaskFilter {
	project: Option<ProjectFilter>,
	title: Option<StringOpVals>,
	kind: Option<StringOpVals>,
}

impl IntoFilterNodes for TaskFilter {
	fn filter_nodes(self, context: Option<String>) -> Vec<FilterNode> {
		let mut nodes = Vec::new();

		if let Some(title) = self.title {
			let node = FilterNode {
				context_path: context,
				name: "title".to_string(),
				opvals: title.0.into_iter().map(|n| n.into()).collect(),
			};
			nodes.push(node)
		}

		nodes
	}
}
