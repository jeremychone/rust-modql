use crate::filter::{FilterGroups, FilterNode, OpVal};
use serde_json::Value;

// -- Sub-Modules
mod ovs_de_bool;
mod ovs_de_number;
mod ovs_de_string;
mod ovs_json;

impl FilterGroups {
	pub fn is_match_json(&self, val_root: &Value) -> bool {
		// -- Iterate the groups (or groups).
		let mut groups = self.groups().iter();
		loop {
			let Some(group) = groups.next() else {
					// We have finished all groups, so, nothing match, return false.
					break false;
				};

			// -- Iterate the nodes of each group
			// Nodes must all pass to have the group good.
			let mut nodes = group.nodes().iter();
			let pass_group = loop {
				let Some(node) = nodes.next() else {
								// We have finished all node, nothing broke false, so, we are good.
								// return true.
								break true;
							};

				if !node.is_match_json(val_root) {
					break false;
				}
			};

			// if the group passed, then, we can break with true
			if pass_group {
				break true;
			}
		}
	}
}

impl FilterNode {
	pub fn is_match_json(&self, val_root: &Value) -> bool {
		let Some(val) = val_root.get(&self.name) else {
			return false;
		};

		let mut opvals = self.opvals.iter();

		loop {
			// if no more opvals, and did not fail, then, true
			let Some(opval) = opvals.next() else {
				break true;
			};

			// if we have a opval, then, need to match it with the val.
			let pass = match opval {
				OpVal::String(ov) => val.as_str().map(|v| ov.is_match(v)),
				OpVal::Int64(ov) => val.as_i64().map(|v| ov.is_match(v)),
				OpVal::Float64(ov) => val.as_f64().map(|v| ov.is_match(v)),
				OpVal::Bool(ov) => val.as_bool().map(|v| ov.is_match(v)),
				_ => panic!("oval NOT SUPPORTED TO IMPLEMENT"), // FIXME
			}
			.unwrap_or(false);

			// if false, break early.
			if !pass {
				break false;
			}
		}
	}
}
