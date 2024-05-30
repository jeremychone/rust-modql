#![cfg(feature = "with-rusqlite")]
#![allow(unused)]

mod support;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use modql::field::Fields;
use modql::{SqliteFromValue, SqliteToValue};
use rusqlite::Connection;

/// Simple enum with From/To SqliteValue
#[derive(SqliteFromValue, SqliteToValue)]
pub enum DItemKind {
	Md,
	Pdf,
	Unknown,
}

/// Simple tuple struct with From/To SqliteValue
#[derive(SqliteFromValue, SqliteToValue)]
pub struct SimpleId(i64);

#[derive(Debug, Clone, Fields)]
pub struct Agent {
	pub id: i64,
	pub name: Option<String>,
	pub level: Option<u32>,
	pub module_id: Option<i64>,
}

#[test]
fn test_rust_sqlite_derives() -> Result<()> {
	// -- Setup & Fixtures
	let kind = DItemKind::Md;
	let sid = SimpleId(123);

	// For now just making sure it compiles above.
	// Later can do the exec in sqlite and check

	Ok(())
}
