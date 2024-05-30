use crate::field::SeaFields;
use sea_query::{ColumnRef, DynIden, IntoIden};

pub trait HasSeaFields {
	/// Returns the `Fields` containing the `Field` items that have non-`None` values.
	fn not_none_sea_fields(self) -> SeaFields;

	/// Returns the `Fields` containing all of the `Field`.
	fn all_sea_fields(self) -> SeaFields;

	/// Return the sea_query::DynIden for each field (just matching the field name)
	fn sea_idens() -> Vec<DynIden>;

	/// Returns the list of column refs (takes the eventual #[field(rel = "table_name")])
	fn sea_column_refs() -> Vec<ColumnRef>;

	/// Returns the list of column refs with the given relation (e.g., table name) and IntoIden (.e.g., StringIden or SIden)
	fn sea_column_refs_with_rel(rel: impl IntoIden) -> Vec<ColumnRef>;
}
