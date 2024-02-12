use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DataStruct, DeriveInput, Fields, Type};

// TODO: Needs to assert that variants do not have any data
pub(crate) fn derive_field_value_inner(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	let name: Ident = input.ident;

	// Build the match arms and get the first variant
	// Note: At this point, we do not nseed the first_variant anymore since we return
	//       `sea_query::Value::String(None)` for nullable, but we keep the code for future
	//       reference.
	let expanded = match input.data {
		syn::Data::Enum(data) => process_enum(name, data),
		syn::Data::Struct(data) => process_struct(name, data),
		_ => panic!("Field can only be used with enums and tuple struct for now"),
	};

	// Return the generated token stream
	TokenStream::from(expanded)
}

fn process_struct(name: Ident, data: DataStruct) -> proc_macro2::TokenStream {
	let first_tuple_field = match data.fields {
		Fields::Unnamed(fields) if fields.unnamed.len() == 1 => fields.unnamed.into_iter().next().unwrap(),
		_ => panic!("Expected a tuple struct with one field"),
	};

	let field_type = first_tuple_field.ty;

	// NOTE: It is important to extract the single TypePath from a potential TypeGroup
	//       to increase resilience with certain declarative macros (e.g., derive_aliases_...!{ struct ...}).
	let field_type_path = match field_type {
		Type::Group(group) => match *group.elem {
			Type::Path(p) => p,
			_ => panic!("Unsupported type... TypeGroup.elem is not a path"),
		},
		Type::Path(p) => p,
		_ => panic!("Unsupported type... not Type::Path or Type::Group"),
	};

	let value_variant = match field_type_path.path.get_ident() {
		Some(ident) => match ident.to_string().as_str() {
			"bool" => quote! { Bool },
			"i8" => quote! { TinyInt },
			"i16" => quote! { SmallInt },
			"i32" => quote! { Int },
			"i64" => quote! { BigInt },
			"u8" => quote! { TinyUnsigned },
			"u16" => quote! { SmallUnsigned },
			"u32" => quote! { Unsigned },
			"u64" => quote! { BigUnsigned },
			"f32" => quote! { Float },
			"f64" => quote! { Double },
			"String" => quote! { String },
			"char" => quote! { Char },
			// TODO: add more type support
			_ => panic!("Unsupported type... {:?}", ident),
		},
		None => panic!("Unsupported type... no ident found"),
	};

	// Determine the appropriate Value variant based on the type of the field
	// let value_variant = match field_type {
	// 	Type::Path(p) if p.path.is_ident("bool") => quote! { Bool },
	// 	Type::Path(p) if p.path.is_ident("i8") => quote! { TinyInt },
	// 	Type::Path(p) if p.path.is_ident("i16") => quote! { SmallInt },
	// 	Type::Path(p) if p.path.is_ident("i32") => quote! { Int },
	// 	Type::Path(p) if p.path.is_ident("i64") => quote! { BigInt },
	// 	Type::Path(p) if p.path.is_ident("u8") => quote! { TinyUnsigned },
	// 	Type::Path(p) if p.path.is_ident("u16") => quote! { SmallUnsigned },
	// 	Type::Path(p) if p.path.is_ident("u32") => quote! { Unsigned },
	// 	Type::Path(p) if p.path.is_ident("u64") => quote! { BigUnsigned },
	// 	Type::Path(p) if p.path.is_ident("f32") => quote! { Float },
	// 	Type::Path(p) if p.path.is_ident("f64") => quote! { Double },
	// 	Type::Path(p) if p.path.is_ident("String") => quote! { String },
	// 	Type::Path(p) if p.path.is_ident("char") => quote! { Char },
	// 	Type::Group(g) => panic!("Unsupported type group... {:?} ...", g),
	// 	// TODO: Add more sea-query types
	// 	_ => panic!("Unsupported type... {:?} ...", field_type),
	// };

	let expanded = quote! {
		impl From<#name> for sea_query::Value {
			fn from(value: #name) -> Self {
				Self::#value_variant(Some(value.0))
			}
		}

		impl sea_query::Nullable for #name {
			fn null() -> sea_query::Value {
				sea_query::Value::#value_variant(None)
			}
		}
	};

	expanded
}

fn process_enum(name: Ident, data_enum: DataEnum) -> proc_macro2::TokenStream {
	let mut first_variant = None;
	let arms = data_enum
		.variants
		.iter()
		.map(|variant| {
			let variant_name = &variant.ident;
			let variant_name_str = variant_name.to_string();
			if first_variant.is_none() {
				first_variant = Some(variant_name.clone());
			}
			quote! {
				#name::#variant_name => #variant_name_str.into(),
			}
		})
		.collect::<Vec<_>>();

	// Note: Note needed anymore, but keep for code example.
	// let first_variant = first_variant.expect("Enum must have at least one variant");

	// Generate the final token stream
	let expanded = quote! {
		impl From<#name> for sea_query::Value {
			fn from(val: #name) -> Self {
				match val {
					#(#arms)*
				}
			}
		}

		impl sea_query::Nullable for #name {
			fn null() -> sea_query::Value {
				// #name::#first_variant.into()
				sea_query::Value::String(None)
			}
		}
	};

	expanded
}
