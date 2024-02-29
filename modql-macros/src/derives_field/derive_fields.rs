use crate::utils::modql_field::ModqlFieldProp;
use crate::utils::struct_modql_attr::{get_modql_struct_prop, StructModqlFieldProp};
use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn derive_fields_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let fields = get_struct_fields(&ast);

	let struct_name = &ast.ident;

	// -- Collect Elements
	// Properties for all fields (with potential additional info with #[field(...)])
	let field_props = modql_field::get_modql_field_props(fields);
	let struct_modql_prop = get_modql_struct_prop(&ast).unwrap();

	// Will be "" if none (this if for the struct #[modql(table = ...)])

	let impl_base_fields = impl_has_fields(struct_name, &struct_modql_prop, &field_props);

	let impl_sea_fields = if cfg!(feature = "with-sea-query") {
		impl_has_sea_fields(struct_name, &struct_modql_prop, &field_props)
	} else {
		quote! {}
	};

	let output = quote! {
		#impl_base_fields

		#impl_sea_fields
	};

	output.into()
}

fn impl_has_fields(
	struct_name: &Ident,
	struct_modql_prop: &StructModqlFieldProp,
	field_props: &[ModqlFieldProp<'_>],
) -> proc_macro2::TokenStream {
	let props_all_names: Vec<&String> = field_props.iter().map(|p| &p.name).collect();

	let struct_rel = struct_modql_prop.rel.as_ref();

	let props_field_refs = field_props.iter().map(|field_prop| {
		let name = field_prop.name.to_string();
		let rel = field_prop.rel.as_ref().or(struct_rel);
		let rel = match rel {
			Some(rel) => quote! { Some(#rel)},
			None => quote! { None },
		};
		quote! {&modql::field::FieldRef{rel: #rel, name: #name}}
	});

	let output = quote! {

		impl modql::field::HasFields for #struct_name {


			fn field_names() -> &'static [&'static str] {
				&[#(
				#props_all_names,
				)*]
			}


			fn field_refs() -> &'static [&'static modql::field::FieldRef] {
				&[#(
				#props_field_refs,
				)*]
			}
		}
	};

	output
}

fn impl_has_sea_fields(
	struct_name: &Ident,
	struct_modql_prop: &StructModqlFieldProp,
	field_props: &[ModqlFieldProp<'_>],
) -> proc_macro2::TokenStream {
	let props_all_names: Vec<&String> = field_props.iter().map(|p| &p.name).collect();

	// this will repeat the struct table name for all fields.
	let props_all_tables: Vec<String> = field_props
		.iter()
		.map(|p| {
			p.rel
				.as_ref()
				.map(|t| t.to_string())
				.unwrap_or_else(|| struct_modql_prop.rel.as_ref().map(|s| s.to_string()).unwrap_or_default())
		})
		.collect();

	fn field_options_quote(mfield_prop: &ModqlFieldProp) -> proc_macro2::TokenStream {
		if let Some(cast_as) = &mfield_prop.cast_as {
			quote! { modql::field::FieldOptions { cast_as: Some(#cast_as.to_string()) } }
		} else {
			quote! { modql::field::FieldOptions { cast_as: None } }
		}
	}

	// -- all_fields() quotes!
	let all_fields_quotes = field_props.iter().map(|p| {
		let name = &p.name;
		let field_options = field_options_quote(p);
		let ident = p.ident;

		quote! {
			ff.push(
				modql::field::SeaField::new_with_options(modql::SIden(#name), self.#ident.into(), #field_options)
			);
		}
	});

	// -- The not_none_sea_fields quotes!
	let not_none_fields_quotes = field_props.iter().map(|p| {
		let name = &p.name;
		let field_options = field_options_quote(p);
		let ident = p.ident;

		if p.is_option {
			quote! {
					if let Some(val) = self.#ident {
						ff.push(
							modql::field::SeaField::new_with_options(modql::SIden(#name), val.into(), #field_options)
						);
					}
			}
		} else {
			quote! {
					ff.push(
						modql::field::SeaField::new_with_options(modql::SIden(#name), self.#ident.into(), #field_options)
					);
			}
		}
	});

	// -- Compose the final code
	let output = quote! {

		impl modql::field::HasSeaFields for #struct_name {

			fn not_none_sea_fields(self) -> modql::field::SeaFields {
				let mut ff: Vec<modql::field::SeaField> = Vec::new();
				#(#not_none_fields_quotes)*
				modql::field::SeaFields::new(ff)
			}

			fn all_sea_fields(self) -> modql::field::SeaFields {
				let mut ff: Vec<modql::field::SeaField> = Vec::new();
				#(#all_fields_quotes)*
				modql::field::SeaFields::new(ff)
			}

			fn sea_idens() -> Vec<sea_query::SeaRc<dyn sea_query::Iden>> {
				vec![#(
				sea_query::IntoIden::into_iden(modql::SIden(#props_all_names)),
				)*]
			}

			fn sea_column_refs() -> Vec<sea_query::ColumnRef> {
				use sea_query::IntoIden;
				use sea_query::ColumnRef;
				use modql::SIden;

				let mut v = Vec::new();

				// NOTE: There's likely a more elegant solution, but this approach is semantically correct.
				#(
					let col_ref = if #props_all_tables == "" {
						ColumnRef::Column(SIden(#props_all_names).into_iden())
					} else {
						ColumnRef::TableColumn(
							SIden(#props_all_tables).into_iden(),
							SIden(#props_all_names).into_iden())
					};
					v.push(col_ref);
				)*
				v
			}

			fn sea_column_refs_with_rel(rel_iden: impl sea_query::IntoIden) -> Vec<sea_query::ColumnRef> {
				use sea_query::IntoIden;
				use sea_query::ColumnRef;
				use modql::SIden;

				let rel_iden = rel_iden.into_iden();

				let mut v = Vec::new();

				// NOTE: There's likely a more elegant solution, but this approach is semantically correct.
				#(
					let col_ref =
						ColumnRef::TableColumn(
							rel_iden.clone(),
							SIden(#props_all_names).into_iden());

					v.push(col_ref);
				)*
				v
			}
		}
	};

	output
}
