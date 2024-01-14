mod utils;

use crate::utils::{db_field, get_struct_fields};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};
use utils::get_fields_sqlb_prop;

pub(crate) fn derive_fields_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let fields = get_struct_fields(&ast);

	let sqb_attr = get_fields_sqlb_prop(&ast).unwrap();

	let struct_name = &ast.ident;

	// -- Collect Elements
	let props = db_field::get_field_db_props(fields);

	let props_all_idents: Vec<&Option<Ident>> = props.iter().map(|p| p.ident).collect();
	let props_all_names: Vec<&String> = props.iter().map(|p| &p.name).collect();

	// Will be "" if none
	let props_all_tables: Vec<String> = props
		.iter()
		.map(|p| {
			p.table
				.as_ref()
				.map(|t| t.to_string())
				.unwrap_or_else(|| sqb_attr.table.as_ref().map(|s| s.to_string()).unwrap_or_default())
		})
		.collect();
	let props_all_columns: Vec<String> = props
		.iter()
		.map(|p| p.column.as_ref().map(|c| c.to_string()).unwrap_or_else(|| p.name.to_string()))
		.collect();

	let props_option_idents: Vec<&Option<Ident>> = props.iter().filter(|p| p.is_option).map(|p| p.ident).collect();
	let props_option_names: Vec<&String> = props.iter().filter(|p| p.is_option).map(|p| &p.name).collect();

	let props_not_option_idents: Vec<&Option<Ident>> = props.iter().filter(|p| !p.is_option).map(|p| p.ident).collect();
	let props_not_option_names: Vec<&String> = props.iter().filter(|p| !p.is_option).map(|p| &p.name).collect();

	// -- Vec push code for the (name, value)
	let ff_all_pushes = quote! {
		#(
			ff.push(
				modql::field::Field::new(modql::SIden(#props_all_names), self.#props_all_idents.into())
			);
		)*
	};

	let ff_not_option_pushes = quote! {
		#(
			ff.push(
				modql::field::Field::new(modql::SIden(#props_not_option_names), self.#props_not_option_idents.into())
			);
		)*
	};

	let ff_option_not_none_pushes = quote! {
		#(
			if let Some(val) = self.#props_option_idents {
				ff.push(
					modql::field::Field::new(modql::SIden(#props_option_names), val.into())
				);
			}
		)*
	};

	// -- Compose the final code
	let output = quote! {
		impl modql::field::HasFields for #struct_name {

			fn not_none_fields<'a>( self) -> modql::field::Fields {
				let mut ff: Vec<modql::field::Field> = Vec::new();
				#ff_not_option_pushes
				#ff_option_not_none_pushes
				modql::field::Fields::new(ff)
			}

			fn all_fields<'a>( self) -> modql::field::Fields {
				let mut ff: Vec<modql::field::Field> = Vec::new();
				#ff_all_pushes
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
