#![cfg(feature = "with-sea-query")]
//! TODO: Add more tests

pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
use modql::filter::{FilterNodes, OpValsInt64, OpValsString};
use modql::SIden;
use sea_query::{Query, SqliteQueryBuilder};

#[derive(Clone, FilterNodes, Default)]
pub struct ProjectFilter {
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,
	#[modql(rel = "foo_rel")]
	label: Option<OpValsString>,
}

#[test]
fn test_expand_filter_nodes() -> Result<()> {
	// -- Setup & Fixtures
	let filter = ProjectFilter {
		id: Some(123.into()),
		label: Some("Test".into()),
		..Default::default()
	};

	// -- Exec
	let cond: Result<sea_query::Condition, modql::filter::IntoSeaError> = filter.try_into();
	let cond = cond?;

	let mut query = Query::select();
	query.from(SIden("project")).cond_where(cond);
	let (sql, _) = query.build(SqliteQueryBuilder);
	// Note: No columns, but that's ok for this test for now.

	// -- Check
	assert!(
		sql.contains(r#"WHERE "id" = ? AND "foo_rel"."label" = ?"#),
		"Incorrect where statment"
	);

	Ok(())
}
