pub type SeaResult<T> = core::result::Result<T, IntoSeaError>;

/// Error for FilterNode to Sea Condition
#[derive(Debug)]
pub enum IntoSeaError {
	// For now, just Custom. Might have more variants later.
	Custom(String),
	SerdeJson(serde_json::Error),
}

// region:    --- Froms
impl From<serde_json::Error> for IntoSeaError {
	fn from(val: serde_json::Error) -> Self {
		Self::SerdeJson(val)
	}
}
// endregion: --- Froms

impl IntoSeaError {
	pub fn custom(message: impl Into<String>) -> Self {
		IntoSeaError::Custom(message.into())
	}
}

// region:    --- Error Boilerplate
impl core::fmt::Display for IntoSeaError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for IntoSeaError {}
// endregion: --- Error Boilerplate
