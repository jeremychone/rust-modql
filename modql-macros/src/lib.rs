// region:    --- Modules

mod derives_field;
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

#[proc_macro_derive(Fields, attributes(field, modql))]
pub fn derive_fields(input: TokenStream) -> TokenStream {
	derives_field::derive_fields_inner(input)
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
/// Will generate something like
/// ```rust,norun
/// impl From<EpochTime> for sea_query::Value {
///   fn from(value: EpochTime) -> Self {
///     Self::BigInt(Some(value.0))
///   }
/// }
/// impl sea_query::Nullable for EpochTime {
///   fn null() -> sea_query::Value {
///     sea_query::Value::BigInt(None)
///   }
/// }
/// ```
/// Notes:
///   - Supports only primitive types (no array yet)
///   - Supports only one tuple field.
///
/// - On Simple enum (plain variant only).
/// ```rust,norun
/// #[derive(modql::field::SeaFieldValue)]
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
#[proc_macro_derive(SeaFieldValue)]
pub fn derive_field_sea_value(input: TokenStream) -> TokenStream {
	derives_field::derive_field_sea_value_inner(input)
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

/// Will implement the `rusqlite::types::FromSql` for the annotated type.
///
/// For example:
///
/// - For simple enum (with variant name only)
/// ```rust,norun
/// pub enum Kind {
///   Md,
///   Pdf,
///   Unknown,
/// }
/// ```
/// Will generate something like:
/// ```rust,norun
///  impl rusqlite::types::FromSql for Kind {
///    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
///      let txt: String = rusqlite::types::FromSql::column_result(value)?;
///      match txt.as_str() {
///        "Md"      => Ok(Kind::Md),
///        "Pdf"     => Ok(Kind::Pdf),
///        "Unknown" => Ok(Kind::Unknown),
///        _ => Err(rusqlite::types::FromSqlError::Other(
///        format!("Invalid enum variant string '{}'", txt).into(),
///        )),
///      }
///    }
///  }
/// ```
///
/// - For simple tuple struct (one value that already implement the FromSqlt)
/// ```rust,norun
/// #[derive(modql::FromSqliteType)]
/// pub struct EpochTime(i64);
/// ```
/// Will generate something like:
/// ```rust,norun
/// impl rusqlite::types::FromSql for EpochTime {
///   fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
///     let val = i64::column_result(value)?;
///     Ok(EpochTime(val))
///   }
/// }
/// ````
///
#[cfg(feature = "with-rusqlite")]
#[proc_macro_derive(FromSqliteValue)]
pub fn derive_from_sqlite_value(input: TokenStream) -> TokenStream {
	derives_rusqlite::derive_from_sqlite_value_inner(input)
}

#[cfg(feature = "with-rusqlite")]
#[proc_macro_derive(ToSqliteValue)]
pub fn derive_to_sqlite_value(input: TokenStream) -> TokenStream {
	derives_rusqlite::derive_to_sqlite_value_inner(input)
}

// endregion: --- with-rusqlite
