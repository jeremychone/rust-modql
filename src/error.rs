//! Main Crate Error

use serde_json::Value;

/// modql Result
pub type Result<T> = core::result::Result<T, Error>;

/// modql Error
#[derive(Debug)]
pub enum Error {
	// region:    --- Json Errors
	JsonValNotOfType(&'static str),

	JsonValArrayWrongType {
		actual_value: Value,
	},
	JsonValArrayItemNotOfType {
		expected_type: &'static str,
		actual_value: Value,
	},

	JsonOpValNotSupported {
		operator: String,
		value: Value,
	},
	// endregion: --- Json Errors
}

// region:    --- Error Boilerpate
impl std::fmt::Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerpate
