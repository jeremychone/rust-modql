#![cfg(feature = "with-rusqlite")]
#![allow(clippy::redundant_closure)]

mod support;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use crate::support::sqlite::{seed_agent, seed_module};
use modql::field::{Fields, HasFields};
use modql::{SqliteFromRow, SqliteFromValue, SqliteToValue};
use rusqlite::types::Value;
use rusqlite::Connection;
use std::result;

#[derive(Debug, Clone, SqliteFromValue, SqliteToValue)]
pub struct Id(i64);

impl Id {
	pub fn as_i64(&self) -> i64 {
		self.0
	}
}

// from &i64
impl From<&i64> for Id {
	fn from(val: &i64) -> Id {
		Id(*val)
	}
}

#[derive(Debug, Clone, Fields, SqliteFromRow)]
pub struct Agent {
	id: Id,
	name: Option<String>,
	model: Option<String>,
	#[field(name = "lvl")]
	level: Option<u32>,
	module_id: Option<i64>,
	// #[field(rel = "module", name = "name")]
	// module_name: Option<String>,
}

#[test]
fn test_sqlite_select_simple() -> Result<()> {
	// -- Setup & Fixtures
	let conn = Connection::open_in_memory()?;
	support::sqlite::create_test_schema(&conn)?;
	let module_id = seed_module(&conn, "test-module-A")?;
	let _agent_id = seed_agent(&conn, "test-agent-01", Some(module_id))?;

	// -- Build the Sql
	let metas = Agent::field_metas();
	let cols = metas.iter().map(|meta| meta.sql_col_ref()).collect::<Vec<_>>();
	let cols = cols.join(", ");
	let sql = format!("SELECT {cols} FROM agent");

	// -- Excute Query
	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then([], |r| Agent::sqlite_from_row(r))?;
	let agents: Vec<Agent> = iter.collect::<result::Result<_, _>>()?;
	let agent = agents.first().ok_or("Should have one agent")?;

	// -- Check result
	assert_eq!(agent.name.as_deref(), Some("test-agent-01"));
	assert_eq!(agent.level, Some(123));

	Ok(())
}

#[test]
fn test_sqlite_select_partial() -> Result<()> {
	// -- Setup & Fixtures
	let conn = Connection::open_in_memory()?;
	support::sqlite::create_test_schema(&conn)?;
	let module_id = seed_module(&conn, "test-module-A")?;
	let _agent_id = seed_agent(&conn, "test-agent-01", Some(module_id))?;

	let only_props = &["id", "name"];

	// -- Build the Sql
	// note: Here we could use filed_metas().sql_col_refs_for(prop_names)
	let metas = Agent::field_metas();
	let cols = metas
		.iter()
		.filter(|m| only_props.contains(&m.prop_name))
		.map(|meta| meta.sql_col_ref())
		.collect::<Vec<_>>();
	let cols = cols.join(", ");
	let sql = format!("SELECT {cols} FROM agent");

	// -- Excute Query
	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then([], |r| Agent::sqlite_from_row_partial(r, &["id", "name"]))?;
	let agents: Vec<Agent> = iter.collect::<result::Result<_, _>>()?;
	let agent = agents.first().ok_or("Should have one agent")?;

	// -- Check result
	assert_eq!(agent.name.as_deref(), Some("test-agent-01"));
	assert_eq!(agent.level, None); // because we did not get it

	Ok(())
}
