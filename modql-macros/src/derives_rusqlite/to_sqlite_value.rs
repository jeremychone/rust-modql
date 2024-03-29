// lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

pub fn derive_to_sqlite_value_inner(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	// Get the identifier of the enum (e.g., "Model")
	let name = input.ident;

	// Build the match arms
	let expanded = match input.data {
		Data::Enum(data) => process_enum(name, data),
		syn::Data::Struct(data) => process_struct(name, data),
		_ => panic!("ToSqliteValue can only be used with enums or simple tuple struct for now"),
	};

	// Return the generated token stream
	TokenStream::from(expanded)
}

/// For a type annotated like:
/// ```rust,notest
/// #[derive(modql::ToSqliteValue)]
/// struct SId(i64);
/// ```
/// Will generate something like:
/// ```rust,notest
/// impl rusqlite::types::ToSql for SId {
///   fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
///     Ok(rusqlite::types::ToSqlOutput::Owned(self.0.into()))
///   }
/// }
/// ```
fn process_struct(name: Ident, data: DataStruct) -> proc_macro2::TokenStream {
	let first_tuple_field = match data.fields {
		Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields.unnamed.into_iter().next().unwrap(),
		_ => panic!("Expected a tuple struct with one field"),
	};

	let field_type = &first_tuple_field.ty;
	let field_ident = &first_tuple_field.ident;

	#[rustfmt::skip]
	let expanded = quote! {
	  impl rusqlite::types::ToSql for #name {
      fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(self.0.into()))
      }
	  }
	};

	expanded
}

/// For an enum type annotated like:
/// ```rust,notest
/// #[derive(ToSqliteValue)]
/// pub enum DItemKind {
///   Md,
///   Pdf,
///   Unknown,
/// }
/// ```
/// Will expand to something like:
/// ```rust,notest
/// impl ToSql for DItemKind {
///   fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
///     let val = match self {
///       DItemKind::Md => "Md",
///       DItemKind::Pdf => "Pdf",
///       DItemKind::Unknown => "Unknown",
///     }
///     .to_string();
///
///     Ok(rusqlite::types::ToSqlOutput::Owned(val.into()))
///   }
/// }
/// ```

fn process_enum(name: Ident, data: DataEnum) -> proc_macro2::TokenStream {
	let arms = data
		.variants
		.iter()
		.map(|variant| {
			let variant_ident = &variant.ident;
			let variant_name_str = variant_ident.to_string();
			quote! {
			  #name::#variant_ident => #variant_name_str,
			}
		})
		.collect::<Vec<_>>();

	// Generate the final token stream
	#[rustfmt::skip]
	let expanded = quote! {

	  impl rusqlite::types::ToSql for #name {
      fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        let val = match self {
        #(#arms)*
        }.to_string();

        Ok(rusqlite::types::ToSqlOutput::Owned(val.into()))
      }
	  }
	};

	expanded
}
