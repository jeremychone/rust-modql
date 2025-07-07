use rusqlite::Result as RusqliteResult;
use rusqlite::types::{ToSql, ToSqlOutput, Value};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone)]
pub enum SqliteValue {
	RusqliteValue(Value),
	SerdeValue(JsonValue),
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

// region:    --- From Implementations

impl From<Value> for SqliteValue {
	fn from(value: Value) -> Self {
		SqliteValue::RusqliteValue(value)
	}
}

impl From<JsonValue> for SqliteValue {
	fn from(value: JsonValue) -> Self {
		SqliteValue::SerdeValue(value)
	}
}

// endregion: --- From Implementations

// region:    --- Support

fn json_value_to_rusqlite_value(json_value: &JsonValue) -> Value {
	let json_str = serde_json::to_string(json_value).unwrap_or_default();
	if json_str.is_empty() {
		Value::Null
	} else {
		Value::Text(json_str)
	}
}

// endregion: --- Support
