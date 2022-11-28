//! Main Crate Error

use serde_json::Value;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	// region:    --- Json Errors
	#[error("Json Value is not of type '{0}'")]
	JsonValNotOfType(&'static str),

	#[error("Json OpVal not supported: {0}:{1:?}")]
	JsonOpValNotSupported(String, Value),
	// endregion: --- Json Errors
}
