use crate::utils::modql_field::{ModqlFieldProp, ModqlFieldsAndSkips};
use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

// FromSqliteRow (aliased to FromRow)
pub fn derive_sqlite_from_row_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);

	// -- Prep the fields and mfields (modql fields)
	let fields = get_struct_fields(&ast);
	let struct_name = &ast.ident;

	let ModqlFieldsAndSkips {
		modql_fields,
		skipped_fields,
	} = modql_field::get_modql_field_props_and_skips(fields);
	let mfields_slice: Vec<&ModqlFieldProp> = modql_fields.iter().collect();

	let fn_sqlite_from_row_quote = impl_fn_sqlite_from_row(&mfields_slice, &skipped_fields);

	let fn_sqlite_from_row_partial_quote = impl_fn_sqlite_from_row_partial(&mfields_slice, &skipped_fields);

	// -- Compose the final code
	let output = quote! {
		impl modql::SqliteFromRow for #struct_name {

			#fn_sqlite_from_row_quote

			#fn_sqlite_from_row_partial_quote
		}
	};

	output.into()
}

fn impl_fn_sqlite_from_row(mfield_props: &[&ModqlFieldProp], skipped_fields: &[&Field]) -> proc_macro2::TokenStream {
	let getters_quotes = mfield_props.iter().map(|mf| {
		let ident = mf.ident;

		// NOTE: Here we assume the select column has been aliased to the name of the property
		let col_name = &mf.prop_name;

		quote! {
			#ident: val.get(#col_name)?,
		}
	});

	// for skipped
	let skipped_fields_quotes = skipped_fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! {
			#ident: Default::default(),
		}
	});

	let output = quote! {
			fn sqlite_from_row(val: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {

				let entity = Self {
					#(#getters_quotes)*
					#(#skipped_fields_quotes)*
				};

				Ok(entity)
			}
	};

	output
}

fn impl_fn_sqlite_from_row_partial(
	mfield_props: &[&ModqlFieldProp],
	skipped_fields: &[&Field],
) -> proc_macro2::TokenStream {
	let getters_quotes = mfield_props.iter().map(|mf| {
		let ident = mf.ident;

		// NOTE: Here we assume the select column has been aliased to the name of the property
		let col_name = &mf.prop_name;
		let is_option = mf.is_option;

		if mf.is_option {
			quote! {
				#ident: if prop_names.contains(&#col_name) { val.get(#col_name)? } else { None },
			}
		}
		// Otherwise, it's required
		// (later we have something like `#[field(partial_absent_as_default)]`)
		else {
			quote! {
				#ident: val.get(#col_name)?,
			}
		}
	});

	// for skipped
	let skipped_fields_quotes = skipped_fields.iter().map(|field| {
		let ident = field.ident.as_ref().unwrap();
		quote! {
			#ident: Default::default(),
		}
	});

	let output = quote! {

			fn sqlite_from_row_partial(val: &rusqlite::Row<'_>, prop_names: &[&str]) -> rusqlite::Result<Self> {
				let entity = Self {
					#(#getters_quotes)*
					#(#skipped_fields_quotes)*
				};

				Ok(entity)
			}

	};

	output
}
