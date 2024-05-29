use crate::utils::modql_field::ModqlFieldsAndSkips;
use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// FromSqliteRow (aliased to FromRow)
pub fn derive_from_sqlite_row_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);

	// -- For below, in case we reactivates
	// let struct_modql_prop = get_struct_modql_attrs(&ast).unwrap();
	// let struct_rel = struct_modql_prop.rel;

	let fields = get_struct_fields(&ast);

	let struct_name = &ast.ident;

	let ModqlFieldsAndSkips {
		modql_fields,
		skipped_fields,
	} = modql_field::get_modql_field_props_and_skips(fields);

	let getters_quotes = modql_fields.iter().map(|mf| {
		let ident = mf.ident;

		// NOTE: Unfortunately, rusqlite or sqlite does not provide the rel table in the prepared statement,
		//       so we only have the name to match it. Therefore, we disable that for now.
		// let col_ref = if let Some(rel) = mf.rel.as_ref().or(struct_rel.as_ref()) {
		// 	format!("{rel}.{}", mf.name)
		// } else {
		// 	mf.name.to_string()
		// };

		let col_ref = &mf.name;

		quote! {
			#ident: val.get(#col_ref)?,
		}
	});

	// for skipped
	let skipped_fields_quotes = skipped_fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! {
			#ident: Default::default(),
		}
	});

	// -- Just for debug
	// let debug_print_quotes = modql_fields.iter().map(|mf| {
	// 	let col_ref = if let Some(rel) = mf.rel.as_ref().or(struct_rel.as_ref()) {
	// 		format!("\"{rel}\".\"{}\"", mf.name)
	// 	} else {
	// 		format!("\"{}\"", mf.name)
	// 	};
	// 	quote! {
	// 		println!("col_ref: {}", #col_ref);
	// 	}
	// });

	// -- Compose the final code
	let output = quote! {
		impl modql::FromSqliteRow for #struct_name {
			fn from_sqlite_row(val: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {

				let entity = Self {
					#(#getters_quotes)*
					#(#skipped_fields_quotes)*
				};

				Ok(entity)
			}
		}
	};

	output.into()
}

// TextEnum
