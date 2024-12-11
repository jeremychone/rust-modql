use crate::filter::OpVal;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct OpValsValue(pub Vec<OpValValue>);

#[derive(Debug, Clone)]
pub enum OpValValue {
	Eq(Value),
	Not(Value),

	In(Vec<Value>),
	NotIn(Vec<Value>),

	Lt(Value),
	Lte(Value),

	Gt(Value),
	Gte(Value),

	Null(bool),
}

// NOTE: We cannot implement the From<Value> for OpValValue, OpValsValue, ..
//       because it could fail if the json::Value is not a scalar type

// region:    --- OpValValue to OpVal::Value
impl From<OpValValue> for OpVal {
	fn from(val: OpValValue) -> Self {
		OpVal::Value(val)
	}
}
// endregion: --- OpValValue to OpVal::Value

mod json {
	use crate::filter::json::OpValueToOpValType;
	use crate::filter::OpValValue;
	use crate::{Error, Result};
	use serde_json::Value;

	impl OpValueToOpValType for OpValValue {
		fn op_value_to_op_val_type(op: &str, value: Value) -> Result<Self>
		where
			Self: Sized,
		{
			fn into_values(value: Value) -> Result<Vec<Value>> {
				let mut values = Vec::new();

				let Value::Array(array) = value else {
					return Err(Error::JsonValArrayWrongType { actual_value: value });
				};

				for item in array.into_iter() {
					values.push(item)
				}

				Ok(values)
			}

			let ov = match (op, value) {
				("$eq", v) => OpValValue::Eq(v),
				("$in", value) => OpValValue::NotIn(into_values(value)?),

				("$not", v) => OpValValue::Not(v),
				("$notIn", value) => OpValValue::NotIn(into_values(value)?),

				("$lt", v) => OpValValue::Lt(v),
				("$lte", v) => OpValValue::Lte(v),

				("$gt", v) => OpValValue::Gt(v),
				("$gte", v) => OpValValue::Gte(v),

				("$null", Value::Bool(v)) => OpValValue::Null(v),

				(_, v) => {
					return Err(Error::JsonOpValNotSupported {
						operator: op.to_string(),
						value: v,
					})
				}
			};
			Ok(ov)
		}
	}
}

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use crate::filter::{sea_is_col_value_null, FilterNodeOptions, SeaResult, ToSeaValueFnHolder};
	use crate::{into_node_column_expr, into_node_value_expr};
	use sea_query::{BinOper, ColumnRef, ConditionExpression, SimpleExpr};

	impl OpValValue {
		pub fn into_sea_cond_expr_with_json_to_sea(
			self,
			col: &ColumnRef,
			node_options: &FilterNodeOptions,
			to_sea_value: &ToSeaValueFnHolder,
		) -> SeaResult<ConditionExpression> {
			// -- CondExpr builder for single value
			let binary_fn = |op: BinOper, json_value: serde_json::Value| -> SeaResult<ConditionExpression> {
				let sea_value = to_sea_value.call(json_value)?;

				let vxpr = into_node_value_expr(sea_value, node_options);
				let column = into_node_column_expr(col.clone(), node_options);
				Ok(ConditionExpression::SimpleExpr(SimpleExpr::binary(column.into(), op,vxpr)))
			};

			// -- CondExpr builder for single value
			let binaries_fn = |op: BinOper, json_values: Vec<serde_json::Value>| -> SeaResult<ConditionExpression> {
				// -- Build the list of sea_query::Value
				let sea_values: Vec<sea_query::Value> = json_values
					.into_iter()
					.map(|v| to_sea_value.call(v))
					.collect::<SeaResult<_>>()?;

				// -- Transform to the list of SimpleExpr
				let vxpr_list: Vec<SimpleExpr> =
					sea_values.into_iter().map(|v| into_node_value_expr(v, node_options)).collect();
				let vxpr = SimpleExpr::Tuple(vxpr_list);

				// -- Return the condition expression
				let column = into_node_column_expr(col.clone(), node_options);
				Ok(ConditionExpression::SimpleExpr(SimpleExpr::binary(column.into(), op, vxpr)))
			};

			let cond = match self {
				OpValValue::Eq(json_value) => binary_fn(BinOper::Equal, json_value)?,
				OpValValue::In(json_values) => binaries_fn(BinOper::In, json_values)?,

				OpValValue::Not(json_value) => binary_fn(BinOper::NotEqual, json_value)?,
				OpValValue::NotIn(json_value) => binaries_fn(BinOper::NotIn, json_value)?,

				OpValValue::Lt(json_value) => binary_fn(BinOper::SmallerThan, json_value)?,
				OpValValue::Lte(json_value) => binary_fn(BinOper::SmallerThanOrEqual, json_value)?,

				OpValValue::Gt(json_value) => binary_fn(BinOper::GreaterThan, json_value)?,
				OpValValue::Gte(json_value) => binary_fn(BinOper::GreaterThanOrEqual, json_value)?,

				OpValValue::Null(null) => sea_is_col_value_null(col.clone(), null),
			};

			Ok(cond)
		}
	}
}
// endregion: --- with-sea-query
