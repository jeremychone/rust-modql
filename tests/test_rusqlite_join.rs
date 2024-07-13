#![cfg(feature = "with-rusqlite")]
#![allow(clippy::redundant_closure)]

mod support;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use crate::support::sqlite::{seed_agent, seed_module};
use modql::field::{Fields, HasFields};
use modql::SqliteFromRow;
use rusqlite::Connection;
use std::result;

#[derive(Debug, Clone, Fields, SqliteFromRow)]
#[modql(rel = "agent")]
pub struct Agent {
	id: i64,
	name: Option<String>,
	model: Option<String>,
	#[field(name = "lvl")]
	level: Option<u32>,
	module_id: Option<i64>,
	#[field(rel = "module", name = "name")]
	module_name: Option<String>,
}

#[test]
fn test_sqlite_select_join_full() -> Result<()> {
	// -- Setup & Fixtures
	let conn = Connection::open_in_memory()?;
	support::sqlite::create_test_schema(&conn)?;
	let fx_module_name = "test-module-A";
	let fx_agent_name = "test-agent-01";
	let module_id = seed_module(&conn, fx_module_name)?;
	let _agent_id = seed_agent(&conn, fx_agent_name, Some(module_id))?;

	// -- Build the Sql
	let cols = Agent::field_metas().sql_col_refs();
	let sql = format!(
		r#"
SELECT {cols} FROM agent
LEFT JOIN module ON agent.module_id = module.id;	
	"#
	);
	println!("->> {sql}");

	// -- Excute Query
	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then([], |r| Agent::sqlite_from_row(r))?;
	let agents: Vec<Agent> = iter.collect::<result::Result<_, _>>()?;
	let agent = agents.first().ok_or("Should have one agent")?;

	// -- Check result
	assert_eq!(agent.name.as_deref(), Some(fx_agent_name));
	assert_eq!(agent.level, Some(123));
	assert_eq!(agent.module_name.as_deref(), Some(fx_module_name));

	Ok(())
}

#[test]
fn test_sqlite_select_join_partial() -> Result<()> {
	// -- Setup & Fixtures
	let conn = Connection::open_in_memory()?;
	support::sqlite::create_test_schema(&conn)?;
	let fx_module_name = "test-module-A";
	let fx_agent_name = "test-agent-01";
	let module_id = seed_module(&conn, fx_module_name)?;
	let _agent_id = seed_agent(&conn, fx_agent_name, Some(module_id))?;

	let prop_names = &["id", "name", "module_id", "module_name"];

	// -- Build the Sql
	let cols = Agent::field_metas().sql_col_refs_for(prop_names);
	let sql = format!(
		r#"
SELECT {cols} FROM agent
LEFT JOIN module ON agent.module_id = module.id;	
	"#
	);
	println!("->> {sql}");

	// -- Excute Query
	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then([], |r| Agent::sqlite_from_row_partial(r, prop_names))?;
	let agents: Vec<Agent> = iter.collect::<result::Result<_, _>>()?;
	let agent = agents.first().ok_or("Should have one agent")?;

	// -- Check result
	assert_eq!(agent.name.as_deref(), Some(fx_agent_name));
	assert_eq!(agent.level, None); // because not requested
	assert_eq!(agent.model, None); // because not requested
	assert_eq!(agent.module_name.as_deref(), Some(fx_module_name));

	Ok(())
}
