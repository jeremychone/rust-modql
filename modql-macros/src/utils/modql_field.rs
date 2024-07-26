use crate::utils::{get_field_attribute, get_meta_value_string};
use proc_macro2::Ident;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::{Field, FieldsNamed, Meta, Token};

// region:    --- Field Prop (i.e., sqlb Field)
pub struct ModqlFieldProp<'a> {
	pub prop_name: String,         // property name
	pub attr_name: Option<String>, // The eventual `#[field(name=..._)]`
	pub name: String,              // resolved name attr_name or prop name;
	pub rel: Option<String>,
	pub cast_as: Option<String>,
	pub is_option: bool,
	pub ident: &'a Option<Ident>,
}

pub fn get_modql_field_props(fields: &FieldsNamed) -> Vec<ModqlFieldProp> {
	let modql_fields_and_skips = get_modql_field_props_and_skips(fields);
	modql_fields_and_skips.modql_fields
}

pub struct ModqlFieldsAndSkips<'a> {
	pub modql_fields: Vec<ModqlFieldProp<'a>>,
	#[allow(unused)] // For early development.
	pub skipped_fields: Vec<&'a Field>,
}

pub fn get_modql_field_props_and_skips(fields: &FieldsNamed) -> ModqlFieldsAndSkips {
	let mut modql_fields = Vec::new();
	let mut skipped_fields = Vec::new();

	for field in fields.named.iter() {
		// -- Get the FieldAttr
		let mfield_attr = get_mfield_prop_attr(field);

		// TODO: Need to check better handling.
		let mfield_attr = mfield_attr.unwrap();
		if mfield_attr.skip {
			skipped_fields.push(field);
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
		let prop_name = ident.as_ref().map(|i| i.to_string()).unwrap();
		let attr_name = mfield_attr.name;
		let name = attr_name.clone().unwrap_or_else(|| prop_name.clone());

		// -- cast_as
		let cast_as = mfield_attr.cast_as;

		// -- Add to array.
		modql_fields.push(ModqlFieldProp {
			rel: mfield_attr.rel,
			name,
			prop_name,
			attr_name,
			ident,
			cast_as,
			is_option,
		})
	}

	ModqlFieldsAndSkips {
		modql_fields,
		skipped_fields,
	}
}

// endregion: --- Field Prop (i.e., sqlb Field)

// region:    --- Field Prop Attribute
struct ModqlFieldPropAttr {
	pub rel: Option<String>,
	pub name: Option<String>,
	pub skip: bool,
	pub cast_as: Option<String>,
}

// #[field(skip)]
// #[field(name = "new_name")]
fn get_mfield_prop_attr(field: &Field) -> Result<ModqlFieldPropAttr, syn::Error> {
	let attribute = get_field_attribute(field, "field");

	let mut skip = false;
	let mut rel: Option<String> = None;
	let mut column: Option<String> = None;
	let mut cast_as: Option<String> = None;

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
					if nv.path.is_ident("rel") {
						rel = get_meta_value_string(nv);
					} else if nv.path.is_ident("name") {
						column = get_meta_value_string(nv);
					} else if nv.path.is_ident("cast_as") {
						cast_as = get_meta_value_string(nv);
					}
				}

				/* ... */
				_ => {
					return Err(syn::Error::new_spanned(
						meta,
						r#"
Unrecognized #[field...] attribute. Accepted attribute
#[field(skip)]
or
#[field(rel="table_name}, name="some_col_name", cast_as="sea query cast as type")]
"#,
					));
				}
			}
		}
	}

	Ok(ModqlFieldPropAttr {
		skip,
		rel,
		name: column,
		cast_as,
	})
}

// endregion: --- Field Prop Attribute
