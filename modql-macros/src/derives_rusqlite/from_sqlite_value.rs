// lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

pub fn derive_from_sqlite_value_inner(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	// Get the identifier of the enum (e.g., "Model")
	let name = input.ident;

	// Build the match arms
	let expanded = match input.data {
		Data::Enum(data) => process_enum(name, data),
		syn::Data::Struct(data) => process_struct(name, data),
		_ => panic!("FromSqliteValue can only be used with enums or simple tuple struct for now (see FromRow)"),
	};

	// Return the generated token stream
	TokenStream::from(expanded)
}

fn process_struct(name: Ident, data: DataStruct) -> proc_macro2::TokenStream {
	let first_tuple_field = match data.fields {
		Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields.unnamed.into_iter().next().unwrap(),
		_ => panic!("Expected a tuple struct with one field"),
	};

	let field_type = &first_tuple_field.ty;

	let expanded = quote! {
		impl rusqlite::types::FromSql for #name {
			fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
				let val = #field_type::column_result(value)?;
				Ok(#name(val))
			}
		}
	};

	expanded
}

fn process_enum(name: Ident, data: DataEnum) -> proc_macro2::TokenStream {
	let arms = data
		.variants
		.iter()
		.map(|variant| {
			let variant_name = &variant.ident;
			let variant_name_str = variant_name.to_string();
			quote! {
				#variant_name_str => Ok(#name::#variant_name),
			}
		})
		.collect::<Vec<_>>();

	// Generate the final token stream
	let expanded = quote! {
		impl rusqlite::types::FromSql for #name {
			fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
				let txt: String = rusqlite::types::FromSql::column_result(value)?;
				match txt.as_str() {
					#(#arms)*
					_ => Err(rusqlite::types::FromSqlError::Other(
						format!("Invalid enum variant string '{}'", txt).into(),
					)),
				}
			}
		}
	};

	expanded
}
