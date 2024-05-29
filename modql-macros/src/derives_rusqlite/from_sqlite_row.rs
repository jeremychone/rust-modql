use crate::utils::modql_field::ModqlFieldsAndSkips;
use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// FromSqliteRow (aliased to FromRow)
pub fn derive_from_sqlite_row_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let fields = get_struct_fields(&ast);

	let struct_name = &ast.ident;

	let ModqlFieldsAndSkips {
		modql_fields,
		skipped_fields,
	} = modql_field::get_modql_field_props_and_skips(fields);

	let getters = modql_fields.iter().map(|p| {
		let name = &p.name;
		let ident = p.ident;

		quote! {
			#ident: val.get(#name)?,
		}
	});

	// for skipped
	let skipped_fields = skipped_fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! {
			#ident: Default::default(),
		}
	});

	// -- Compose the final code
	let output = quote! {
		impl modql::FromSqliteRow for #struct_name {
			fn from_sqlite_row(val: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
				let entity = Self {
					#(#getters)*
					#(#skipped_fields)*
				};

				Ok(entity)
			}
		}
	};

	output.into()
}

// TextEnum
