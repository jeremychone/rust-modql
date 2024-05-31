use crate::field::{HasFields, SeaFields};
use sea_query::{ColumnRef, DynIden, IntoIden, SelectStatement};

pub trait HasSeaFields: HasFields {
	/// Returns the `Fields` containing the `Field` items that have non-`None` values.
	fn not_none_sea_fields(self) -> SeaFields;

	/// Returns the `Fields` containing all of the `Field`.
	fn all_sea_fields(self) -> SeaFields;

	/// Return the sea_query::DynIden for each field (just matching the field name)
	fn sea_idens() -> Vec<DynIden>;

	/// Returns the list of column refs (takes the eventual #[field(rel = "table_name")])
	/// WARNING: This won't have the aliases if there need to be some.
	///          Use `sea_apply_select_columns(select)` or `T::field_metas()` to build manually.
	/// TODO: Need to use the `field_metas().. meta.sea_column_ref()``
	fn sea_column_refs() -> Vec<ColumnRef>;

	/// Returns the list of column refs with the given relation (e.g., table name) and IntoIden (.e.g., StringIden or SIden)
	fn sea_column_refs_with_rel(rel: impl IntoIden) -> Vec<ColumnRef>;

	fn sea_apply_select_columns(&self, sea_select: &mut SelectStatement) {
		for meta in Self::field_metas().iter() {
			meta.sea_apply_select_column(sea_select);
		}
	}
}
