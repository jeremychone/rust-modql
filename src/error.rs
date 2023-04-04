//! Main Crate Error

use serde_json::Value;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// region:    --- Json Errors
	JsonValNotOfType(&'static str),

	JsonOpValNotSupported(String, Value),
	// endregion: --- Json Errors
}

// region:    --- Error Boiler
impl std::fmt::Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boiler
