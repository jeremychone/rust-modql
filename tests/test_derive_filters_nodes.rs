//! Should compile. No test functions yet.

use modql::filter::{FilterNodes, OpValsInt64, OpValsString};

#[derive(FilterNodes)]
pub struct ProjectFilter {
	#[context("foo")]
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,
}
