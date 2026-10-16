mod support;

use crate::support::Result;
use crate::support::rusqlite_utils::{create_schema, seed_data};
use modql::SqliteFromRow;
use modql::field::HasFields;
use modql_macros::Fields;
use pretty_sqlite::pretty_table;
use rusqlite::Connection;

// cargo run --example c01-rusqlite-join --features with-rusqlite

fn main() -> Result<()> {
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_schema(&conn)?;
	seed_data(&conn)?;

	// let content = pretty_table(&conn, "project")?;
	// println!("Project table:\n{content}");

	// let content = pretty_table(&conn, "task")?;
	// println!("Task table:\n{content}");

	let cols = Task::field_metas().sql_col_refs();
	let sql = format!(
		r#"
SELECT {cols} FROM task
LEFT JOIN project ON task.project_id = project.id;
	"#
	);
	println!("SQL: {sql}\n");

	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then([], Task::sqlite_from_row)?;
	let tasks = iter.collect::<core::result::Result<Vec<Task>, _>>()?;
	for task in tasks {
		println!("Task: {task:?}");
	}

	Ok(())
}

#[derive(Debug, Fields, SqliteFromRow)]
#[modql(rel = "project")]
struct Project {
	id: i64,
	name: String,
}

#[derive(Debug, Fields, SqliteFromRow)]
#[modql(rel = "task")]
struct Task {
	title: String,
	desc: String,
	project_id: i64,

	#[field(rel = "project", name = "name")]
	project_name: String,
}
