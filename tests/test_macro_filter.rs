#![allow(unused)] // silence unused warnings while exploring (to comment out)

use modql::{FilterNodes, IntOpVals, StringOpVals};

#[derive(FilterNodes)]
pub struct ProjectFilter {
	#[context("foo")]
	id: Option<IntOpVals>,
	name: Option<StringOpVals>,
}
