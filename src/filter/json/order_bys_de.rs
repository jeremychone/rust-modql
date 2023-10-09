use crate::filter::{OrderBy, OrderBys};
use serde::de::SeqAccess;
use serde::{de, Deserialize, Deserializer};
use std::fmt;

impl<'de> Deserialize<'de> for OrderBys {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_any(OrderBysVisitor)
	}
}

struct OrderBysVisitor;

impl<'de> de::Visitor<'de> for OrderBysVisitor {
	type Value = OrderBys; // for deserialize

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "OrderBysVisitor visitor not implemented for this type.")
	}

	fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		Ok(OrderBy::from(v).into())
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		Ok(OrderBy::from(v.to_string()).into())
	}

	fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: de::SeqAccess<'de>,
	{
		let mut order_bys: Vec<OrderBy> = Vec::new();

		while let Some(string) = seq.next_element::<String>()? {
			order_bys.push(OrderBy::from(string));
		}

		Ok(OrderBys::new(order_bys))
	}
	// FIXME: Needs to add support for visit_seq
}
