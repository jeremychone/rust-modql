use crate::field::Fields;
use sea_query::{ColumnRef, DynIden, IntoIden};

// region:    --- HasFields
pub trait HasFields {
	/// Returns the `Fields` containing the `Field` items that have non-`None` values.
	fn not_none_fields(self) -> Fields;

	/// Returns the `Fields` containing all of the `Field`.
	fn all_fields(self) -> Fields;

	/// Returns the array of all field names this struct has.
	fn field_names() -> &'static [&'static str];

	/// Return the sea_query::DynIden for each field (just matching the field name)
	fn field_idens() -> Vec<DynIden>;

	/// Returns the list of column refs (takes the eventual #[modql(table = "table_name")])
	fn field_column_refs() -> Vec<ColumnRef>;

	/// Returns the list of column refs with the given relation (e.g., table name) and IntoIden (.e.g., StringIden or SIden)
	fn field_column_refs_with_rel(rel: impl IntoIden) -> Vec<ColumnRef>;
}
// endregion: --- HasFields
