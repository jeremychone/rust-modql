use crate::filter::*;
use crate::{Error, Result};
use serde_json::{Number, Value};

/// Trait to go from a `operator?: Value` to the appropriate OpValue.
pub(super) trait FromJsonToOpVal {
	/// e.g., `"name": "Hello World"`
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized;

	/// e.g., `{"$contains": "World", "$startsWith": "Hello"}
	fn from_json_opvals_value(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized;
}

// region:    --- StringOpVal
impl FromJsonToOpVal for OpValString {
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		match value {
			Value::String(string_v) => Ok(OpValString::Eq(string_v)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_opvals_value(op: &str, value: Value) -> Result<Self>
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

// region:    --- FromJsonToOpVal for Nums

// - `ov` e.g., `OpValInt64`
// - `asfn` e.g., `as_i64`
macro_rules! from_json_to_opval_num{
	($(($ov:ident, $asfn:expr)),+) => {
		$(

/// match a the op_value
impl FromJsonToOpVal for $ov {
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		match value {
			Value::Number(num) => Ok($ov::Eq($asfn(num)?)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_opvals_value(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		// FIXME: Needs to do the In/Array patterns.
		let ov = match (op, value) {
			("$eq", Value::Number(num)) => $ov::Eq($asfn(num)?),
			("$not", Value::Number(num)) => $ov::Not($asfn(num)?),

			("$lt", Value::Number(num)) => $ov::Lt($asfn(num)?),
			("$lte", Value::Number(num)) => $ov::Lte($asfn(num)?),

			("$gt", Value::Number(num)) => $ov::Gt($asfn(num)?),
			("$gte", Value::Number(num)) => $ov::Gte($asfn(num)?),

			(_, value) => return Err(Error::JsonOpValNotSupported(op.to_string(), value)),
		};

		Ok(ov)
	}
}
		)+
	};
}

from_json_to_opval_num!(
	(OpValUint64, as_u64),
	(OpValUint32, as_u32),
	(OpValInt64, as_i64),
	(OpValInt32, as_i32),
	(OpValFloat64, as_f64),
	(OpValFloat32, as_f32)
);
// endregion: --- FromJsonToOpVal for Nums

// region:    --- BoolOpVal
/// match a the op_value
impl FromJsonToOpVal for OpValBool {
	fn from_json_scalar_value(value: Value) -> Result<Self>
	where
		Self: Sized,
	{
		match value {
			Value::Bool(v) => Ok(OpValBool::Eq(v)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_opvals_value(op: &str, value: Value) -> Result<Self>
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
	($(($ov:ident, $ovs:ident)),*) => {
		$(

impl TryFrom<Value> for $ovs {
	type Error = Error;

	fn try_from(value: Value) -> Result<Self> {
		let mut ovs = Vec::new();

		match value {
			// e.g. {"$contains": "World", "$startsWith": "Hello"}
			Value::Object(obj) => {
				for (k, v) in obj.into_iter() {
					let ov = $ov::from_json_opvals_value(&k, v)?;
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
	(OpValString, OpValsString),
	(OpValUint64, OpValsUint64),
	(OpValUint32, OpValsUint32),
	(OpValInt64, OpValsInt64),
	(OpValInt32, OpValsInt32),
	(OpValFloat64, OpValsFloat64),
	(OpValBool, OpValsBool)
);

// endregion: --- TryFrom<Value> for OpVals

// region:    --- Helpers
fn as_u64(num: Number) -> Result<u64> {
	num.as_u64().ok_or(Error::JsonValNotOfType("u64"))
}

fn as_u32(num: Number) -> Result<u32> {
	num.as_u64().map(|n| n as u32).ok_or(Error::JsonValNotOfType("u32"))
}

fn as_i64(num: Number) -> Result<i64> {
	num.as_i64().ok_or(Error::JsonValNotOfType("i64"))
}

fn as_i32(num: Number) -> Result<i32> {
	num.as_i64().map(|n| n as i32).ok_or(Error::JsonValNotOfType("i32"))
}

fn as_f64(num: Number) -> Result<f64> {
	num.as_f64().ok_or(Error::JsonValNotOfType("f64"))
}

fn as_f32(num: Number) -> Result<f32> {
	num.as_f64().map(|n| n as f32).ok_or(Error::JsonValNotOfType("f32"))
}
// endregion: --- Helpers
