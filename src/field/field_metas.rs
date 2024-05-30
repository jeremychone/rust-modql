use crate::field::FieldMeta;

pub struct FieldMetas(&'static [&'static FieldMeta]);

impl FieldMetas {
	pub const fn new(metas: &'static [&'static FieldMeta]) -> Self {
		Self(metas)
	}

	pub fn iter(&self) -> core::slice::Iter<'_, &'static FieldMeta> {
		self.0.iter()
	}

	pub fn sql_col_refs(&self) -> String {
		let cols = self.0.iter().map(|meta| meta.sql_col_ref()).collect::<Vec<_>>();
		cols.join(", ")
	}

	pub fn sql_col_refs_for(&self, prop_names: &[&str]) -> String {
		let cols = self
			.0
			.iter()
			.filter(|m| prop_names.contains(&m.prop_name))
			.map(|meta| meta.sql_col_ref())
			.collect::<Vec<_>>();
		cols.join(", ")
	}
}
