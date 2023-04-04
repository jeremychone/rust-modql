use quote::ToTokens;
use syn::{DeriveInput, Field, FieldsNamed};

/// Returns the fields named of a struct
pub(crate) fn get_struct_fields(ast: &DeriveInput) -> &FieldsNamed {
	let syn::Data::Struct(syn::DataStruct {
		fields: syn::Fields::Named(ref fields),
		..
	}) = ast.data else {
		panic!("Only support Struct")
	};
	fields
}

/// Returns the type_name of a field
pub(crate) fn get_type_name(field: &Field) -> String {
	format!("{}", &field.ty.to_token_stream())
}

// TODO: Needs to make sure to only take the value of context
pub(crate) fn extract_context_attr_value(field: &Field) -> Option<String> {
	// FIXME: Finding the attribute context
	for att in field.attrs.iter() {
		let lit = att.parse_args::<syn::LitStr>();
		if let Ok(lit) = lit {
			let lit = lit.value();
			return Some(lit);
		}
	}
	None
}
