use crate::field::FieldMetas;

pub trait HasFields {
	/// Returns the array of all field names (can be customize with `#[field(rel=..., name=...], #[field(ignore)]`)
	fn field_names() -> &'static [&'static str];

	#[allow(deprecated)]
	#[deprecated(note = "use field_metas")]
	fn field_refs() -> &'static [&'static FieldRef];

	fn field_metas() -> &'static FieldMetas;
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
