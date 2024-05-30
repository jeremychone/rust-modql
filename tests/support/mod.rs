pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

#[cfg(feature = "with-rusqlite")]
pub mod sqlite;
