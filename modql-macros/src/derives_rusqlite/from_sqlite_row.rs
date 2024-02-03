use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// FromSqliteRow (aliased to FromRow)
pub fn derive_from_sqlite_row_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let fields = get_struct_fields(&ast);

	let struct_name = &ast.ident;

	let props = modql_field::get_modql_field_props(fields);

	let getters = props.iter().map(|p| {
		let name = &p.name;
		let ident = p.ident;

		quote! {
			#ident: val.get(#name)?,
		}
	});

	// -- Compose the final code
	let output = quote! {
		impl modql::FromSqliteRow for #struct_name {
			fn from_rusqlite_row<'r>(val: &'r rusqlite::Row<'r>) -> rusqlite::Result<Self> {
				let entity = Self {
					#(#getters)*
				};

				Ok(entity)
			}
		}
	};

	output.into()
}

// TextEnum
