use crate::field::{Error, FieldMeta, Result};
use rusqlite::types::{FromSql, ToSql, Value, ValueRef};

/// NOTE: Probably can deprecate now that we have meta (will need DynFieldMeta)
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
	/// Pointer to the compile-time `FieldMeta` corresponding to this field.
	pub meta: Option<&'static FieldMeta>,
}

impl SqliteField {
	/// Return the eventual Rusqlite Value hold in this struct
	pub fn sqlite_value(&self) -> &Value {
		&self.value
	}

	/// Extract and transform the Rusqlite Value into the specified type
	pub fn into_sqlite_value<T>(self) -> Result<T>
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

// region:    --- Constructors

impl SqliteField {
	pub fn new(iden: &'static str, value: Value) -> Self {
		let column_ref = column_ref_from_iden(iden);
		SqliteField {
			iden,
			column_ref,
			value,
			meta: None,
		}
	}

	/// Preferred constructor when the `FieldMeta` is known.
	pub fn new_with_options_meta(iden: &'static str, value: Value, meta: &'static FieldMeta) -> Self {
		let column_ref = column_ref_from_iden(iden);

		SqliteField {
			iden,
			column_ref,
			value,
			meta: Some(meta),
		}
	}
}

/// NOTE SHould be part of field meta
fn column_ref_from_iden(iden: &'static str) -> SqliteColumnRef {
	let mut parts = iden.splitn(2, '.');
	match (parts.next(), parts.next()) {
		(Some(col), None) => SqliteColumnRef { rel: None, col },
		(rel, Some(col)) => SqliteColumnRef { rel, col },
		(None, Some(col)) => SqliteColumnRef { rel: None, col },
		(None, None) => SqliteColumnRef { rel: None, col: "" },
	}
}

// endregion: --- Constructors
