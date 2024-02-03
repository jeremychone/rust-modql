use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// TODO: Needs to assert that variants do not have any data
pub(crate) fn derive_field_enum_inner(input: TokenStream) -> TokenStream {
	// Parse the input tokens into a syntax tree
	let input = parse_macro_input!(input as DeriveInput);

	let name = input.ident;

	// Build the match arms and get the first variant
	let mut first_variant = None;
	let arms = match input.data {
		syn::Data::Enum(data) => data
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
			.collect::<Vec<_>>(),
		_ => panic!("SeaEnum can only be used with enums"),
	};

	let first_variant = first_variant.expect("Enum must have at least one variant");

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
				#name::#first_variant.into()
			}
		}
	};

	// Return the generated token stream
	TokenStream::from(expanded)
}
