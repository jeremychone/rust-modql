#![cfg(feature = "with-sea-query")]

//! Should compile. No test functions yet.
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
use modql::filter::{FilterNodes, OpValValue, OpValsInt64, OpValsString, OpValsValue, SeaResult};
use sea_query::{BinOper, ColumnRef, ConditionExpression, SimpleExpr, Value};

#[derive(FilterNodes, Default)]
pub struct ProjectFilter {
	#[modql(rels = "foo_rel")]
	id: Option<OpValsInt64>,
	name: Option<OpValsString>,

	#[modql(to_sea_condition_fn = "my_to_sea_condition")]
	ctime: Option<OpValsValue>,
}

fn my_to_sea_condition(col: &ColumnRef, op_val_value: OpValValue) -> SeaResult<ConditionExpression> {
	let binary_fn = |op: BinOper, v: serde_json::Value| {
		let v: i32 = serde_json::from_value(v).unwrap();
		let vexpr: SimpleExpr = Value::from(v).into();
		let expr = SimpleExpr::binary(col.clone().into(), op, vexpr);
		SeaResult::Ok(ConditionExpression::SimpleExpr(expr))
	};

	match op_val_value {
		OpValValue::Eq(v) => binary_fn(BinOper::Equal, v),
		OpValValue::Not(_) => todo!(),
		OpValValue::In(_) => todo!(),
		OpValValue::NotIn(_) => todo!(),
		OpValValue::Lt(_) => todo!(),
		OpValValue::Lte(_) => todo!(),
		OpValValue::Gt(_) => todo!(),
		OpValValue::Gte(_) => todo!(),
		OpValValue::Null(_) => todo!(),
	}
}

#[test]
fn test_expand_filter_nodes() -> Result<()> {
	let _filter = ProjectFilter {
		id: Some(123.into()),
		ctime: Some(OpValValue::Eq(serde_json::Value::from("some-date")).into()),
		..Default::default()
	};

	Ok(())
}
