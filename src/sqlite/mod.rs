//! Requires the `with-rusqlite` and `with-sea-query` features
//! and provides a very basic `sqlite::FromRow` based on the `Fields` derivation.
//!

// region:    --- Section

pub use modql_macros::FromSqliteRow as FromRow;

// endregion: --- Section

pub trait FromRow
where
	Self: Sized,
{
	fn from_rusqlite_row<'r>(val: &'r rusqlite::Row<'r>) -> rusqlite::Result<Self>;
}
