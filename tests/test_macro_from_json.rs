#![allow(unused)] // silence unused warnings while exploring (to comment out)

use modql::{Error, FilterNodes, FromJson, IntOpVals, StringOpVals};

#[derive(FilterNodes, FromJson)]
pub struct ProjectFilter {
	id: Option<IntOpVals>,
	name: Option<StringOpVals>,
}
