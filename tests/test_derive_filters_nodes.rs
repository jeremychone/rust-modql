//! Should compile. No test functions yet.

use modql::filter::{FilterNodes, IntOpVals, StringOpVals};

#[derive(FilterNodes)]
pub struct ProjectFilter {
	#[context("foo")]
	id: Option<IntOpVals>,
	name: Option<StringOpVals>,
}
