// region:    --- Modules

mod derives_filter;
mod utils;

use crate::derives_filter::derive_filter_nodes_inner;
use proc_macro::TokenStream;

// endregion: --- Modules

#[cfg(feature = "with-sea-query")]
mod derives_field;

#[cfg(feature = "with-rusqlite")]
mod derives_rusqlite;

#[proc_macro_derive(FilterNodes, attributes(modql))]
pub fn derive_filter_nodes(input: TokenStream) -> TokenStream {
	derive_filter_nodes_inner(input)
}

#[cfg(feature = "with-sea-query")]
#[proc_macro_derive(Fields, attributes(field, fields))]
pub fn derive_fields(input: TokenStream) -> TokenStream {
	derives_field::derive_fields_inner(input)
}

#[cfg(feature = "with-sea-query")]
#[proc_macro_derive(FieldEnum, attributes(field, fields))]
pub fn derive_field_enum(input: TokenStream) -> TokenStream {
	derives_field::derive_field_enum_inner(input)
}

#[cfg(feature = "with-rusqlite")]
#[proc_macro_derive(FromSqliteRow, attributes(field, fields))]
pub fn derive_from_sqlite_row(input: TokenStream) -> TokenStream {
	derives_rusqlite::derive_from_sqlite_row_inner(input)
}
