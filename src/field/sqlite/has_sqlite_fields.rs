use crate::field::{HasFields, SqliteColumnRef, SqliteFields};

pub trait HasSqliteFields: HasFields {
	/// Returns the `Fields` containing the `Field` items that have non-`None` values.
	fn sqlite_not_none_fields(self) -> SqliteFields;

	/// Returns the `Fields` containing all of the `Field`.
	fn sqlite_all_fields(self) -> SqliteFields;

	// /// Returns the list of column refs (takes the eventual #[field(rel = "table_name")])
	// /// WARNING: This won't have the aliases if there need to be some.
	// ///          Use `lite_apply_select_columns(select)` or `T::field_metas()` to build manually.
	// /// TODO: Need to use the `field_metas().. meta.sea_column_ref()``
	// fn lite_column_refs() -> Vec<&'static str>;

	/// Returns the list of column refs with the given relation (e.g., table name) and IntoIden (.e.g., StringIden or SIden)
	fn sqlite_column_refs_with_rel(rel: &str) -> Vec<SqliteColumnRef>;
}
