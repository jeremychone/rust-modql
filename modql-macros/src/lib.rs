// #![allow(unused)]
use crate::derive_filter_nodes::derive_filter_nodes_inner;
use proc_macro::TokenStream;

mod derive_filter_nodes;
mod utils;

#[cfg(feature = "with-sea-query")]
mod derive_fields;

#[cfg(feature = "with-rusqlite")]
mod derive_rusqlite;

#[proc_macro_derive(FilterNodes, attributes(modql))]
pub fn derive_filter_nodes(input: TokenStream) -> TokenStream {
	derive_filter_nodes_inner(input)
}

#[cfg(feature = "with-sea-query")]
#[proc_macro_derive(Fields, attributes(field, fields))]
pub fn derive_fields(input: TokenStream) -> TokenStream {
	derive_fields::derive_fields_inner(input)
}

#[cfg(feature = "with-rusqlite")]
#[proc_macro_derive(FromSqliteRow, attributes(field, fields))]
pub fn derive_from_sqlite_row(input: TokenStream) -> TokenStream {
	derive_rusqlite::derive_from_sqlite_row_inner(input)
}
