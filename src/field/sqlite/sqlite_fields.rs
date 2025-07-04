use crate::field::SqliteField;
use rusqlite::types::Value;
use rusqlite::ToSql;

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

	pub fn into_vec(self) -> Vec<SqliteField> {
		self.0
	}

	/// Alias to self.unzip()
	pub fn for_insert(self) -> (Vec<&'static str>, Vec<Value>) {
		self.0.into_iter().map(|f| (f.iden, f.value)).unzip()
	}

	/// Alias to self.zip()
	pub fn for_update(self) -> Vec<(&'static str, Value)> {
		self.0.into_iter().map(|f| (f.iden, f.value)).collect()
	}
}

/// SQL Convenient method
impl SqliteFields {
	/// will return a string like `"id", "name", "content"` for each member of the field
	pub fn sql_columns(&self) -> String {
		self.0.iter().map(|f| format!("\"{}\"", f.iden)).collect::<Vec<_>>().join(", ")
	}

	/// Will return a string like `?, ?, ?` for each member of the field
	pub fn sql_placeholders(&self) -> String {
		self.0.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
	}

	/// Will return `"id" = ?, "name" = ?, "content" = ?`
	pub fn sql_setters(&self) -> String {
		self.0
			.iter()
			.map(|f| format!("\"{}\" = ?", f.iden))
			.collect::<Vec<_>>()
			.join(", ")
	}

	pub fn into_values(self) -> Vec<Value> {
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
