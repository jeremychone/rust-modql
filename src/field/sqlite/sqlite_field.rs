use crate::field::{Error, FieldMeta, Result, SqliteValue};
use rusqlite::types::{FromSql, ToSql, Value, ValueRef};

#[derive(Debug, Clone)]
pub struct SqliteField {
	pub iden: &'static str,
	pub column_ref: SqliteColumnRef,
	pub value: SqliteValue,
	/// Pointer to the compile-time `FieldMeta` corresponding to this field.
	pub meta: Option<&'static FieldMeta>,
}

/// From/To Values
impl SqliteField {
	/// Return the eventual Rusqlite Value hold in this struct
	pub fn sqlite_value(&self) -> &SqliteValue {
		&self.value
	}

	/// Extract and transform the SqliteValue into the Rust Specified value that implements
	/// rusqlite::FromSql
	/// NOTE: Should be from_sqlite_value
	pub fn into_sqlite_value<T>(self) -> Result<T>
	where
		T: FromSql,
	{
		let rusqlite_value: Value = self.value.into();
		let value_ref = ValueRef::from(&rusqlite_value);

		let res = T::column_result(value_ref).map_err(|_| Error::FieldValueIntoTypeError {
			field_name: self.iden.to_string(),
		})?;

		Ok(res)
	}
}

/// Column Helpers
impl SqliteField {
	/// The column element (under `"`) for insert or udpate.
	/// Just the name, not the cast.
	pub fn sql_column(&self) -> String {
		self.meta
			.map(|m| m.sql_col_ref())
			.unwrap_or_else(|| self.column_ref.sql_col_ref())
	}

	/// The column element (under `"`) use for a item in a select statement
	/// Meaning it might have some wrapping function like `json("usage")`
	/// e.g., `"name"`, or `"project"."id."`, or `json("usage")`
	pub fn sql_column_for_select(&self) -> String {
		let col = self.sql_column();
		let prop_name = self.meta.map(|m| m.prop_name).unwrap_or(self.iden);
		SqliteField::sql_column_for_select_inner(col, prop_name, self.meta)
	}

	/// The placeholder for insert or update
	/// e.g. `?` or `jsonb(?)`
	pub fn sql_placehoder_for_write(&self) -> &'static str {
		if let Some(meta) = self.meta {
			if let Some(write_placeholder) = meta.write_placeholder {
				return write_placeholder;
			}
			match meta.cast_as {
				Some("json") => return "json(?)",
				Some("jsonb") => return "jsonb(?)",
				_ => (),
			}
		}
		"?"
	}
}

/// Constructors
impl SqliteField {
	/// NOTE: for now, we do the impl Into Rusqlite Value (cannot have blanket implementation for rusqlite value)
	pub fn new(iden: &'static str, value: impl Into<Value>) -> Self {
		let value = value.into();
		let column_ref = column_ref_from_iden(iden);
		SqliteField {
			iden,
			column_ref,
			value: value.into(),
			meta: None,
		}
	}

	/// Preferred constructor when the `FieldMeta` is known.
	/// NOTE: This one is more advanced, so take the Into SqliteValue (do a `rusqlite::Value::from(..)`
	pub fn new_with_meta(iden: &'static str, value: impl Into<SqliteValue>, meta: &'static FieldMeta) -> Self {
		let value = value.into();
		let column_ref = column_ref_from_iden(iden);

		SqliteField {
			iden,
			column_ref,
			value,
			meta: Some(meta),
		}
	}
}

/// NOTE Should be part of field meta
fn column_ref_from_iden(iden: &'static str) -> SqliteColumnRef {
	let mut parts = iden.splitn(2, '.');
	match (parts.next(), parts.next()) {
		(Some(col), None) => SqliteColumnRef { rel: None, col },
		(rel, Some(col)) => SqliteColumnRef { rel, col },
		(None, Some(col)) => SqliteColumnRef { rel: None, col },
		(None, None) => SqliteColumnRef { rel: None, col: "" },
	}
}

// region:    --- Column Ref

/// NOTE: Probably can deprecate now that we have meta (will need DynFieldMeta)
#[derive(Debug, Clone)]
pub struct SqliteColumnRef {
	pub rel: Option<&'static str>,
	pub col: &'static str,
}

impl SqliteColumnRef {
	pub fn sql_col_ref(&self) -> String {
		if let Some(rel) = self.rel {
			format!("\"{}\".\"{}\"", rel, self.col)
		} else {
			format!("\"{}\"", self.col)
		}
	}
}

// endregion: --- Column Ref

// region:    --- Crate Support

/// Column Static Helpers (To be shared with HasSqliteField and SqliteFields)
impl SqliteField {
	pub(crate) fn sql_column_for_select_inner(
		col: String,
		prop_name: &'static str,
		meta: Option<&'static FieldMeta>,
	) -> String {
		if let Some(cast_as) = meta.and_then(|m| m.cast_as) {
			if cast_as.starts_with("json") {
				return format!("json({col}) as {prop_name}",);
			}
		}
		col
	}
}

// endregion: --- Crate Support
