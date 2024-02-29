#![cfg(feature = "with-sea-query")]
#![allow(unused)] // Ok for those tests.

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
use modql::filter::{FilterGroups, FilterNode, IntoFilterNodes, OpValsBool, OpValsInt64, OpValsString};
use modql::SIden;
use modql_macros::FilterNodes;
use sea_query::{Condition, PostgresQueryBuilder, Query};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use serde_with::{serde_as, OneOrMany};

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct TaskFilter {
	id: Option<OpValsInt64>,
	title: Option<OpValsString>,
	bool: Option<OpValsBool>,
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct TaskListParams {
	#[serde_as(deserialize_as = "Option<OneOrMany<_>>")]
	filters: Option<Vec<TaskFilter>>,
}

#[test]
fn test_json_filters_main() -> Result<()> {
	// let params = json!({
	// 	"filters": [{
	// 		"id": {"$gt": 123},
	// 		"title": {"$contains": "World"}
	// 	},
	// 	{
	// 		"title": {"$startsWith": "Hello"}
	// 	}]
	// });

	let params = json!({
		"filters": {
			// "title": {"$contains": "World"},
			"title": {"$in": ["123", "124"]}
		}
	});

	let params: TaskListParams = from_value(params)?;

	let filters = params.filters.unwrap();

	let fg: FilterGroups = filters.into();

	let cond: Condition = fg.into_sea_condition()?;

	let mut query = Query::select();
	query.from(SIden("task"));
	query.cond_where(cond);

	let (sql, values) = query.build(PostgresQueryBuilder);
	// Note: for now, just check that all compiles and no runtime errors.

	Ok(())
}
