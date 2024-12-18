mod support;

use crate::support::rusqlite_utils::{create_schema, seed_data};
use crate::support::Result;
use modql::field::HasFields;
use modql::{SIden, SqliteFromRow};
use modql_macros::Fields;
use pretty_sqlite::pretty_table;
use rusqlite::Connection;
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

// cargo run --example c02-rusqlite-sea-query-select --all-features

fn main() -> Result<()> {
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_schema(&conn)?;
	seed_data(&conn)?;

	// let content = pretty_table(&conn, "project")?;
	// println!("Project table:\n{content}");

	// let content = pretty_table(&conn, "task")?;
	// println!("Task table:\n{content}");

	let mut query = Query::select();
	query.from(SIden("task"));
	let metas = Task::field_metas();
	for &meta in metas.iter() {
		meta.sea_apply_select_column(&mut query);
	}

	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	println!("Sql: {sql}\n");

	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then(&*values.as_params(), Task::sqlite_from_row)?;
	let tasks = iter.collect::<core::result::Result<Vec<Task>, _>>()?;
	for task in tasks {
		println!("Task: {task:?}");
	}

	Ok(())
}

#[derive(Debug, Fields, SqliteFromRow)]
struct Project {
	id: i64,
	name: String,
}

#[derive(Debug, Fields, SqliteFromRow)]
struct Task {
	title: String,
	desc: String,
	project_id: i64,
}
