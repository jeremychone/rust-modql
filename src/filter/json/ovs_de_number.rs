use super::ovs_json::FromJsonOpValue;
use crate::filter::{FloatOpVal, FloatOpVals, IntOpVal, IntOpVals};
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

// region:    --- IntOpVals
impl<'de> Deserialize<'de> for IntOpVals {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(IntOpValsVisitor)
	}
}

struct IntOpValsVisitor;

impl<'de> Visitor<'de> for IntOpValsVisitor {
	type Value = IntOpVals; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "IntOpValsVisitor visitor not implemented for this type.")
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(IntOpVal::Eq(v).into())
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(IntOpVal::Eq(v as i64).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<IntOpVal> = Vec::new();

		while let Some(k) = map.next_key::<&str>()? {
			// Note: Important to always
			let value = map.next_value::<Value>()?;
			let opval = IntOpVal::from_json_op_value(k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(IntOpVals(opvals))
	}
}
// endregion: --- IntOpVals

// region:    --- FloatOpVals
impl<'de> Deserialize<'de> for FloatOpVals {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(FloatOpValsVisitor)
	}
}

struct FloatOpValsVisitor;

impl<'de> Visitor<'de> for FloatOpValsVisitor {
	type Value = FloatOpVals; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "FloatOpValsVisitor visitor not implemented for this type.")
	}

	fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(FloatOpVal::Eq(v).into())
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(FloatOpVal::Eq(v as f64).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<FloatOpVal> = Vec::new();

		while let Some(k) = map.next_key::<&str>()? {
			// Note: Important to always
			let value = map.next_value::<Value>()?;
			let opval = FloatOpVal::from_json_op_value(k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(FloatOpVals(opvals))
	}
}
// endregion: --- FloatOpVals
