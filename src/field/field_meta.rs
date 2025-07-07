#[derive(Debug, Clone)]
pub struct FieldMeta {
	/// rel from either the struct `#[modql(rel=...)]` or from property `#[field(rel=...)]`
	pub rel: Option<&'static str>,

	/// If the rel is a struct rel or the field rel matches the struct rel
	pub is_struct_rel: bool,

	/// Name of the struct property no matter what.
	pub prop_name: &'static str,

	/// The attribute name (i.e., `#[field(name=...)]`)
	pub attr_name: Option<&'static str>,

	/// `#[field(cast_as=...)`
	pub cast_as: Option<&'static str>,

	/// `#[field(write_placeholder=...)`
	pub write_placeholder: Option<&'static str>,

	/// if it is an Option type
	pub is_option: bool,
}

impl FieldMeta {
	pub fn name(&self) -> &'static str {
		self.attr_name.unwrap_or(self.prop_name)
	}

	/// return the alias name if there should be one
	/// (if prop_name != name, it will return prop_name)
	pub fn alias(&self) -> Option<&'static str> {
		let attr_name = self.attr_name?;
		if self.prop_name != attr_name {
			Some(self.prop_name)
		} else {
			None
		}
	}

	/// Return the quote column ref with the eventual ref and eventual
	/// alias `AS "prop_name"` if prop_name does not match a provide attr_name (i.e. #[field(name=...)])
	/// e.g., `"conv"."desc" AS "description"`
	pub fn sql_col_ref(&self) -> String {
		let mut col_ref = if let Some(rel) = self.rel {
			format!("\"{}\".\"{}\"", rel, self.name())
		} else {
			format!("\"{}\"", self.name())
		};

		if let Some(alias_name) = self.alias() {
			col_ref.push_str(&format!(" AS \"{alias_name}\""));
		}
		col_ref
	}
}

// when with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use crate::SIden;
	use sea_query::{Alias, ColumnRef, IntoIden, SelectStatement};

	impl FieldMeta {
		pub fn sea_column_ref(&self) -> ColumnRef {
			match self.rel {
				Some(rel) => ColumnRef::TableColumn(SIden(rel).into_iden(), SIden(self.name()).into_iden()),
				None => ColumnRef::Column(SIden(self.name()).into_iden()),
			}
		}

		pub fn sea_apply_select_column(&self, sea_select: &mut SelectStatement) {
			let col_ref = self.sea_column_ref();

			if let Some(alias_name) = self.alias() {
				sea_select.expr_as(col_ref, Alias::new(alias_name));
			} else {
				sea_select.column(col_ref);
			}
		}
	}
}
