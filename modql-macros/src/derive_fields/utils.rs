use crate::utils::{get_field_attribute, get_meta_value_string};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Attribute, DeriveInput, Field, FieldsNamed, Meta, Token};

// region:    --- Field Prop (i.e., sqlb Field)
pub struct Prop<'a> {
	pub table: Option<String>,
	pub column: Option<String>,
	pub name: String,
	pub is_option: bool,
	pub ident: &'a Option<Ident>,
}

pub fn get_field_props(fields: &FieldsNamed) -> Vec<Prop> {
	let mut props = Vec::new();

	for field in fields.named.iter() {
		// -- Get the FieldAttr
		let field_attr = get_field_prop_attr(field);

		// TODO: Need to check better handling.
		let field_attr = field_attr.unwrap();
		if field_attr.skip {
			continue;
		}

		// -- ident
		let ident = &field.ident;

		// -- is_option
		// NOTE: By macro limitation, we can do only type name match and it would not support type alias
		//       For now, assume Option is used as is or type name contains it.
		//       We can add other variants of Option if proven needed.
		let type_name = format!("{}", &field.ty.to_token_stream());
		let is_option = type_name.contains("Option ");

		// -- name
		let name = if let Some(name) = field_attr.name {
			name
		} else {
			ident.as_ref().map(|i| i.to_string()).unwrap()
			// quote! {stringify!(#ident)}
		};

		// -- Add to array.
		props.push(Prop {
			table: field_attr.table,
			column: field_attr.column,
			name,
			is_option,
			ident,
		})
	}

	props
}

// endregion: --- Field Prop (i.e., sqlb Field)

// region:    --- Field Prop Attribute
pub struct FieldPropAttr {
	pub table: Option<String>,
	pub column: Option<String>,
	pub skip: bool,
	pub name: Option<String>,
}

// #[field(skip, name = "new_name")]
// #[field(name = "new_name")]
pub fn get_field_prop_attr(field: &Field) -> Result<FieldPropAttr, syn::Error> {
	let attribute = get_field_attribute(field, "field");

	let mut skip = false;
	let mut name: Option<String> = None;
	let mut table: Option<String> = None;
	let mut column: Option<String> = None;

	if let Some(attribute) = attribute {
		let nested = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

		for meta in nested {
			match meta {
				// #[field(skip)]
				Meta::Path(path) if path.is_ident("skip") => {
					skip = true;
				}

				// #[field(name=value)]
				Meta::NameValue(nv) => {
					if nv.path.is_ident("name") {
						name = get_meta_value_string(nv);
					} else if nv.path.is_ident("table") {
						table = get_meta_value_string(nv);
					} else if nv.path.is_ident("column") {
						column = get_meta_value_string(nv);
					}
				}

				/* ... */
				_ => {
					return Err(syn::Error::new_spanned(meta, "unrecognized field"));
				}
			}
		}
	}

	Ok(FieldPropAttr {
		skip,
		table,
		column,
		name,
	})
}

// endregion: --- Field Prop Attribute

// region:    --- Struct Prop Attribute
pub struct SqlbProp {
	pub table: Option<String>,
}

pub fn get_fields_prop(dinput: &DeriveInput) -> Result<SqlbProp, syn::Error> {
	let sqlb_attr = get_dinput_attribute(dinput, "sqlb");
	let mut table = None;

	if let Some(attribute) = sqlb_attr {
		let nested = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

		for meta in nested {
			match meta {
				// #[sqlb(table=value)]
				Meta::NameValue(nv) => {
					if nv.path.is_ident("table") {
						table = get_meta_value_string(nv);
					}
				}

				/* ... */
				_ => {
					return Err(syn::Error::new_spanned(meta, "unrecognized field"));
				}
			}
		}
	}

	Ok(SqlbProp { table })
}

fn get_dinput_attribute<'a>(dinput: &'a DeriveInput, name: &str) -> Option<&'a Attribute> {
	dinput.attrs.iter().find(|a| a.path().is_ident(name))
}
// endregion: --- Struct Prop Attribute
