#![allow(unused)] // For early development.
use quote::ToTokens;
use syn::{Attribute, DeriveInput, Expr, Field, FieldsNamed, Lit, MetaNameValue};

/// Returns the fields named of a struct
pub(crate) fn get_struct_fields(ast: &DeriveInput) -> &FieldsNamed {
	let syn::Data::Struct(syn::DataStruct {
		fields: syn::Fields::Named(ref fields),
		..
	}) = ast.data
	else {
		panic!("Only support Struct")
	};
	fields
}

/// Returns the type_name of a field
pub(crate) fn get_type_name(field: &Field) -> String {
	format!("{}", &field.ty.to_token_stream())
}

pub fn get_field_attribute<'a>(field: &'a Field, name: &str) -> Option<&'a Attribute> {
	field.attrs.iter().find(|a| a.path().is_ident(name))
}

pub fn get_meta_value_string(nv: MetaNameValue) -> Option<String> {
	if let Expr::Lit(exp_lit) = nv.value {
		if let Lit::Str(lit_str) = exp_lit.lit {
			return Some(lit_str.value());
		}
	}
	None
}

// region:    --- Old

// TODO: Needs to make sure to only take the value of context
// pub(crate) fn DEPREATED_extract_context_attr_value(field: &Field) -> Option<String> {
// 	// FIXME: Finding the attribute context
// 	for att in field.attrs.iter() {
// 		let lit = att.parse_args::<syn::LitStr>();
// 		if let Ok(lit) = lit {
// 			let lit = lit.value();
// 			return Some(lit);
// 		}
// 	}
// 	None
// }

// endregion: --- Old
