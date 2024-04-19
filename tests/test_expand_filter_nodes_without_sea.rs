//! Should compile. No test functions yet.

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
use modql::filter::{FilterNodes, OpValsInt64, OpValsString};

#[derive(FilterNodes, Default)]
pub struct ProjectFilter {
	#[modql(rel = "foo_rel")]
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

	let _cond: std::result::Result<sea_query::Condition, modql::filter::IntoSeaError> = filter.try_into();

	Ok(())
}
