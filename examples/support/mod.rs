pub mod rusqlite_utils;
pub mod sqlx_utils;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
