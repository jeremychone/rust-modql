use crate::field::{Error, Result};
use crate::sea_utils::StringIden;
use crate::SIden;
use sea_query::{ColumnRef, DynIden, SimpleExpr, Value};
use sea_query::{IntoIden, ValueType};

#[derive(Debug, Clone)]
pub struct SeaField {
	pub iden: DynIden,
	pub column_ref: ColumnRef,
	pub value: SimpleExpr,
}

impl SeaField {
	pub fn sea_value(&self) -> Option<&Value> {
		if let SimpleExpr::Value(value) = &self.value {
			Some(value)
		} else {
			None
		}
	}

	pub fn value_into<T>(self) -> Result<T>
	where
		T: ValueType,
	{
		let SimpleExpr::Value(value) = self.value else {
			return Err(Error::FieldValueNotSeaValue);
		};

		T::try_from(value).map_err(|_| Error::FieldValueIntoTypeError {
			field_name: self.iden.to_string(),
		})
	}
}

#[derive(Default, Debug)]
pub struct FieldOptions {
	pub cast_as: Option<String>,
}

impl SeaField {
	/// Create a new SeaField from an `IntoIden` and `Into<SimpleExpr>` for the value
	pub fn new(iden: impl IntoIden, value: impl Into<SimpleExpr>) -> Self {
		let iden = iden.into_iden();
		let column_ref = ColumnRef::Column(iden.clone());
		SeaField {
			iden,
			column_ref,
			value: value.into(),
		}
	}

	/// Create a new SeaField for a static column name and a `Into<SimpleExpr>` for the value
	pub fn siden(iden: &'static str, value: impl Into<SimpleExpr>) -> Self {
		let iden = SIden(iden).into_iden();
		let column_ref = ColumnRef::Column(iden.clone());
		SeaField {
			iden,
			column_ref,
			value: value.into(),
		}
	}

	pub fn new_with_options(iden: impl IntoIden, value: SimpleExpr, options: FieldOptions) -> Self {
		let iden = iden.into_iden();
		let column_ref = ColumnRef::Column(iden.clone());
		let mut value = value;
		if let Some(cast_as) = options.cast_as {
			value = value.cast_as(StringIden(cast_as))
		}

		SeaField {
			iden,
			column_ref,
			value,
		}
	}
}

// region:    --- Froms

impl From<(&'static str, SimpleExpr)> for SeaField {
	fn from(val: (&'static str, SimpleExpr)) -> Self {
		SeaField::new(SIden(val.0), val.1)
	}
}

// endregion: --- Froms
