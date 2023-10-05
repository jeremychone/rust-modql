use crate::utils::{extract_context_attr_value, get_struct_fields, get_type_name};
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

	for field in fields.named.iter() {
		// NOTE: By macro limitation, we can do only type name match and it would not support type alias
		//       For now, assume Option is use as is, not even in a fully qualified way.
		//       We can add other variants of Option if proven needed
		let type_name = get_type_name(field);

		// NOTE: For now only convert the properties of types with option and OpVal
		if type_name.starts_with("Option ") && type_name.contains("OpVal") {
			if let Some(ident) = field.ident.as_ref() {
				prop_opval_idents.push(ident);
				let block = if let Some(ctx) = extract_context_attr_value(field) {
					quote! {
						Some(#ctx.to_string())
					}
				} else {
					quote! {
						None
					}
				};

				props_opval_contexts.push(block);
			}
		} else {
			props.push(&field.ident);
		}
	}

	//// Generate each type of .pushes code block
	// let ff_pushes = quote! {
	// 	#(
	// 		ff.push((stringify!(#props), self.#props.clone()).into());
	// 	)*
	// };

	let ff_opt_pushes = quote! {
		#(
			if let Some(val) = self.#prop_opval_idents {
				let node = modql::filter::FilterNode {
					context_path: #props_opval_contexts,
					name: stringify!(#prop_opval_idents).to_string(),
					opvals: val.0.into_iter().map(|n| n.into()).collect(),
				};
				nodes.push(node);
			}
		)*
	};

	//// Out code for the impl IntoFilterNodes
	let out_impl_into_filter_nodes = quote! {
		impl modql::filter::IntoFilterNodes for #struct_name {
			fn filter_nodes(self, context: Option<String>) -> Vec<modql::filter::FilterNode> {
				let mut nodes = Vec::new();
				#ff_opt_pushes
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

			impl modql::filter::SeaFilter for #struct_name {
				fn into_sea_condition(self) -> sea_query::Condition {
					use modql::filter::SeaFilter;
					use modql::filter::FilterGroup;
					FilterGroup::from(self).into_sea_condition()
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
