// #![allow(unused)]
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use modql::filter::{FilterGroups, FilterNode, FilterNodes, OpValBool, OpValString, OpValsBool, OpValsString};

#[test]
fn test_readme_01() -> Result<()> {
	let filter_nodes: Vec<FilterNode> = vec![
		(
			"title",
			OpValString::ContainsAny(vec!["Hello".to_string(), "welcome".to_string()]),
		)
			.into(),
		("done", true).into(),
	];
	let filter_groups: FilterGroups = filter_nodes.into();

	println!("filter_groups:\n{filter_groups:?}");

	Ok(())
}

#[test]
fn test_readme_02() -> Result<()> {
	#[derive(FilterNodes)]
	struct MyFilter {
		done: Option<OpValsBool>,
		name: Option<OpValsString>,
	}

	let filter = MyFilter {
		done: Some(OpValBool::Eq(true).into()),
		name: Some(
			vec![
				OpValString::Contains("Hello".to_string()),
				OpValString::Contains("welcome".to_string()),
			]
			.into(),
		),
	};

	let filter_groups: FilterGroups = filter.into();

	println!("filter_groups:\n{filter_groups:?}");

	Ok(())
}
