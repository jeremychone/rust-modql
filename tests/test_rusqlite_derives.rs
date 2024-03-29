#![cfg(feature = "with-rusqlite")]
#![allow(unused)]

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

use modql::{FromSqliteValue, ToSqliteValue};

/// Simple enum with From/To SqliteValue
#[derive(FromSqliteValue, ToSqliteValue)]
pub enum DItemKind {
	Md,
	Pdf,
	Unknown,
}

/// Simple tuple struct with From/To SqliteValue
#[derive(FromSqliteValue, ToSqliteValue)]
pub struct SimpleId(i64);

#[test]
fn test_rust_sqlite_derives() -> Result<()> {
	// -- Setup & Fixtures
	let kind = DItemKind::Md;
	let sid = SimpleId(123);

	// For now just making sure it compiles above.
	// Later can do the exec in sqlite and check

	Ok(())
}
