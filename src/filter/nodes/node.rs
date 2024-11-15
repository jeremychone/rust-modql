use crate::filter::ops::OpVal;
use crate::filter::{OpValBool, OpValFloat64, OpValInt32, OpValInt64, OpValString};

pub trait IntoFilterNodes {
	fn filter_nodes(self, rel: Option<String>) -> Vec<FilterNode>;
}

#[derive(Debug, Clone, Default)]
pub struct FilterNodeOptions {
	pub cast_as: Option<String>, // for db casting. e.g., Will be applied to sea-query value.
	pub cast_column_as: Option<String>, // for db casting. e.g., Will be applied to sea-query column.
}

#[derive(Debug, Clone)]
pub struct FilterNode {
	pub rel: Option<String>, // would be for the project.title (project in this case)
	pub name: String,
	pub opvals: Vec<OpVal>,
	pub options: FilterNodeOptions,

	#[cfg(feature = "with-sea-query")]
	pub for_sea_condition: Option<crate::filter::ForSeaCondition>,
}

impl FilterNode {
	pub fn new(name: impl Into<String>, opvals: impl Into<Vec<OpVal>>) -> FilterNode {
		FilterNode {
			rel: None,
			name: name.into(),
			opvals: opvals.into(),
			options: FilterNodeOptions::default(),

			#[cfg(feature = "with-sea-query")]
			for_sea_condition: None,
		}
	}

	pub fn new_with_rel(rel: Option<String>, name: impl Into<String>, opvals: impl Into<Vec<OpVal>>) -> FilterNode {
		FilterNode {
			rel,
			name: name.into(),
			opvals: opvals.into(),
			options: FilterNodeOptions::default(),

			#[cfg(feature = "with-sea-query")]
			for_sea_condition: None,
		}
	}
}

// region:    --- From Tuples (OpValType)
// Implements the From trait from tuples to FilterNode
macro_rules! from_tuples_opval {
	($($OV:ident),+) => {
		$(
			/// From trait from (prop_name, OpVal) for FilterNode
			/// (e.g., `let node: FilterNode = ("id", IntOpVal::Gt(1)).into()`)
			impl From<(&str, $OV)> for FilterNode {
				fn from((name, ov): (&str, $OV)) -> Self {
					let opvals = vec![ov.into()];
					FilterNode::new(name, opvals)
				}
			}

			/// From `trait from (prop_name, Vec<OpValType>)`  for FilterNode
			/// (e.g., `let node: FilterNode = (prop_name, Vec<OpValType>).into()`)
			impl From<(&str, Vec<$OV>)> for FilterNode {
				fn from((name, ovs): (&str, Vec<$OV>)) -> Self {
					let opvals: Vec<OpVal> = ovs.into_iter().map(|v| OpVal::from(v)).collect();
					FilterNode::new(name, opvals)
				}
			}
		)+
	};
}
from_tuples_opval!(
	// String
	OpValString,
	// Nums
	// OpValUint64,
	// OpValUint32,
	OpValInt64,
	// OpValInt32,
	OpValFloat64,
	// OpValFloat32,
	// Bool
	OpValBool
);
// endregion: --- From Tuples (OpValType)

// region:    --- Froms Tuples (String val)

impl From<(&str, &str)> for FilterNode {
	fn from((name, ov): (&str, &str)) -> Self {
		let opvals = vec![OpValString::Eq(ov.to_string()).into()];
		FilterNode::new(name.to_string(), opvals)
	}
}

impl From<(&str, &String)> for FilterNode {
	fn from((name, ov): (&str, &String)) -> Self {
		let opvals = vec![OpValString::Eq(ov.to_string()).into()];
		FilterNode::new(name.to_string(), opvals)
	}
}

impl From<(&str, String)> for FilterNode {
	fn from((name, ov): (&str, String)) -> Self {
		let opvals = vec![OpValString::Eq(ov).into()];
		FilterNode::new(name.to_string(), opvals)
	}
}

// endregion: --- Froms Tuples (String val)

// region:    --- From Tuples (num val)
// - `nt` e.g., `u64`
// - `ov` e.g., `OpValUint64`
macro_rules! from_tuples_num{
	($(($nt:ty, $ov:ident)),+) => {
		$(

impl From<(&str, $nt)> for FilterNode {
	fn from((name, ov): (&str, $nt)) -> Self {
		let opvals = vec![$ov::Eq(ov).into()];
		FilterNode::new(name.to_string(), opvals)
	}
}
		)+
	};
}

from_tuples_num!(
	// (u64, OpValUint64),
	// (u32, OpValUint32),
	(i64, OpValInt64),
	(i32, OpValInt32),
	// (f32, OpValFloat32),
	(f64, OpValFloat64)
);

// endregion: --- From Tuples (num val)

// region:    --- From Tuples (bool val)
impl From<(&str, bool)> for FilterNode {
	fn from((name, ov): (&str, bool)) -> Self {
		let opvals = vec![OpValBool::Eq(ov).into()];
		FilterNode::new(name.to_string(), opvals)
	}
}

// endregion: --- From Tuples (bool val)

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use crate::filter::{ForSeaCondition, IntoSeaError, OpValValue, SeaResult};
	use crate::sea_utils::StringIden;
	use sea_query::{ColumnRef, ConditionExpression, IntoColumnRef, IntoIden};

	impl FilterNode {
		pub fn into_sea_cond_expr_list(self) -> SeaResult<Vec<ConditionExpression>> {
			let col: ColumnRef = match self.rel {
				Some(rel) => ColumnRef::TableColumn(StringIden(rel).into_iden(), StringIden(self.name).into_iden()),
				None => StringIden(self.name).into_column_ref(),
			};
			let mut node_sea_exprs: Vec<ConditionExpression> = Vec::new();
			let for_sea_cond = self.for_sea_condition;
			let node_options = &self.options;

			for op_val in self.opvals.into_iter() {
				let cond_expr = match op_val {
					OpVal::String(ov) => ov.into_sea_cond_expr(&col, node_options)?,
					OpVal::Int64(ov) => ov.into_sea_cond_expr(&col, node_options)?,
					OpVal::Int32(ov) => ov.into_sea_cond_expr(&col, node_options)?,
					OpVal::Float64(ov) => ov.into_sea_cond_expr(&col, node_options)?,
					OpVal::Bool(ov) => ov.into_sea_cond_expr(&col, node_options)?,
					OpVal::Value(ov) => {
						let Some(for_sea_cond) = for_sea_cond.as_ref() else {
							return Err(IntoSeaError::Custom(
								"OpValsValue must have a #[modql(to_sea_value_fn=\"fn_name\"] or to_sea_condition_fn attribute"
									.to_string(),
							));
						};

						match for_sea_cond {
							ForSeaCondition::ToSeaValue(to_sea_value) => {
								OpValValue::into_sea_cond_expr_with_json_to_sea(ov, &col, node_options, to_sea_value)?
							}
							ForSeaCondition::ToSeaCondition(to_sea_condition) => to_sea_condition.call(&col, ov)?,
						}
					}
				};

				node_sea_exprs.push(cond_expr);
			}

			Ok(node_sea_exprs)
		}
	}
}
// endregion: --- with-sea-query
