#![allow(unused)] // silence unused warnings while exploring (to comment out)

use modql::{FilterNode, IncludeNode, IncludeValue, Includes, IntOpVal, IntoFilterNodes, StringOpVal, StringOpVals};

pub struct Task {
	project_id: i64,
	title: String,
	kind: String,
}

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

pub struct TaskFilter {
	project: Option<ProjectFilter>,
	title: Option<StringOpVals>,
	kind: Option<StringOpVals>,
}
