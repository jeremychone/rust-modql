use crate::utils::modql_field::ModqlFieldProp;
use crate::utils::struct_modql_attr::{get_struct_modql_props, StructModqlFieldProps};
use crate::utils::{get_struct_fields, modql_field};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Index};

pub(crate) fn derive_fields_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let fields = get_struct_fields(&ast);

	let struct_name = &ast.ident;

	// -- Collect Elements
	// Properties for all fields (with potential additional info with #[field(...)])
	let field_props = modql_field::get_modql_field_props(fields);
	let struct_modql_prop = get_struct_modql_props(&ast).unwrap();

	let impl_has_fields = impl_has_fields(struct_name, &struct_modql_prop, &field_props);

	let impl_names_as_consts = if let Some(names_as_consts) = struct_modql_prop.names_as_consts.as_deref() {
		impl_names_as_consts(struct_name, &field_props, names_as_consts)
	} else {
		quote! {}
	};

	// -- impl HasSeaFields
	let impl_sea_fields = if cfg!(feature = "with-sea-query") {
		impl_has_sea_fields(struct_name, &struct_modql_prop, &field_props)
	} else {
		quote! {}
	};

	// -- impl HasSqliteFields
	let impl_sqlite_fields = if cfg!(feature = "with-rusqlite") {
		impl_has_sqlite_fields(struct_name, &struct_modql_prop, &field_props)
	} else {
		quote! {}
	};

	let output = quote! {
		#impl_has_fields

		#impl_names_as_consts

		#impl_sea_fields

		#impl_sqlite_fields
	};

	output.into()
}

fn impl_names_as_consts(
	struct_name: &Ident,
	field_props: &[ModqlFieldProp<'_>],
	prop_name_prefix: &str,
) -> proc_macro2::TokenStream {
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

	let props_field_metas = field_props.iter().map(|field_prop| {
		let prop_name = field_prop.prop_name.to_string();

		let attr_name = match field_prop.attr_name.as_ref() {
			Some(attr_name) => quote! { Some(#attr_name)},
			None => quote! { None },
		};

		let field_rel = field_prop.rel.as_ref();

		let is_struct_rel = match (struct_rel, field_rel) {
			(Some(_), None) => true,
			(Some(struct_rel), Some(field_rel)) => struct_rel == field_rel,
			_ => false,
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

		quote! {
			&modql::field::FieldMeta{
				rel: #rel,
				is_struct_rel: #is_struct_rel,
				prop_name: #prop_name,
				attr_name: #attr_name,
				cast_as: #cast_as,
				is_option: #is_option,
			}
		}
	});

	let const_field_metas = quote! {
		impl #struct_name {
			pub const __MODQL_FIELD_METAS: &'static [&'static modql::field::FieldMeta] = &[
				#(#props_field_metas,)*];
		}
	};

	let has_fields_impl = quote! {
		impl modql::field::HasFields for #struct_name {
			fn field_names() -> &'static [&'static str] {
				&[#(#props_all_names,)*]
			}

			fn field_metas() -> &'static modql::field::FieldMetas {
				static METAS_HOLDER: modql::field::FieldMetas =
					modql::field::FieldMetas::new(#struct_name::__MODQL_FIELD_METAS);
				&METAS_HOLDER
			}
		}
	};

	quote! {
		#const_field_metas
		#has_fields_impl
	}
}

fn impl_has_sea_fields(
	struct_name: &Ident,
	struct_modql_prop: &StructModqlFieldProps,
	field_props: &[ModqlFieldProp<'_>],
) -> proc_macro2::TokenStream {
	let prop_all_names: Vec<&String> = field_props.iter().map(|p| &p.name).collect();

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
			quote! { modql::field::SeaFieldOptions { cast_as: Some(#cast_as.to_string()) } }
		} else {
			quote! { modql::field::SeaFieldOptions { cast_as: None } }
		}
	}

	// -- all_fields() quotes!
	let all_fields_quotes = field_props.iter().map(|p| {
		let name = &p.name;
		let field_options_q = field_options_quote(p);
		let ident = p.ident;

		quote! {
			ff.push(
				modql::field::SeaField::new_with_options(modql::SIden(#name), self.#ident.into(), #field_options_q)
			);
		}
	});

	// -- The not_none_sea_fields quotes!
	let not_none_fields_quotes = field_props.iter().map(|p| {
		let name = &p.name;
		let field_options_q = field_options_quote(p);
		let ident = p.ident;

		if p.is_option {
			quote! {
				if let Some(val) = self.#ident {
					ff.push(
						modql::field::SeaField::new_with_options(modql::SIden(#name), val.into(), #field_options_q)
					);
				}
			}
		} else {
			quote! {
				ff.push(
					modql::field::SeaField::new_with_options(modql::SIden(#name), self.#ident.into(), #field_options_q)
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
				vec![#( sea_query::IntoIden::into_iden(modql::SIden(#prop_all_names)), )*]
			}

			fn sea_column_refs() -> Vec<sea_query::ColumnRef> {
				use sea_query::{ColumnRef, IntoIden};
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
				use sea_query::{ColumnRef, IntoIden};
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

fn impl_has_sqlite_fields(
	struct_name: &Ident,
	_struct_modql_prop: &StructModqlFieldProps,
	field_props: &[ModqlFieldProp<'_>],
) -> proc_macro2::TokenStream {
	let prop_all_names: Vec<&String> = field_props.iter().map(|p| &p.name).collect();

	let all_fields_quotes = field_props.iter().enumerate().map(|(idx, p)| {
		let idx_lit = Index::from(idx);
		let name = &p.name;
		let ident = p.ident;

		quote! {
			ff.push(
				modql::field::SqliteField::new_with_options_meta(
					#name,
					self.#ident.into(),
					Self::__MODQL_FIELD_METAS[#idx_lit]
				)
			);
		}
	});

	let not_none_fields_quotes = field_props.iter().enumerate().map(|(idx, p)| {
		let idx_lit = Index::from(idx);
		let name = &p.name;
		let ident = p.ident;

		if p.is_option {
			quote! {
				if let Some(val) = self.#ident {
					ff.push(
						modql::field::SqliteField::new_with_options_meta(
							#name,
							val.into(),
							Self::__MODQL_FIELD_METAS[#idx_lit]
						)
					);
				}
			}
		} else {
			quote! {
				ff.push(
					modql::field::SqliteField::new_with_options_meta(
						#name,
						self.#ident.into(),
						Self::__MODQL_FIELD_METAS[#idx_lit]
					)
				);
			}
		}
	});

	let output = quote! {
		impl modql::field::HasSqliteFields for #struct_name {
			fn sqlite_not_none_fields(self) -> modql::field::SqliteFields {
				let mut ff: Vec<modql::field::SqliteField> = Vec::new();
				#(#not_none_fields_quotes)*
				modql::field::SqliteFields::new(ff)
			}

			fn sqlite_all_fields(self) -> modql::field::SqliteFields {
				let mut ff: Vec<modql::field::SqliteField> = Vec::new();
				#(#all_fields_quotes)*
				modql::field::SqliteFields::new(ff)
			}

			fn sqlite_column_refs_with_rel(rel: &'static str) -> Vec<modql::field::SqliteColumnRef> {
				vec![
					#( modql::field::SqliteColumnRef{ rel: Some(rel), col: #prop_all_names }, )*
				]
			}
		}
	};

	output
}
