use crate::filter::FilterNodeOptions;
use sea_query::{ColumnName, ColumnRef, ExprTrait, Iden, IdenStatic, SimpleExpr, Value};

/// String sea-query `Iden` wrapper
#[derive(Debug)]
pub struct StringIden(pub String);

impl Iden for StringIden {
	fn unquoted(&self) -> &str {
		&self.0
	}
}

impl StringIden {
	pub fn into_column_ref(self) -> ColumnRef {
		ColumnRef::Column(self.0.into())
	}

	pub fn to_column_ref(&self) -> ColumnRef {
		ColumnRef::Column(self.0.clone().into())
	}
}

/// Static str sea-query `Iden` wrapper
#[derive(Debug, Clone, Copy)]
pub struct SIden(pub &'static str);

impl Iden for SIden {
	fn unquoted(&self) -> &str {
		&self.0
	}
}

impl IdenStatic for SIden {
	fn as_str(&self) -> &'static str {
		self.0
	}
}

impl SIden {
	pub fn to_column_ref(&self) -> ColumnRef {
		ColumnRef::Column(self.0.into())
	}
}

/// Convert a FilterNode value T into a sea-query SimpleExpr as long as T implements Into<sea_query::Value>
pub fn into_node_value_expr<T>(val: T, node_options: &FilterNodeOptions) -> SimpleExpr
where
	T: Into<Value>,
{
	let mut vxpr = SimpleExpr::Value(val.into());
	if let Some(cast_as) = node_options.cast_as.as_ref() {
		vxpr = vxpr.cast_as(StringIden(cast_as.into()));
	}
	vxpr
}

pub fn into_node_column_expr(col: ColumnRef, node_options: &FilterNodeOptions) -> SimpleExpr {
	let Some(cast_column_as) = &node_options.cast_column_as else {
		// If no cast is needed, wrap the ColumnRef as a SimpleExpr
		return SimpleExpr::Column(col);
	};

	SimpleExpr::Column(col).cast_as(StringIden(cast_column_as.into()))
}
