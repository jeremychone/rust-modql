use crate::utils::{get_dinput_attribute, get_meta_value_string};
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Meta, Token};

// region:    --- Struct Prop Attribute
pub struct StructModqlFieldProp {
	pub rel: Option<String>,
}

pub fn get_modql_struct_prop(dinput: &DeriveInput) -> Result<StructModqlFieldProp, syn::Error> {
	// FIXME: We should remove this, 'sqlb' should not be a thing anymore.
	let sqlb_attr = get_dinput_attribute(dinput, "modql");
	let mut rel = None;

	if let Some(attribute) = sqlb_attr {
		let nested = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

		for meta in nested {
			match meta {
				// #[modql(rel=value)]
				Meta::NameValue(nv) => {
					#[allow(clippy::if_same_then_else)]
					if nv.path.is_ident("rel") {
						rel = get_meta_value_string(nv);
					}
					// NOTE: To be deprecated (should be `rel` for relation)
					else if nv.path.is_ident("table") {
						rel = get_meta_value_string(nv);
					}
				}

				/* ... */
				_ => {
					return Err(syn::Error::new_spanned(meta, "unrecognized field"));
				}
			}
		}
	}

	Ok(StructModqlFieldProp { rel })
}

// endregion: --- Struct Prop Attribute
