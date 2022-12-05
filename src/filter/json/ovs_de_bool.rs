use super::ovs_json::FromJsonOpValue;
use crate::{BoolOpVal, BoolOpVals, IntOpVal, IntOpVals, StringOpVal};
use serde::ser::SerializeMap;
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;

impl<'de> Deserialize<'de> for BoolOpVals {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(BoolOpValsVisitor)
	}
}

struct BoolOpValsVisitor;

impl<'de> Visitor<'de> for BoolOpValsVisitor {
	type Value = BoolOpVals; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "BoolOpValsVisitor visitor not implemented for this type.")
	}

	fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(BoolOpVal::Eq(v).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<BoolOpVal> = Vec::new();

		while let Some(k) = map.next_key::<&str>()? {
			// Note: Important to always
			let value = map.next_value::<Value>()?;
			let opval = BoolOpVal::from_json_op_value(k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(BoolOpVals(opvals))
	}
}
