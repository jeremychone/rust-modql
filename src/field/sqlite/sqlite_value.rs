use rusqlite::Result as RusqliteResult;
use rusqlite::types::{ToSql, ToSqlOutput, Value as RusqliteValue};
use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Debug, Clone)]
pub enum SqliteValue {
	RusqliteValue(RusqliteValue),
	SerdeValue(JsonValue),
}

impl SqliteValue {
	/// NOTE: This will set the value to Null if fail to serialize
	/// TODO: Need to error!() trace when fail to serialize happen.
	pub fn from_serializable<T: Serialize>(value: T) -> Self {
		let value = serde_json::to_value(value).unwrap_or(JsonValue::Null);
		SqliteValue::SerdeValue(value)
	}
}

// region:    --- ToSql Implementation

impl ToSql for SqliteValue {
	fn to_sql(&self) -> RusqliteResult<ToSqlOutput<'_>> {
		match self {
			SqliteValue::RusqliteValue(value) => value.to_sql(),
			SqliteValue::SerdeValue(json_value) => {
				// Convert serde_json::Value to rusqlite::Value first (will be string )
				let rusqlite_value = json_value_to_rusqlite_value(json_value);
				Ok(ToSqlOutput::Owned(rusqlite_value))
			}
		}
	}
}

// endregion: --- ToSql Implementation

// region:    --- Froms for SqliteValue

impl From<RusqliteValue> for SqliteValue {
	fn from(value: RusqliteValue) -> Self {
		SqliteValue::RusqliteValue(value)
	}
}

impl From<JsonValue> for SqliteValue {
	fn from(value: JsonValue) -> Self {
		SqliteValue::SerdeValue(value)
	}
}

// endregion: --- Froms for SqliteValue

// region:    --- Froms for Rusqlite Value

impl From<SqliteValue> for RusqliteValue {
	fn from(value: SqliteValue) -> Self {
		match value {
			SqliteValue::RusqliteValue(value) => value,
			SqliteValue::SerdeValue(value) => json_value_to_rusqlite_value(&value),
		}
	}
}

// endregion: --- Froms for Rusqlite Value

// region:    --- Support

fn json_value_to_rusqlite_value(json_value: &JsonValue) -> RusqliteValue {
	let json_str = serde_json::to_string(json_value).unwrap_or_default();
	if json_str.is_empty() {
		RusqliteValue::Null
	} else {
		RusqliteValue::Text(json_str)
	}
}

// endregion: --- Support
