use crate::field::{Error, Result};
use rusqlite::types::FromSql;
use rusqlite::types::ToSql;
use rusqlite::types::Value;
use rusqlite::types::ValueRef;

#[derive(Debug, Clone)]
pub struct SqliteColumnRef {
	pub rel: Option<&'static str>,
	pub col: &'static str,
}

#[derive(Debug, Clone)]
pub struct SqliteField {
	pub iden: &'static str,
	pub column_ref: SqliteColumnRef,
	pub value: Value,
	pub options: LiteFieldOptions,
}

impl SqliteField {
	/// Return the eventual Rusqlite Value hold in this struct
	pub fn lite_value(&self) -> &Value {
		&self.value
	}

	/// Extract and transofrm the Rusqlite Value into the specified type
	pub fn into_lite_value<T>(self) -> Result<T>
	where
		T: FromSql,
	{
		let value_ref = ValueRef::from(&self.value);

		let res = T::column_result(value_ref).map_err(|_| Error::FieldValueIntoTypeError {
			field_name: self.iden.to_string(),
		})?;

		Ok(res)
	}
}

/// TODO: Might ant to have a common option (rather than SeaFieldOptions, ...)
#[derive(Debug, Default, Clone)]
pub struct LiteFieldOptions {
	pub cast_as: Option<String>,
}

impl SqliteField {
	/// TODO: Need to take a FromSql probably for Value
	pub fn new(iden: &'static str, value: Value) -> Self {
		Self::new_concrete(iden, value)
	}

	/// The concrete version of the new.
	pub fn new_concrete(iden: &'static str, value: Value) -> Self {
		let mut parts = iden.splitn(2, '.');
		let column_ref = match (parts.next(), parts.next()) {
			(Some(col), None) => SqliteColumnRef { rel: None, col },
			(rel, Some(col)) => SqliteColumnRef { rel, col },
			// this should not happen (but do no break)
			(None, Some(col)) => SqliteColumnRef { rel: None, col },
			// for now, we make the colum empty
			(None, None) => SqliteColumnRef { rel: None, col: "" },
		};

		SqliteField {
			iden,
			column_ref,
			value,
			options: LiteFieldOptions::default(),
		}
	}

	pub fn new_with_options(iden: &'static str, value: Value, options: LiteFieldOptions) -> Self {
		/// TODO: need to refactor that to make it more idiomatic
		let mut field = Self::new_concrete(iden, value);
		field.options = options;
		field
	}
}

// region:    --- Froms

impl From<(&'static str, Value)> for SqliteField {
	fn from(val: (&'static str, Value)) -> Self {
		SqliteField::new(val.0, val.1)
	}
}

// endregion: --- Froms
