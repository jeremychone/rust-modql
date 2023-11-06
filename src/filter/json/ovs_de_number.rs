use super::ovs_json::OpValueToOpValType;
use crate::filter::{OpValFloat64, OpValInt32, OpValInt64, OpValsFloat64, OpValsInt32, OpValsInt64};
use serde::{de::MapAccess, de::Visitor, Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;

// region:    --- OpValsInt64
impl<'de> Deserialize<'de> for OpValsInt64 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(Int64OpValsVisitor)
	}
}

struct Int64OpValsVisitor;

impl<'de> Visitor<'de> for Int64OpValsVisitor {
	type Value = OpValsInt64; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "OpValsInt64 visitor not implemented for this type.")
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValInt64::Eq(v).into())
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValInt64::Eq(v as i64).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<OpValInt64> = Vec::new();
		while let Some(k) = map.next_key::<String>()? {
			let value = map.next_value::<Value>()?;
			let opval = OpValInt64::op_value_to_op_val_type(&k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(OpValsInt64(opvals))
	}
}
// endregion: --- OpValsInt64

// region:    --- OpValsInt32
impl<'de> Deserialize<'de> for OpValsInt32 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(Int32OpValsVisitor)
	}
}

struct Int32OpValsVisitor;

impl<'de> Visitor<'de> for Int32OpValsVisitor {
	type Value = OpValsInt32; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "OpValsInt32 visitor not implemented for this type.")
	}

	fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValInt32::Eq(v).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<OpValInt32> = Vec::new();

		while let Some(k) = map.next_key::<String>()? {
			// Note: Important to always
			let value = map.next_value::<Value>()?;
			let opval = OpValInt32::op_value_to_op_val_type(&k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(OpValsInt32(opvals))
	}
}
// endregion: --- OpValsInt64

// region:    --- OpValsFloat64
impl<'de> Deserialize<'de> for OpValsFloat64 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(FloatOpValsVisitor)
	}
}

struct FloatOpValsVisitor;

impl<'de> Visitor<'de> for FloatOpValsVisitor {
	type Value = OpValsFloat64; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "OpValsFloat64 visitor not implemented for this type.")
	}

	fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValFloat64::Eq(v).into())
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(OpValFloat64::Eq(v as f64).into())
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut opvals: Vec<OpValFloat64> = Vec::new();

		while let Some(k) = map.next_key::<String>()? {
			// Note: Important to always
			let value = map.next_value::<Value>()?;
			let opval = OpValFloat64::op_value_to_op_val_type(&k, value).map_err(serde::de::Error::custom)?;
			opvals.push(opval)
		}

		Ok(OpValsFloat64(opvals))
	}
}
// endregion: --- OpValsFloat64
