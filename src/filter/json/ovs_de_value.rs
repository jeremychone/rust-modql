use crate::filter::json::ovs_json::OpValueToOpValType;
use crate::filter::{OpValValue, OpValsValue};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

impl<'de> Deserialize<'de> for OpValsValue {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let v: Value = Deserialize::deserialize(deserializer)?;

		let op_vals_value: OpValsValue = if v.is_number() || v.is_boolean() || v.is_string() {
			OpValValue::Eq(v).into()
		} else if v.is_object() {
			let mut opvals: Vec<OpValValue> = Vec::new();
			let Value::Object(obj) = v else {
				return Err(serde::de::Error::custom("OpValValue should be object"));
			};

			for (key, value) in obj.into_iter() {
				let op_val = OpValValue::op_value_to_op_val_type(&key, value).map_err(serde::de::Error::custom)?;
				opvals.push(op_val);
			}
			OpValsValue(opvals)
		} else {
			return Err(serde::de::Error::custom(
				"OpValJson value mut be either number, bool, string, or an Object",
			));
		};

		Ok(op_vals_value)
	}
}
