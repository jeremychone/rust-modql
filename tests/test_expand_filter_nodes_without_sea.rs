//! Should compile. No test functions yet.

use anyhow::Result;
use modql::filter::{FilterNodes, OpValsInt64, OpValsString};

#[derive(FilterNodes, Default)]
pub struct ProjectFilter {
	#[modql(context_path = "foo_context_path")]
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,
}

#[cfg(feature = "with-sea-query")]
#[test]
fn test_expand_filter_nodes() -> Result<()> {
	let filter = ProjectFilter {
		id: Some(123.into()),

		..Default::default()
	};

	let _cond: Result<sea_query::Condition, modql::filter::SeaError> = filter.try_into();

	Ok(())
}
