#![cfg(all(feature = "with-rusqlite", feature = "with-sea-query"))]
#![allow(clippy::redundant_closure)]

mod support;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use crate::support::sqlite::{exec_select, insert_with_returnning_id, seed_agent, seed_module};
use modql::field::{Fields, HasFields, HasSeaFields};
use modql::{SIden, SqliteFromRow};
use rusqlite::Connection;
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

#[derive(Debug, Clone, Fields, SqliteFromRow)]
pub struct Agent {
	id: i64,
	name: Option<String>,
	model: Option<String>,
	#[field(name = "lvl")]
	level: Option<u32>,
	module_id: Option<i64>,
	// #[field(rel = "module", name = "name")]
	// module_name: Option<String>,
}

#[derive(Debug, Clone, Fields)]
pub struct AgentForCreate {
	name: Option<String>,
	model: Option<String>,
	#[field(name = "lvl")]
	level: Option<u32>,
	module_id: Option<i64>,
}

#[test]
fn test_sea_select() -> Result<()> {
	// -- Setup & Fixtures
	let conn = Connection::open_in_memory()?;
	support::sqlite::create_test_schema(&conn)?;
	let module_id = seed_module(&conn, "test-module-A")?;
	let _agent_id = seed_agent(&conn, "test-agent-01", Some(module_id))?;

	// -- Build sea select
	let mut query = Query::select();
	query.from(SIden("agent"));
	let metas = Agent::field_metas();
	for &meta in metas.iter() {
		meta.sea_apply_select_column(&mut query);
	}

	// -- Exec sea-query
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	let mut stmt = conn.prepare(&sql)?;
	let iter = stmt.query_and_then(&*values.as_params(), |r| Agent::sqlite_from_row(r))?;
	let agents = iter.collect::<core::result::Result<Vec<Agent>, _>>()?;

	// -- Check result
	let agent = agents.first().ok_or("Should have one agent")?;
	assert_eq!(agent.name.as_deref(), Some("test-agent-01"));
	assert_eq!(agent.level, Some(123));

	Ok(())
}

#[test]
fn test_sea_insert_and_raw_select() -> Result<()> {
	// -- Setup & Fixtures
	let conn = Connection::open_in_memory()?;
	support::sqlite::create_test_schema(&conn)?;
	let fx_agent_name = "test_insert_and_select AGENT";
	let fx_level = 234;

	// -- Insert
	let agent_c = AgentForCreate {
		name: Some(fx_agent_name.to_string()),
		model: None,
		level: Some(fx_level),
		module_id: None,
	};
	let fields = agent_c.not_none_sea_fields();
	let (columns, sea_values) = fields.for_sea_insert();
	let mut query = Query::insert();
	query.into_table(SIden("agent")).columns(columns).values(sea_values)?;
	query.returning_col(SIden("id"));
	let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
	let _agent_id = insert_with_returnning_id(&conn, &sql, &*values.as_params())?;

	// -- Build & execute raw select
	let metas = Agent::field_metas();
	let cols = metas.iter().map(|meta| meta.sql_col_ref()).collect::<Vec<_>>();
	let cols = cols.join(", ");
	let sql = format!("SELECT {cols} FROM agent");

	// -- Excute Query (without sea-query)
	let agents: Vec<Agent> = exec_select(&conn, &sql)?;
	let agent = agents.first().ok_or("Should have one agent")?;

	// -- Check result
	assert_eq!(agent.name.as_deref(), Some(fx_agent_name));
	assert_eq!(agent.level, Some(fx_level));

	Ok(())
}
