use crate::field::SeaField;
use sea_query::{DynIden, SimpleExpr};

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
	/// Simple api to append a SeaField to the list.
	pub fn push(&mut self, field: SeaField) {
		self.0.push(field);
	}

	/// The consuming builder API equivalent to `push(..)`
	pub fn append(mut self, field: SeaField) -> Self {
		self.push(field);
		self
	}

	/// The static 'str for iden version of the `append(..)`
	pub fn append_siden(mut self, iden: &'static str, value: impl Into<SimpleExpr>) -> Self {
		let field = SeaField::siden(iden, value);
		self.push(field);
		self
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

// region:    --- Froms

impl From<Vec<SeaField>> for SeaFields {
	fn from(val: Vec<SeaField>) -> Self {
		SeaFields(val)
	}
}

impl From<SeaField> for SeaFields {
	fn from(val: SeaField) -> Self {
		SeaFields(vec![val])
	}
}

// endregion: --- Froms
