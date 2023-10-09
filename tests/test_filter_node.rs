#![allow(unused)] // For early development.
#![cfg(feature = "with-sea-query")]

use modql::filter::{FilterNode, OpValInt32, OpValValue, SeaError, SeaResult, ToSeaConditionFnHolder};
use sea_query::{ColumnRef, ConditionExpression};
use std::sync::Arc;

#[test]
fn test_filter_node_with_sea_condition() {
	let special_to_sea_cond = ToSeaConditionFnHolder::new(special_to_sea_condition); // This should implement IntoSeaCondition

	let node = FilterNode {
		context_path: None,
		name: "some_name".to_string(),
		opvals: vec![123.into()],
		for_sea_condition: Some(special_to_sea_cond.into()),
	};
}

pub fn special_to_sea_condition(col: &ColumnRef, op_val: OpValValue) -> SeaResult<ConditionExpression> {
	todo!()
}
