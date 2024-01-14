use crate::utils::get_meta_value_string;
use syn::punctuated::Punctuated;
use syn::{Attribute, DeriveInput, Meta, Token};

// region:    --- SQLB TO BE DEPRECATED: Struct Prop Attribute
pub struct SqlbProp {
	pub table: Option<String>,
}

pub fn get_fields_sqlb_prop(dinput: &DeriveInput) -> Result<SqlbProp, syn::Error> {
	// FIXME: We should remove this, 'sqlb' should not be a thing anymore.
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
// endregion: --- SQLB TO BE DEPRECATED: Struct Prop Attribute
