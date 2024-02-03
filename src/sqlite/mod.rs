//! Requires the `with-rusqlite` and `with-sea-query` features
//! and provides a very basic `sqlite::FromRow` based on the `Fields` derivation.
//!

// #[deprecated(note = "Use FromSqliteRow")]
// pub use modql_macros::FromSqliteRow as FromRow;

pub use modql_macros::FromSqliteRow;
pub use modql_macros::FromSqliteValue;

pub trait FromSqliteRow
where
	Self: Sized,
{
	fn from_rusqlite_row<'r>(val: &'r rusqlite::Row<'r>) -> rusqlite::Result<Self>;
}
