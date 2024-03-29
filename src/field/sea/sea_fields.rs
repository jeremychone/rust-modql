use crate::field::SeaField;
use sea_query::{DynIden, SimpleExpr};

// region:    --- Fields
#[derive(Debug, Clone)]
pub struct SeaFields(Vec<SeaField>);

// Constructor
impl SeaFields {
	pub fn new(fields: Vec<SeaField>) -> Self {
		SeaFields(fields)
	}
}

// Api
impl SeaFields {
	pub fn push(&mut self, field: SeaField) {
		self.0.push(field);
	}

	pub fn into_vec(self) -> Vec<SeaField> {
		self.0
	}

	/// Alias to self.unzip()
	pub fn for_sea_insert(self) -> (Vec<DynIden>, Vec<SimpleExpr>) {
		self.unzip()
	}
	/// returns a tuble: (Vec_of_column_idens, Vec_of_value_exprs)
	pub fn unzip(self) -> (Vec<DynIden>, Vec<SimpleExpr>) {
		self.0.into_iter().map(|f| (f.iden, f.value)).unzip()
	}

	/// Alias to self.zip()
	pub fn for_sea_update(self) -> impl Iterator<Item = (DynIden, SimpleExpr)> {
		self.zip()
	}
	/// returns Iterator of (column_iden, value_expr)
	/// Useful for sea query update.
	pub fn zip(self) -> impl Iterator<Item = (DynIden, SimpleExpr)> {
		self.0.into_iter().map(|f| (f.iden, f.value))
	}
}

impl IntoIterator for SeaFields {
	type Item = SeaField;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

// endregion: --- Fields
