use super::Result;
use modql::SqliteFromRow;
use rusqlite::{Connection, Params};

pub fn create_test_schema(conn: &Connection) -> Result<()> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS agent (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT,
						model   TEXT,
						lvl     INTEGER,
						module_id INTEGER,
						data_t  TEXT,
            data_b  BLOB
        ) STRICT",
		(), // empty list of parameters.
	)?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS module (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT
        ) STRICT",
		(), // empty list of parameters.
	)?;

	Ok(())
}

// region:    --- Seeders

pub fn seed_agent(conn: &Connection, name: &str, model_id: Option<i64>) -> Result<i64> {
	let id = insert_with_returnning_id(
		conn,
		"INSERT INTO agent (name, lvl, module_id) VALUES (?, ?, ?) RETURNING id",
		(name, &123, &model_id),
	)?;

	Ok(id)
}

pub fn seed_module(conn: &Connection, name: &str) -> Result<i64> {
	let id = insert_with_returnning_id(conn, "INSERT INTO module (name) VALUES (?) RETURNING id", [name])?;

	Ok(id)
}

// endregion: --- Seeders

// region:    --- Query Helpers

pub fn insert_with_returnning_id<P: Params>(conn: &Connection, insert_sql: &str, value_params: P) -> Result<i64> {
	let mut stmt = conn.prepare(insert_sql)?;
	let id = stmt.query_row(value_params, |r| r.get::<_, i64>(0))?;

	Ok(id)
}

#[allow(unused)]
pub fn exec_select<T: SqliteFromRow>(conn: &Connection, sql: &str) -> Result<Vec<T>> {
	let mut stmt = conn.prepare(sql)?;
	let iter = stmt.query_and_then([], |r| T::sqlite_from_row(r))?;
	let items = iter.collect::<core::result::Result<Vec<T>, _>>()?;

	Ok(items)
}

// endregion: --- Query Helpers
