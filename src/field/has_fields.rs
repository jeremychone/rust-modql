// region:    --- HasFields

#[derive(Debug)]
pub struct FieldRef {
	pub rel: Option<&'static str>,
	pub name: &'static str,
}

pub trait HasFields {
	/// Returns the array of all field names (can be customize with `#[field(rel=..., name=...], #[field(ignore)]`)
	fn field_names() -> &'static [&'static str];

	fn field_refs() -> &'static [&'static FieldRef];
}
// endregion: --- HasFields
