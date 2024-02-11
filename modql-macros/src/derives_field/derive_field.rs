use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DataEnum, DeriveInput};

// TODO: Needs to assert that variants do not have any data
pub(crate) fn derive_field_inner(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	let name: Ident = input.ident;

	// Build the match arms and get the first variant
	// Note: At this point, we do not nseed the first_variant anymore since we return
	//       `sea_query::Value::String(None)` for nullable, but we keep the code for future
	//       reference.
	let expanded = match input.data {
		syn::Data::Enum(data) => process_enum(name, data),
		_ => panic!("FieldEnum can only be used with enums"),
	};

	// Return the generated token stream
	TokenStream::from(expanded)
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
