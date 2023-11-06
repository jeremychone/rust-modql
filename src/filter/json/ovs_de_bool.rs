use super::ovs_json::OpValueToOpValType;
use crate::filter::{OpValBool, OpValsBool};
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

impl<'de> Deserialize<'de> for OpValsBool {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(BoolOpValsVisitor)
	}
}

struct BoolOpValsVisitor;

impl<'de> Visitor<'de> for BoolOpValsVisitor {
	type Value = OpValsBool; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "BoolOpValsVisitor visitor not implemented for this type.")
	}

	fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValBool::Eq(v).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<OpValBool> = Vec::new();

		while let Some(k) = map.next_key::<String>()? {
			// Note: Important to always call next_value
			let value = map.next_value::<Value>()?;
			let opval = OpValBool::op_value_to_op_val_type(&k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(OpValsBool(opvals))
	}
}
