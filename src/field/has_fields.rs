use crate::field::FieldMetas;

pub trait HasFields {
	/// Returns the array of all field names (can be customize with `#[field(rel=..., name=...], #[field(ignore)]`)
	fn field_names() -> &'static [&'static str];

	fn field_metas() -> &'static FieldMetas;

	// TODO: needs to use the Meta to get the eventual rel.name
	fn sql_columns() -> String {
		Self::field_names()
			.iter()
			.map(|name| format!("\"{name}\""))
			.collect::<Vec<_>>()
			.join(", ")
	}

	fn sql_placeholders() -> String {
		Self::field_names().iter().map(|_| "?").collect::<Vec<_>>().join(", ")
	}
}

/// To deprecate in favor of FieldMeta
#[deprecated(note = "use FieldMeta")]
#[derive(Debug)]
pub struct FieldRef {
	/// Eventual relation (e.g., table name)
	pub rel: Option<&'static str>,
	/// The name of the field (e.g., column name)
	pub name: &'static str,
}
