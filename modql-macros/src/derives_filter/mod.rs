mod utils;

use crate::derives_filter::utils::get_filter_field_attr;
use crate::utils::struct_modql_attr::get_struct_modql_props;
use crate::utils::{get_struct_fields, get_type_name};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

pub fn derive_filter_nodes_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);

	//// get struct name and fields
	let struct_name = &ast.ident;
	let fields = get_struct_fields(&ast);

	let struct_attrs = get_struct_modql_props(&ast).unwrap();

	//// Properties to be collected
	let mut props: Vec<&Option<Ident>> = Vec::new(); // not needed for now.
	let mut props_opval_idents: Vec<&Ident> = Vec::new();
	let mut props_opval_rels: Vec<proc_macro2::TokenStream> = Vec::new();
	let mut props_opval_to_sea_holder_fn_build: Vec<proc_macro2::TokenStream> = Vec::new();
	let mut props_filter_node_options: Vec<proc_macro2::TokenStream> = Vec::new();

	for field in fields.named.iter() {
		// NOTE: By macro limitation, we can do only type name match and it would not support type alias
		//       For now, assume Option is use as is, not even in a fully qualified way.
		//       We can add other variants of Option if proven needed
		let type_name = get_type_name(field);

		// NOTE: For now only convert the properties of types with option and OpVal
		if type_name.starts_with("Option ") && type_name.contains("OpVal") {
			if let Some(ident) = field.ident.as_ref() {
				props_opval_idents.push(ident);

				// -- Extract the attributes
				let modql_field_attr = get_filter_field_attr(field).unwrap();

				// -- rel
				let block_rel = if let Some(rel) = modql_field_attr.rel {
					quote! {
						Some(#rel.to_string())
					}
				} else if let Some(struct_rel) = struct_attrs.rel.as_ref() {
					quote! { Some(#struct_rel.to_string()) }
				} else {
					quote! { None }
				};
				props_opval_rels.push(block_rel);

				// -- options: FilterNodeOptions
				let quote_filter_node_options_cast_as = if let Some(cast_as) = modql_field_attr.cast_as {
					quote! { Some(#cast_as.to_string()) }
				} else {
					quote! { None }
				};

				let quote_filter_node_options_cast_column_as = if let Some(cast_column_as) = modql_field_attr.cast_column_as {
					quote! { Some(#cast_column_as.to_string()) }
				} else {
					quote! { None }
				};
				props_filter_node_options.push(quote! {
					modql::filter::FilterNodeOptions {
						cast_as: #quote_filter_node_options_cast_as,
						cast_column_as: #quote_filter_node_options_cast_column_as,
					}
				});

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
				if let Some(val) = self.#props_opval_idents {
					let op_vals: Vec<modql::filter::OpVal> = val.0.into_iter().map(|n| n.into()).collect();
					#props_opval_to_sea_holder_fn_build
					let node = modql::filter::FilterNode{
						rel: #props_opval_rels,
						name: stringify!(#props_opval_idents).to_string(),
						opvals: op_vals,
						options: #props_filter_node_options,
						for_sea_condition: fn_holder,
					};
					nodes.push(node);
				}
			)*
		}
	} else {
		quote! {
			#(
				if let Some(val) = self.#props_opval_idents {
					let op_vals: Vec<modql::filter::OpVal> = val.0.into_iter().map(|n| n.into()).collect();
					let node = modql::filter::FilterNode{
						rel: #props_opval_rels,
						name: stringify!(#props_opval_idents).to_string(),
						opvals: op_vals,
						options: #props_filter_node_options,
					};
					nodes.push(node);
				}
			)*
		}
	};

	//// Out code for the impl IntoFilterNodes
	let out_impl_into_filter_nodes = quote! {
		impl modql::filter::IntoFilterNodes for #struct_name {
			fn filter_nodes(self, rel: Option<String>) -> Vec<modql::filter::FilterNode> {
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
				type Error = modql::filter::IntoSeaError;

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
