// region:    --- Modules

mod error;

pub use self::error::{SeaError, SeaResult};

use crate::filter::OpValValue;
use sea_query::{ColumnRef, ConditionExpression};

// endregion: --- Modules

#[derive(Clone, Debug)]
pub enum ForSeaCondition {
	ToSeaValue(ToSeaValueFnHolder),
	ToSeaCondition(ToSeaConditionFnHolder),
}

impl From<ToSeaValueFnHolder> for ForSeaCondition {
	fn from(val: ToSeaValueFnHolder) -> Self {
		ForSeaCondition::ToSeaValue(val)
	}
}

impl From<ToSeaConditionFnHolder> for ForSeaCondition {
	fn from(val: ToSeaConditionFnHolder) -> Self {
		ForSeaCondition::ToSeaCondition(val)
	}
}

// region:    --- ToSeaValueFn
pub type JsonToSeaValueFn = fn(serde_json::Value) -> SeaResult<sea_query::Value>;

#[derive(Clone, Debug)]
pub struct ToSeaValueFnHolder {
	fun: JsonToSeaValueFn,
}

impl ToSeaValueFnHolder {
	pub fn new(fun: JsonToSeaValueFn) -> Self {
		ToSeaValueFnHolder { fun }
	}

	pub fn call(&self, json_value: serde_json::Value) -> SeaResult<sea_query::Value> {
		(self.fun)(json_value)
	}
}
// endregion: --- ToSeaValueFn

// region:    --- ToSeaConditionFn
pub type ToSeaConditionFn = fn(col: &ColumnRef, op_value: OpValValue) -> SeaResult<ConditionExpression>;

#[derive(Clone, Debug)]
pub struct ToSeaConditionFnHolder {
	fun: ToSeaConditionFn,
}

impl ToSeaConditionFnHolder {
	pub fn new(fun: ToSeaConditionFn) -> Self {
		ToSeaConditionFnHolder { fun }
	}

	pub fn call(&self, col: &ColumnRef, op_value: OpValValue) -> SeaResult<ConditionExpression> {
		(self.fun)(col, op_value)
	}
}
// endregion: --- ToSeaConditionFn
