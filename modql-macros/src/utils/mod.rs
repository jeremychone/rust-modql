// region:    --- Modules
pub mod modql_field;
pub mod struct_modql_attr;

use quote::ToTokens;
use syn::{Attribute, DeriveInput, Expr, Field, FieldsNamed, Lit, MetaNameValue};

// endregion: --- Modules

/// Returns the syn:: fields named of a struct
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

pub fn get_dinput_attribute<'a>(dinput: &'a DeriveInput, name: &str) -> Option<&'a Attribute> {
	dinput.attrs.iter().find(|a| a.path().is_ident(name))
}

pub fn get_meta_value_string(nv: MetaNameValue) -> Option<String> {
	if let Expr::Lit(exp_lit) = nv.value {
		if let Lit::Str(lit_str) = exp_lit.lit {
			return Some(lit_str.value());
		}
	}
	None
}
