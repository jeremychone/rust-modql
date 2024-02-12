use crate::utils::{get_dinput_attribute, get_meta_value_string};
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Meta, Token};

// region:    --- Struct Prop Attribute
pub struct StructModqlFieldProp {
	pub table: Option<String>,
}

pub fn get_modql_struct_prop(dinput: &DeriveInput) -> Result<StructModqlFieldProp, syn::Error> {
	// FIXME: We should remove this, 'sqlb' should not be a thing anymore.
	let sqlb_attr = get_dinput_attribute(dinput, "modql");
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

	Ok(StructModqlFieldProp { table })
}

// endregion: --- Struct Prop Attribute
