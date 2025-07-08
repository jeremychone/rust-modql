use crate::field::{SqliteField, SqliteValue};
use rusqlite::ToSql;
use rusqlite::types::Value as RusqliteValue;

#[derive(Debug, Clone, Default)]
pub struct SqliteFields(Vec<SqliteField>);

// Constructor
impl SqliteFields {
	pub fn new(fields: Vec<SqliteField>) -> Self {
		SqliteFields(fields)
	}
}

// Setters & Getters
impl SqliteFields {
	/// Simple api to append a SeaField to the list.
	pub fn push(&mut self, field: SqliteField) {
		self.0.push(field);
	}

	/// The consuming builder API equivalent to `push(..)`
	pub fn append(mut self, field: SqliteField) -> Self {
		self.push(field);
		self
	}

	pub fn into_fields(self) -> Vec<SqliteField> {
		self.0
	}

	pub fn fields(&self) -> Vec<&SqliteField> {
		self.0.iter().collect()
	}

	/// Alias to self.unzip()
	pub fn for_insert(self) -> (Vec<&'static str>, Vec<SqliteValue>) {
		self.0.into_iter().map(|f| (f.iden, f.value)).unzip()
	}

	/// Alias to self.zip()
	pub fn for_update(self) -> Vec<(&'static str, SqliteValue)> {
		self.0.into_iter().map(|f| (f.iden, f.value)).collect()
	}
}

/// SQL Convenient method
impl SqliteFields {
	/// will return a string like `"id", "name", "content"` for each member of the field
	pub fn sql_columns(&self) -> String {
		/// TODO: needs to handle when rel.col (should use col ref)
		self.0.iter().map(|f| f.sql_column()).collect::<Vec<_>>().join(", ")
	}

	pub fn sql_columns_for_select(&self) -> String {
		self.0.iter().map(|f| f.sql_column_for_select()).collect::<Vec<_>>().join(", ")
	}

	/// Will return a string like `?, ?, ?` for each member of the field
	pub fn sql_placeholders(&self) -> String {
		let mut buf: Vec<&str> = Vec::new();
		for field in self.0.iter() {
			buf.push(field.sql_placehoder_for_write())
		}
		buf.join(", ")
	}

	/// Will return `"id" = ?, "name" = ?, "content" = ?`
	/// TODO: Need to support the rel.col
	pub fn sql_setters(&self) -> String {
		let mut buf: Vec<String> = Vec::new();
		for field in self.0.iter() {
			let placeholder = field.sql_placehoder_for_write();
			buf.push(format!("\"{}\" = {placeholder}", field.iden))
		}
		buf.join(", ")
	}

	pub fn into_values(self) -> Vec<SqliteValue> {
		self.0.into_iter().map(|f| f.value).collect()
	}

	pub fn values_as_dyn_to_sql_vec(&self) -> Vec<&dyn ToSql> {
		self.0.iter().map(|f| (&f.value) as &dyn ToSql).collect()
	}
}

impl IntoIterator for SqliteFields {
	type Item = SqliteField;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

// region:    --- Froms

impl From<Vec<SqliteField>> for SqliteFields {
	fn from(val: Vec<SqliteField>) -> Self {
		SqliteFields(val)
	}
}

impl From<SqliteField> for SqliteFields {
	fn from(val: SqliteField) -> Self {
		SqliteFields(vec![val])
	}
}

// endregion: --- Froms
