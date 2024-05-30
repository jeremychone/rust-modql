//! Requires the `with-rusqlite` and `with-sea-query` features
//! and provides a very basic `sqlite::FromRow` based on the `Fields` derivation.
//!

pub use modql_macros::SqliteFromRow;
pub use modql_macros::SqliteFromValue;
pub use modql_macros::SqliteToValue;

// -- deprecated
pub use modql_macros::FromSqliteRow;
pub use modql_macros::FromSqliteValue;
pub use modql_macros::ToSqliteValue;

#[deprecated(note = "use SqliteFromRow")]
pub trait FromSqliteRow: SqliteFromRow
where
	Self: Sized,
{
}

pub trait SqliteFromRow
where
	Self: Sized,
{
	#[deprecated(note = "use sqlite_from_row")]
	fn from_sqlite_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
		Self::sqlite_from_row(row)
	}

	fn sqlite_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self>;

	fn sqlite_from_row_partial(row: &rusqlite::Row<'_>, prop_names: &[&str]) -> rusqlite::Result<Self>;
}

struct Stuff {
	id: i32,
	name: Option<String>,
	desc: Option<String>,
}
