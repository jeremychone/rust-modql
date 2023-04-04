use crate::filter::{
	OpValBool, OpValFloat64, OpValInt64, OpValString, OpValsBool, OpValsFloat64, OpValsInt64, OpValsString,
};
use crate::{Error, Result};
use serde_json::{Number, Value};

/// Trait to go from a `operator?: Value` to the appropriate OpValue.
pub(super) trait FromJsonOpValue {
	/// e.g., `"name": "Hello World"`
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized;

	/// e.g., `{"$contains": "World", "$startsWith": "Hello"}
	fn from_json_op_value(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized;
}

// region:    --- StringOpVal
impl FromJsonOpValue for OpValString {
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		match value {
			Value::String(string_v) => Ok(OpValString::Eq(string_v)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		// FIXME: Needs to do the In/Array patterns.
		let ov = match (op, value) {
			("$eq", Value::String(string_v)) => OpValString::Eq(string_v),
			("$not", Value::String(string_v)) => OpValString::Not(string_v),

			("$contains", Value::String(string_v)) => OpValString::Contains(string_v),
			("$notContains", Value::String(string_v)) => OpValString::NotContains(string_v),

			("$startsWith", Value::String(string_v)) => OpValString::StartsWith(string_v),
			("$notStartsWith", Value::String(string_v)) => OpValString::NotStartsWith(string_v),

			("$endsWith", Value::String(string_v)) => OpValString::EndsWith(string_v),
			("$notEndsWith", Value::String(string_v)) => OpValString::NotEndsWith(string_v),

			("$lt", Value::String(string_v)) => OpValString::Lt(string_v),
			("$lte", Value::String(string_v)) => OpValString::Lte(string_v),

			("$gt", Value::String(string_v)) => OpValString::Gt(string_v),
			("$gte", Value::String(string_v)) => OpValString::Gte(string_v),

			(_, v) => return Err(Error::JsonOpValNotSupported(op.to_string(), v)),
		};
		Ok(ov)
	}
}
// endregion: --- StringOpVal

// region:    --- IntOpVal
/// match a the op_value
impl FromJsonOpValue for OpValInt64 {
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		match value {
			Value::Number(num) => Ok(OpValInt64::Eq(as_i64(num)?)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		// FIXME: Needs to do the In/Array patterns.
		let ov = match (op, value) {
			("$eq", Value::Number(num)) => OpValInt64::Eq(as_i64(num)?),
			("$not", Value::Number(num)) => OpValInt64::Not(as_i64(num)?),

			("$lt", Value::Number(num)) => OpValInt64::Lt(as_i64(num)?),
			("$lte", Value::Number(num)) => OpValInt64::Lte(as_i64(num)?),

			("$gt", Value::Number(num)) => OpValInt64::Gt(as_i64(num)?),
			("$gte", Value::Number(num)) => OpValInt64::Gte(as_i64(num)?),

			(_, value) => return Err(Error::JsonOpValNotSupported(op.to_string(), value)),
		};

		Ok(ov)
	}
}
// endregion: --- IntOpVal

// region:    --- FloatOpVal
/// match a the op_value
impl FromJsonOpValue for OpValFloat64 {
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		match value {
			Value::Number(num) => Ok(OpValFloat64::Eq(as_f64(num)?)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		// FIXME: Needs to do the In/Array patterns.
		let ov = match (op, value) {
			("$eq", Value::Number(num)) => OpValFloat64::Eq(as_f64(num)?),
			("$not", Value::Number(num)) => OpValFloat64::Not(as_f64(num)?),

			("$lt", Value::Number(num)) => OpValFloat64::Lt(as_f64(num)?),
			("$lte", Value::Number(num)) => OpValFloat64::Lte(as_f64(num)?),

			("$gt", Value::Number(num)) => OpValFloat64::Gt(as_f64(num)?),
			("$gte", Value::Number(num)) => OpValFloat64::Gte(as_f64(num)?),

			(_, value) => return Err(Error::JsonOpValNotSupported(op.to_string(), value)),
		};

		Ok(ov)
	}
}
// endregion: --- FloatOpVal

// region:    --- BoolOpVal
/// match a the op_value
impl FromJsonOpValue for OpValBool {
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		match value {
			Value::Bool(v) => Ok(OpValBool::Eq(v)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		let ov = match (op, value) {
			("$eq", Value::Bool(v)) => OpValBool::Eq(v),
			("$not", Value::Bool(v)) => OpValBool::Not(v),
			(_, value) => return Err(Error::JsonOpValNotSupported(op.to_string(), value)),
		};

		Ok(ov)
	}
}
// endregion: --- BoolOpVal

// region:    --- TryFrom<Value> for OpVals

// Common implementation
macro_rules! impl_try_from_value_for_opvals {
	($($ov:ident, $ovs:ident),*) => {
		$(
impl TryFrom<Value> for $ovs {
	type Error = Error;

	fn try_from(value: Value) -> Result<Self> {
		let mut ovs = Vec::new();

		match value {
			// e.g. {"$contains": "World", "$startsWith": "Hello"}
			Value::Object(obj) => {
				for (k, v) in obj.into_iter() {
					let ov = $ov::from_json_op_value(&k, v)?;
					ovs.push(ov);
				}
			}
			// If value not an object, assume it is a scalar value for ::Eq(...)
			// e.g. "Hello World" (which will give $ov::eq("Hello World"..))
			value => ovs.push($ov::from_json_scalar_value(value)?),
		}
		Ok($ovs(ovs))
	}
}
		)*
	};
}

// For all opvals (must specified the pair as macro rules are hygienic)
impl_try_from_value_for_opvals!(
	OpValString,
	OpValsString,
	OpValInt64,
	OpValsInt64,
	OpValFloat64,
	OpValsFloat64,
	OpValBool,
	OpValsBool
);

// BoolOpVal,
// BoolOpVals

// endregion: --- TryFrom<Value> for OpVals

// region:    --- Helpers
fn as_i64(num: Number) -> Result<i64> {
	num.as_i64().ok_or(Error::JsonValNotOfType("i64"))
}

fn as_f64(num: Number) -> Result<f64> {
	num.as_f64().ok_or(Error::JsonValNotOfType("f64"))
}
// endregion: --- Helpers
