use serde_json::{Number, Value};

use crate::{BoolOpVal, BoolOpVals, Error, FloatOpVal, FloatOpVals, IntOpVal, IntOpVals, StringOpVal, StringOpVals};

/// Trait to go from a `operator?: Value` to the appropriate OpValue.
pub trait FromJsonOpValue {
	/// e.g., `"name": "Hello World"`
	fn from_json_scalar_value(value: Value) -> Result<Self, Error>
	where
		Self: Sized;

	/// e.g., `{"$contains": "World", "$startsWith": "Hello"}
	fn from_json_op_value(op: &str, value: Value) -> Result<Self, Error>
	where
		Self: Sized;
}

// region:    --- StringOpVal
impl FromJsonOpValue for StringOpVal {
	fn from_json_scalar_value(value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		match value {
			Value::String(string_v) => Ok(StringOpVal::Eq(string_v)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let ov = match (op, value) {
			("$eq", Value::String(string_v)) => StringOpVal::Eq(string_v),
			("$not", Value::String(string_v)) => StringOpVal::Not(string_v),

			("$contains", Value::String(string_v)) => StringOpVal::Contains(string_v),
			("$notContains", Value::String(string_v)) => StringOpVal::NotContains(string_v),

			("$startsWith", Value::String(string_v)) => StringOpVal::StartsWith(string_v),
			("$notStartsWith", Value::String(string_v)) => StringOpVal::NotStartsWith(string_v),

			("$endsWith", Value::String(string_v)) => StringOpVal::EndsWith(string_v),
			("$notEndsWith", Value::String(string_v)) => StringOpVal::NotEndsWith(string_v),

			("$lt", Value::String(string_v)) => StringOpVal::Lt(string_v),
			("$lte", Value::String(string_v)) => StringOpVal::Lte(string_v),

			("$gt", Value::String(string_v)) => StringOpVal::Gt(string_v),
			("$gte", Value::String(string_v)) => StringOpVal::Gte(string_v),

			(_, v) => return Err(Error::JsonOpValNotSupported(op.to_string(), v)),
		};
		Ok(ov)
	}
}
// endregion: --- StringOpVal

// region:    --- IntOpVal
/// match a the op_value
impl FromJsonOpValue for IntOpVal {
	fn from_json_scalar_value(value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		match value {
			Value::Number(num) => Ok(IntOpVal::Eq(as_i64(num)?)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let ov = match (op, value) {
			("$eq", Value::Number(num)) => IntOpVal::Eq(as_i64(num)?),
			("$not", Value::Number(num)) => IntOpVal::Not(as_i64(num)?),

			("$lt", Value::Number(num)) => IntOpVal::Lt(as_i64(num)?),
			("$lte", Value::Number(num)) => IntOpVal::Lte(as_i64(num)?),

			("$gt", Value::Number(num)) => IntOpVal::Gt(as_i64(num)?),
			("$gte", Value::Number(num)) => IntOpVal::Gte(as_i64(num)?),

			(_, value) => return Err(Error::JsonOpValNotSupported(op.to_string(), value)),
		};

		Ok(ov)
	}
}
// endregion: --- IntOpVal

// region:    --- FloatOpVal
/// match a the op_value
impl FromJsonOpValue for FloatOpVal {
	fn from_json_scalar_value(value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		match value {
			Value::Number(num) => Ok(FloatOpVal::Eq(as_f64(num)?)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let ov = match (op, value) {
			("$eq", Value::Number(num)) => FloatOpVal::Eq(as_f64(num)?),
			("$not", Value::Number(num)) => FloatOpVal::Not(as_f64(num)?),

			("$lt", Value::Number(num)) => FloatOpVal::Lt(as_f64(num)?),
			("$lte", Value::Number(num)) => FloatOpVal::Lte(as_f64(num)?),

			("$gt", Value::Number(num)) => FloatOpVal::Gt(as_f64(num)?),
			("$gte", Value::Number(num)) => FloatOpVal::Gte(as_f64(num)?),

			(_, value) => return Err(Error::JsonOpValNotSupported(op.to_string(), value)),
		};

		todo!()
	}
}
// endregion: --- FloatOpVal

// region:    --- BoolOpVal
/// match a the op_value
impl FromJsonOpValue for BoolOpVal {
	fn from_json_scalar_value(value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		match value {
			Value::Bool(v) => Ok(BoolOpVal::Eq(v)),
			v => Err(Error::JsonOpValNotSupported("".to_string(), v)),
		}
	}

	fn from_json_op_value(op: &str, value: Value) -> Result<Self, Error>
	where
		Self: Sized,
	{
		let ov = match (op, value) {
			("$eq", Value::Bool(v)) => BoolOpVal::Eq(v),
			("$not", Value::Bool(v)) => BoolOpVal::Not(v),
			(_, value) => return Err(Error::JsonOpValNotSupported(op.to_string(), value)),
		};

		todo!()
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

	fn try_from(value: Value) -> Result<Self, Error> {
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
	StringOpVal,
	StringOpVals,
	IntOpVal,
	IntOpVals,
	FloatOpVal,
	FloatOpVals,
	BoolOpVal,
	BoolOpVals
);

// BoolOpVal,
// BoolOpVals

// endregion: --- TryFrom<Value> for OpVals

// region:    --- Helpers
fn as_i64(num: Number) -> Result<i64, Error> {
	num.as_i64().ok_or(Error::JsonValNotOfType("i64"))
}

fn as_f64(num: Number) -> Result<f64, Error> {
	num.as_f64().ok_or(Error::JsonValNotOfType("f64"))
}
// endregion: --- Helpers
