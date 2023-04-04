//! Main Crate Error

use serde_json::Value;

#[derive(Debug)]
pub enum Error {
	// region:    --- Json Errors
	JsonValNotOfType(&'static str),

	JsonOpValNotSupported(String, Value),
	// endregion: --- Json Errors
}

impl std::fmt::Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
