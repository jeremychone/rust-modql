pub type SeaResult<T> = core::result::Result<T, SeaError>;

#[derive(Debug)]
pub enum SeaError {
	// For now, just Custom. Might have more variants later.
	Custom(String),
	SerdeJson(serde_json::Error),
}

// region:    --- Froms
impl From<serde_json::Error> for SeaError {
	fn from(val: serde_json::Error) -> Self {
		Self::SerdeJson(val)
	}
}
// endregion: --- Froms

impl SeaError {
	pub fn custom(message: impl Into<String>) -> Self {
		SeaError::Custom(message.into())
	}
}

// region:    --- Error Boilerplate
impl core::fmt::Display for SeaError {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for SeaError {}
// endregion: --- Error Boilerplate
