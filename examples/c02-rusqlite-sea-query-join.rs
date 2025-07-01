mod support;

use crate::support::rusqlite_utils::{create_schema, seed_data};
use crate::support::Result;
use modql::field::HasFields;
use modql::RusqliteBinder;
use modql::{SIden, SqliteFromRow};
use modql_macros::Fields;
use pretty_sqlite::pretty_table;
use rusqlite::Connection;
use sea_query::{Expr, ExprTrait, IntoColumnRef, JoinType, Query, SqliteQueryBuilder};

// cargo run --example c02-rusqlite-sea-query-join --all-features

fn main() -> Result<()> {
	let conn = Connection::open_in_memory()?; // for file: Connection::open(path)?
	create_schema(&conn)?;
	seed_data(&conn)?;

	// let content = pretty_table(&conn, "project")?;
	// println!("Project table:\n{content}");

	// let content = pretty_table(&conn, "task")?;
	// println!("Task table:\n{content}");

	let mut query = Query::select();
	let task_iden = SIden("task");
	let project_iden = SIden("project");
	query.from(task_iden).join(
		JoinType::LeftJoin,
		project_iden,
		Expr::col((task_iden, SIden("project_id")).into_column_ref())
			.equals((project_iden, SIden("id")).into_column_ref()),
	);
	let metas = Task::field_metas();
	for &meta in metas.iter() {
		meta.sea_apply_select_column(&mut query);
	}

	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	println!("SQL: {sql}\n");

	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then(&*values.as_params(), Task::sqlite_from_row)?;
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
