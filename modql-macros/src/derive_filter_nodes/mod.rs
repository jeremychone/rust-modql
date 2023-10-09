mod utils;

use crate::derive_filter_nodes::utils::get_modql_field_attr;
use crate::utils::{get_struct_fields, get_type_name};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

pub fn derive_filter_nodes_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);

	//// get struct name and fields
	let struct_name = &ast.ident;
	let fields = get_struct_fields(&ast);

	//// Properties to be collected
	let mut props: Vec<&Option<Ident>> = Vec::new(); // not needed for now.
	let mut prop_opval_idents: Vec<&Ident> = Vec::new();
	let mut props_opval_contexts: Vec<proc_macro2::TokenStream> = Vec::new();
	let mut props_opval_to_sea_holder_fn_build: Vec<proc_macro2::TokenStream> = Vec::new();

	for field in fields.named.iter() {
		// NOTE: By macro limitation, we can do only type name match and it would not support type alias
		//       For now, assume Option is use as is, not even in a fully qualified way.
		//       We can add other variants of Option if proven needed
		let type_name = get_type_name(field);

		// NOTE: For now only convert the properties of types with option and OpVal
		if type_name.starts_with("Option ") && type_name.contains("OpVal") {
			if let Some(ident) = field.ident.as_ref() {
				prop_opval_idents.push(ident);

				// -- Extract the attributes
				let modql_field_attr = get_modql_field_attr(field).unwrap();

				// -- context_path
				let block_context = if let Some(context_path) = modql_field_attr.context_path {
					quote! {
						Some(#context_path.to_string())
					}
				} else {
					quote! { None }
				};
				props_opval_contexts.push(block_context);

				// -- to_sea_holder_build
				if cfg!(feature = "with-sea-query") {
					// TODO: Fail if both to_sea_condition_fn and to_sea_value_fn are defined

					let block = if let Some(to_sea_condition_fn) = modql_field_attr.to_sea_condition_fn {
						let to_sea_condition_fn = syn::Ident::new(&to_sea_condition_fn, proc_macro2::Span::call_site());
						quote! {
							// None
							let fn_holder = modql::filter::ToSeaConditionFnHolder::new(#to_sea_condition_fn);
							let fn_holder = Some(fn_holder.into());
						}
					} else if let Some(to_sea_value_fn) = modql_field_attr.to_sea_value_fn {
						let to_sea_value_fn = syn::Ident::new(&to_sea_value_fn, proc_macro2::Span::call_site());
						quote! {
							// None
							let fn_holder = modql::filter::ToSeaValueFnHolder::new(#to_sea_value_fn);
							let fn_holder = Some(fn_holder.into());
						}
					} else {
						quote! {
							let fn_holder = None;
						}
					};
					props_opval_to_sea_holder_fn_build.push(block);
				}
			}
		} else {
			props.push(&field.ident);
		}
	}

	let ff_opt_node_pushes = if cfg!(feature = "with-sea-query") {
		quote! {
			#(
				if let Some(val) = self.#prop_opval_idents {
					let op_vals: Vec<modql::filter::OpVal> = val.0.into_iter().map(|n| n.into()).collect();
					#props_opval_to_sea_holder_fn_build
					let node = modql::filter::FilterNode{
						context_path: #props_opval_contexts,
						name: stringify!(#prop_opval_idents).to_string(),
						opvals: op_vals,
						for_sea_condition: fn_holder,
					};
					nodes.push(node);
				}
			)*
		}
	} else {
		quote! {
			#(
				if let Some(val) = self.#prop_opval_idents {
					let op_vals: Vec<modql::filter::OpVal> = val.0.into_iter().map(|n| n.into()).collect();
					let node = modql::filter::FilterNode{
						context_path: #props_opval_contexts,
						name: stringify!(#prop_opval_idents).to_string(),
						opvals: op_vals,
					};
					nodes.push(node);
				}
			)*
		}
	};

	//// Out code for the impl IntoFilterNodes
	let out_impl_into_filter_nodes = quote! {
		impl modql::filter::IntoFilterNodes for #struct_name {
			fn filter_nodes(self, context: Option<String>) -> Vec<modql::filter::FilterNode> {
				let mut nodes = Vec::new();
				#ff_opt_node_pushes
				nodes
			}
		}
	};

	//// Out code for the from struct for Vec<FilterNode>
	let out_into_filter_node = quote! {
		impl From<#struct_name> for Vec<modql::filter::FilterNode> {
			fn from(val: #struct_name) -> Self {
				modql::filter::IntoFilterNodes::filter_nodes(val, None)
			}
		}
	};

	let out_into_op_group = quote! {
		impl From<#struct_name> for modql::filter::FilterGroup {
			fn from(val: #struct_name) -> Self {
				let nodes: Vec<modql::filter::FilterNode> = val.into();
				nodes.into()
			}
		}
	};

	//// Out code for from struct for FilterGroups
	let out_into_op_groups = quote! {
		impl From<#struct_name> for modql::filter::FilterGroups {
			fn from(val: #struct_name) -> Self {
				let nodes: Vec<modql::filter::FilterNode> = val.into();
				nodes.into()
			}
		}
	};

	let out_sea_filter = if cfg!(feature = "with-sea-query") {
		quote! {
			impl TryFrom<#struct_name> for sea_query::Condition {
				type Error = modql::filter::SeaError;

				fn try_from(val: #struct_name) -> modql::filter::SeaResult<Self> {
					modql::filter::FilterGroup::from(val).try_into()
				}
			}
		}
	} else {
		quote! {}
	};

	//// Final out code
	let output = quote! {
		#out_impl_into_filter_nodes
		#out_into_filter_node
		#out_into_op_group
		#out_into_op_groups
		#out_sea_filter
	};

	output.into()
}
