#![allow(unused)] // silence unused warnings while exploring (to comment out)

use modql::{Error, FilterNodes, IntOpVals, StringOpVals};
use serde::Deserialize;

#[derive(FilterNodes, Deserialize)]
pub struct ProjectFilter {
	id: Option<IntOpVals>,
	name: Option<StringOpVals>,
}
