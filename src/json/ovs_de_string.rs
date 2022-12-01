use super::ovs_json::FromJsonOpValue;
use crate::{IntOpVal, IntOpVals, StringOpVal, StringOpVals};
use serde::ser::SerializeMap;
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;

impl<'de> Deserialize<'de> for StringOpVals {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(StringOpValsVisitor)
	}
}

struct StringOpValsVisitor;

impl<'de> Visitor<'de> for StringOpValsVisitor {
	type Value = StringOpVals; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "StringOpValsVisitor visitor not implemented for this type.")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		println!("->> visit_str {v}");
		Ok(StringOpVal::Eq(v.to_string()).into())
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(StringOpVal::Eq(v).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<StringOpVal> = Vec::new();

		// Note: If use next_key::<&str>, error "invalid type: string \"$contains\", expected a borrowed string"
		//       when using in app code (works in unit test somehow). Must have a good reason, just not sure yet.
		//       So, using <String> works.
		while let Some(k) = map.next_key::<String>()? {
			// Note: Important to always call next_value
			let value = map.next_value::<Value>()?;
			let opval = StringOpVal::from_json_op_value(&k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(StringOpVals(opvals))
	}
}
