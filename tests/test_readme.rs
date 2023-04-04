// #![allow(unused)]

use modql::filter::{FilterGroups, FilterNode, FilterNodes, OpValBool, OpValString, OpValsBool, OpValsString};

#[test]
fn test_readme_01() -> anyhow::Result<()> {
	let filter_nodes: Vec<FilterNode> = vec![
		(
			"title",
			OpValString::ContainsIn(vec!["Hello".to_string(), "welcome".to_string()]),
		)
			.into(),
		("done", true).into(),
	];
	let filter_groups: FilterGroups = filter_nodes.into();

	println!("filter_groups:\n{filter_groups:?}");

	Ok(())
}

#[test]
fn test_readme_02() -> anyhow::Result<()> {
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
