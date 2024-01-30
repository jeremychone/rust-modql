mod utils;

use crate::utils::modql_field::ModqlFieldProp;
use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use utils::get_struct_modql_prop;

pub(crate) fn derive_fields_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let fields = get_struct_fields(&ast);

	let struct_name = &ast.ident;

	// -- Collect Elements
	let props = modql_field::get_modql_field_props(fields);

	let props_all_names: Vec<&String> = props.iter().map(|p| &p.name).collect();

	// Will be "" if none
	let struct_modql_prop = get_struct_modql_prop(&ast).unwrap();
	let props_all_tables: Vec<String> = props
		.iter()
		.map(|p| {
			p.table
				.as_ref()
				.map(|t| t.to_string())
				.unwrap_or_else(|| struct_modql_prop.table.as_ref().map(|s| s.to_string()).unwrap_or_default())
		})
		.collect();

	let props_all_columns: Vec<String> = props
		.iter()
		.map(|p| p.column.as_ref().map(|c| c.to_string()).unwrap_or_else(|| p.name.to_string()))
		.collect();

	fn field_options_quote(mfield_prop: &ModqlFieldProp) -> proc_macro2::TokenStream {
		if let Some(cast_as) = &mfield_prop.cast_as {
			quote! { modql::field::FieldOptions { cast_as: Some(#cast_as.to_string()) } }
		} else {
			quote! { modql::field::FieldOptions { cast_as: None } }
		}
	}

	// -- all_fields() quotes!
	let all_fields_quotes = props.iter().map(|p| {
		let name = &p.name;
		let field_options = field_options_quote(p);
		let ident = p.ident;

		quote! {
			ff.push(
				modql::field::Field::new_with_options(modql::SIden(#name), self.#ident.into(), #field_options)
			);
		}
	});

	// -- The not_none_fields quotes!
	let not_none_fields_quotes = props.iter().map(|p| {
		let name = &p.name;
		let field_options = field_options_quote(p);
		let ident = p.ident;

		if p.is_option {
			quote! {
					if let Some(val) = self.#ident {
						ff.push(
							modql::field::Field::new_with_options(modql::SIden(#name), val.into(), #field_options)
						);
					}
			}
		} else {
			quote! {
					ff.push(
						modql::field::Field::new_with_options(modql::SIden(#name), self.#ident.into(), #field_options)
					);
			}
		}
	});

	// -- Compose the final code
	let output = quote! {
		impl modql::field::HasFields for #struct_name {

			fn not_none_fields(self) -> modql::field::Fields {
				let mut ff: Vec<modql::field::Field> = Vec::new();
				#(#not_none_fields_quotes)*
				modql::field::Fields::new(ff)
			}

			fn all_fields(self) -> modql::field::Fields {
				let mut ff: Vec<modql::field::Field> = Vec::new();
				#(#all_fields_quotes)*
				modql::field::Fields::new(ff)
			}

			fn field_names() -> &'static [&'static str] {
				&[#(
				#props_all_names,
				)*]
			}

			fn field_idens() -> Vec<sea_query::SeaRc<dyn sea_query::Iden>> {
				vec![#(
				sea_query::IntoIden::into_iden(modql::SIden(#props_all_names)),
				)*]
			}

			fn field_column_refs() -> Vec<sea_query::ColumnRef> {
				use sea_query::IntoIden;
				use sea_query::ColumnRef;
				use modql::SIden;

				let mut v = Vec::new();

				// NOTE: There's likely a more elegant solution, but this approach is semantically correct.
				#(
					let col_ref = if #props_all_tables == "" {
						ColumnRef::Column(SIden(#props_all_columns).into_iden())
					} else {
						ColumnRef::TableColumn(
							SIden(#props_all_tables).into_iden(),
							SIden(#props_all_columns).into_iden())
					};
					v.push(col_ref);
				)*
				v
			}
		}
	};

	output.into()
}
