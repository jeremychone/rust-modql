// lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

pub fn sqlite_from_sqlite_value_inner(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	// Get the identifier of the enum (e.g., "Model")
	let name = input.ident;

	// Build the match arms
	let expanded = match input.data {
		Data::Enum(data) => process_enum(name, data),
		_ => panic!("FromSqliteValue can only be used with enums for now (see FromRow)"),
	};

	// Return the generated token stream
	TokenStream::from(expanded)
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
