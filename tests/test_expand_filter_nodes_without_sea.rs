//! Should compile. No test functions yet.

use anyhow::Result;
use modql::filter::{FilterNodes, OpValsInt64, OpValsString, SeaError};
use sea_query::Condition;

#[derive(FilterNodes, Default)]
pub struct ProjectFilter {
	#[modql(context_path = "foo_context_path")]
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,
}

#[test]
fn test_expand_filter_nodes() -> Result<()> {
	let filter = ProjectFilter {
		id: Some(123.into()),

		..Default::default()
	};

	let _cond: Result<Condition, SeaError> = filter.try_into();

	Ok(())
}
