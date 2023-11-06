use super::ovs_json::OpValueToOpValType;
use crate::filter::{OpValString, OpValsString};
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

impl<'de> Deserialize<'de> for OpValsString {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(StringOpValsVisitor)
	}
}

struct StringOpValsVisitor;

impl<'de> Visitor<'de> for StringOpValsVisitor {
	type Value = OpValsString; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "StringOpValsVisitor visitor not implemented for this type.")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValString::Eq(v.to_string()).into())
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValString::Eq(v).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<OpValString> = Vec::new();

		// Note: If use next_key::<&str>, error "invalid type: string \"$contains\", expected a borrowed string"
		//       so using String for now.
		while let Some(k) = map.next_key::<String>()? {
			// Note: Important to always call next_value
			let value = map.next_value::<Value>()?;
			let opval = OpValString::op_value_to_op_val_type(&k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(OpValsString(opvals))
	}
}
