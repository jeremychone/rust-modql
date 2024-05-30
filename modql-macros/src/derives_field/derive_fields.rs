use crate::utils::modql_field::ModqlFieldProp;
use crate::utils::struct_modql_attr::{get_struct_modql_props, StructModqlFieldProps};
use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn derive_fields_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let fields = get_struct_fields(&ast);

	let struct_name = &ast.ident;

	// -- Collect Elements
	// Properties for all fields (with potential additional info with #[field(...)])
	let field_props = modql_field::get_modql_field_props(fields);
	let struct_modql_prop = get_struct_modql_props(&ast).unwrap();

	// Will be "" if none (this if for the struct #[modql(table = ...)])

	let impl_has_fields = impl_has_fields(struct_name, &struct_modql_prop, &field_props);

	let impl_names_as_consts = if let Some(names_as_consts) = struct_modql_prop.names_as_consts.as_deref() {
		//
		impl_names_as_consts(struct_name, &field_props, names_as_consts)
	} else {
		quote! {}
	};

	let impl_sea_fields = if cfg!(feature = "with-sea-query") {
		impl_has_sea_fields(struct_name, &struct_modql_prop, &field_props)
	} else {
		quote! {}
	};

	let output = quote! {
		#impl_has_fields

		#impl_names_as_consts

		#impl_sea_fields
	};

	output.into()
}

fn impl_names_as_consts(
	struct_name: &Ident,
	field_props: &[ModqlFieldProp<'_>],
	prop_name_prefix: &str,
) -> proc_macro2::TokenStream {
	// If prefix not empty, amek sure it ends with `_`
	let prop_name_prefix = if !prop_name_prefix.is_empty() && !prop_name_prefix.ends_with('_') {
		format!("{prop_name_prefix}_")
	} else {
		prop_name_prefix.to_string()
	};

	let consts = field_props.iter().map(|field| {
		let prop_name = &field.prop_name;
		let const_name = format!("{}{}", prop_name_prefix, prop_name.to_uppercase());
		let const_name = Ident::new(&const_name, Span::call_site());

		let name = &field.name;
		quote! {
			pub const #const_name: &'static str = #name;
		}
	});

	quote! {
		impl #struct_name {
			#(#consts)*
		}
	}
}

fn impl_has_fields(
	struct_name: &Ident,
	struct_modql_prop: &StructModqlFieldProps,
	field_props: &[ModqlFieldProp<'_>],
) -> proc_macro2::TokenStream {
	let props_all_names: Vec<&String> = field_props.iter().map(|p| &p.name).collect();

	let struct_rel = struct_modql_prop.rel.as_ref();

	// -- Build FieldRef quotes
	let props_field_refs = field_props.iter().map(|field_prop| {
		let name = field_prop.name.to_string();
		let rel = field_prop.rel.as_ref().or(struct_rel);
		let rel = match rel {
			Some(rel) => quote! { Some(#rel)},
			None => quote! { None },
		};
		quote! {&modql::field::FieldRef{rel: #rel, name: #name}}
	});

	// -- Build the FieldMeta quotes
	let props_field_metas = field_props.iter().map(|field_prop| {
		// This below is resolved in the FieldMeta implemntation (same logic)
		// let name = field_prop.name.to_string();

		let prop_name = field_prop.prop_name.to_string();

		let attr_name = match field_prop.attr_name.as_ref() {
			Some(attr_name) => quote! { Some(#attr_name)},
			None => quote! { None },
		};

		let rel = field_prop.rel.as_ref().or(struct_rel);
		let rel = match rel {
			Some(rel) => quote! { Some(#rel)},
			None => quote! { None },
		};
		let cast_as = match &field_prop.cast_as {
			Some(cast_as) => quote! { Some(#cast_as)},
			None => quote! { None },
		};
		let is_option = field_prop.is_option;

		quote! {&modql::field::FieldMeta{
				rel: #rel,
				prop_name: #prop_name,
				attr_name: #attr_name,
				cast_as: #cast_as,
				is_option: #is_option,
			}
		}
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

			fn field_metas() -> &'static modql::field::FieldMetas {
				static METAS: &[&modql::field::FieldMeta] = &[#(
				#props_field_metas,
				)*];

				static METAS_HOLDER: modql::field::FieldMetas = modql::field::FieldMetas::new(METAS);

				&METAS_HOLDER
			}

		}
	};

	output
}

fn impl_has_sea_fields(
	struct_name: &Ident,
	struct_modql_prop: &StructModqlFieldProps,
	field_props: &[ModqlFieldProp<'_>],
) -> proc_macro2::TokenStream {
	let prop_all_names: Vec<&String> = field_props.iter().map(|p| &p.name).collect();

	// this will repeat the struct table name for all fields.
	let prop_all_rels: Vec<String> = field_props
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
				sea_query::IntoIden::into_iden(modql::SIden(#prop_all_names)),
				)*]
			}

			fn sea_column_refs() -> Vec<sea_query::ColumnRef> {
				use sea_query::IntoIden;
				use sea_query::ColumnRef;
				use modql::SIden;

				let mut v = Vec::new();

				// NOTE: There's likely a more elegant solution, but this approach is semantically correct.
				#(
					let col_ref = if #prop_all_rels == "" {
						ColumnRef::Column(SIden(#prop_all_names).into_iden())
					} else {
						ColumnRef::TableColumn(
							SIden(#prop_all_rels).into_iden(),
							SIden(#prop_all_names).into_iden())
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
							SIden(#prop_all_names).into_iden());

					v.push(col_ref);
				)*
				v
			}
		}
	};

	output
}
