use serde_json::{Number, Value};

use crate::{Error, IntOpVal, IntOpVals, StringOpVal, StringOpVals};

/// match a the op_value
/// val: "Hello World" // StringOpValue::Eq("Hello World")
/// or
/// val: {
///   "$contains": "World", // StringOpValue::Contains("World"),
///   "$startsWith": "Hello", // StringOpValue::StartsWith("Hello")
/// }
impl TryFrom<Value> for StringOpVals {
	type Error = Error;

	fn try_from(val: Value) -> Result<Self, Error> {
		let mut ovs = Vec::new();

		fn get_ov(k: String, v: Value) -> Result<StringOpVal, Error> {
			let ov = match (k.as_str(), v) {
				("$eq", Value::String(string_v)) => StringOpVal::Eq(string_v),
				("$not", Value::String(string_v)) => StringOpVal::Not(string_v),

				("$contains", Value::String(string_v)) => StringOpVal::Contains(string_v),
				("$notContains", Value::String(string_v)) => StringOpVal::NotContains(string_v),

				("$startsWith", Value::String(string_v)) => StringOpVal::StartsWith(string_v),
				("$notStartsWith", Value::String(string_v)) => StringOpVal::NotStartsWith(string_v),

				("$endsWith", Value::String(string_v)) => StringOpVal::EndsWith(string_v),
				("$notEndsWith", Value::String(string_v)) => StringOpVal::NotEndsWith(string_v),

				(_, v) => return Err(Error::JsonOpValNotSupported(k, v)),
			};
			Ok(ov)
		}

		match val {
			Value::String(string_v) => ovs.push(StringOpVal::Eq(string_v)),
			Value::Object(obj) => {
				for (k, v) in obj.into_iter() {
					let ov = get_ov(k, v)?;
					ovs.push(ov);
				}
			}
			_ => panic!("StringOpVal value can only be string or object"),
		}
		Ok(StringOpVals(ovs))
	}
}

/// match a the op_value
impl TryFrom<Value> for IntOpVals {
	type Error = Error;
	fn try_from(val: Value) -> Result<Self, Error> {
		let mut ovs = Vec::new();

		// Get a IntOpValue from k, v.
		// Note: Compartmentalize to isolate logic, and make it simpler later
		//       if we want to return a list of errors (and not failed at first)
		fn get_ov(k: String, v: Value) -> Result<IntOpVal, Error> {
			let ov = match (k.as_str(), v) {
				("$eq", Value::Number(num)) => IntOpVal::Eq(as_i64(num)?),
				("$not", Value::Number(num)) => IntOpVal::Not(as_i64(num)?),
				(_, v) => return Err(Error::JsonOpValNotSupported(k, v)),
			};
			Ok(ov)
		}

		match val {
			Value::Number(num) => ovs.push(IntOpVal::Eq(as_i64(num)?)),
			Value::Object(obj) => {
				for (k, v) in obj.into_iter() {
					let ov = get_ov(k, v)?;
					ovs.push(ov);
				}
			}
			_ => panic!("StringOpVal value can only be string or object"),
		}
		Ok(IntOpVals(ovs))
	}
}

fn as_i64(num: Number) -> Result<i64, Error> {
	num.as_i64().ok_or(Error::JsonValNotOfType("i64"))
}

fn as_f64(num: Number) -> Result<f64, Error> {
	num.as_f64().ok_or(Error::JsonValNotOfType("f64"))
}
