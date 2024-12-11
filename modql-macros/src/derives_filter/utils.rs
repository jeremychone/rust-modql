use crate::utils::{get_field_attribute, get_meta_value_string};
use syn::punctuated::Punctuated;
use syn::Field;
use syn::{Meta, Token};

pub struct MoqlFilterFieldAttr {
	pub rel: Option<String>,
	pub to_sea_condition_fn: Option<String>,
	pub to_sea_value_fn: Option<String>,
	pub cast_as: Option<String>,
	pub cast_column_as: Option<String>,
}

pub fn get_filter_field_attr(field: &Field) -> Result<MoqlFilterFieldAttr, syn::Error> {
	let attribute = get_field_attribute(field, "modql");

	let mut rel: Option<String> = None;
	let mut to_sea_condition_fn: Option<String> = None;
	let mut to_sea_value_fn: Option<String> = None;
	let mut cast_as: Option<String> = None;
	let mut cast_column_as: Option<String> = None;
	if let Some(attribute) = attribute {
		let nested = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

		for meta in nested {
			match meta {
				// #[modql(rel= "project", to_sea_condition_fn = "my_sea_cond_fn_name")]
				Meta::NameValue(nv) => {
					if nv.path.is_ident("to_sea_condition_fn") {
						to_sea_condition_fn = get_meta_value_string(nv);
					} else if nv.path.is_ident("to_sea_value_fn") {
						to_sea_value_fn = get_meta_value_string(nv);
					} else if nv.path.is_ident("cast_as") {
						cast_as = get_meta_value_string(nv);
					} else if nv.path.is_ident("rel") {
						rel = get_meta_value_string(nv);
					} else if nv.path.is_ident("cast_column_as") {
						cast_column_as = get_meta_value_string(nv);
					}
				}

				/* ... */
				_ => {
					let msg = "unrecognized modql attribute value";
					return Err(syn::Error::new_spanned(meta, msg));
				}
			}
		}
	}

	Ok(MoqlFilterFieldAttr {
		rel,
		to_sea_condition_fn,
		to_sea_value_fn,
		cast_as,
		cast_column_as,
	})
}
