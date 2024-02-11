// region:    --- Modules

mod derives_filter;
mod utils;

use crate::derives_filter::derive_filter_nodes_inner;
use proc_macro::TokenStream;

// endregion: --- Modules

#[proc_macro_derive(FilterNodes, attributes(modql))]
pub fn derive_filter_nodes(input: TokenStream) -> TokenStream {
	derive_filter_nodes_inner(input)
}

// region:    --- with-seaquery

#[cfg(feature = "with-sea-query")]
mod derives_field;

#[cfg(feature = "with-sea-query")]
#[proc_macro_derive(Fields, attributes(field, fields))]
pub fn derive_fields(input: TokenStream) -> TokenStream {
	derives_field::derive_fields_inner(input)
}

#[cfg(feature = "with-sea-query")]
#[proc_macro_derive(FieldEnum, attributes(field, fields))]
#[deprecated(note = "Use derive(Field) instead")]
pub fn derive_field_enum(input: TokenStream) -> TokenStream {
	derives_field::derive_field_enum_inner(input)
}

/// Implements `From<T> for sea_query::Value` and `sea_query::Nullable for T`
/// where T is the struct or enum annotated with `#[derive(Field)]` for simple
/// tuple structs or enums.
///
/// For more complex types, implement both of these traits for the type.
///
/// For example:
///
/// - On simple type and single element tuple struct
/// ```rust,norun
/// #[derive(modql::field::Field)]
/// pub struct EpochTime(pub(in crate::time) i64);
/// ```
/// Notes:
///   - Supports only primitive types (no array yet)
///   - Supports only one tuple field.
///
/// - On Simple enum (plain variant only).
/// ```rust,norun
/// #[derive(modql::field::Field)]
/// pub enum Kind {
///   Md,
///   Pdf,
///   Unknown,
/// }
/// ```
/// Notes:
///   - Will be treated a sea_query::Value::String with the name of the variant.
///   - No rename for now.
#[cfg(feature = "with-sea-query")]
#[proc_macro_derive(Field, attributes(field, fields))]
pub fn derive_field(input: TokenStream) -> TokenStream {
	derives_field::derive_field_inner(input)
}

// endregion: --- with-seaquery

// region:    --- with-rusqlite

#[cfg(feature = "with-rusqlite")]
mod derives_rusqlite;

#[cfg(feature = "with-rusqlite")]
#[proc_macro_derive(FromSqliteRow, attributes(field, fields))]
pub fn derive_from_sqlite_row(input: TokenStream) -> TokenStream {
	derives_rusqlite::derive_from_sqlite_row_inner(input)
}

#[cfg(feature = "with-rusqlite")]
#[proc_macro_derive(FromSqliteValue, attributes(field, fields))]
pub fn derive_from_sqlite_value(input: TokenStream) -> TokenStream {
	derives_rusqlite::sqlite_from_sqlite_value_inner(input)
}

// endregion: --- with-rusqlite
