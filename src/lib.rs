// #![allow(unused)]
// --- Sub-Modules
mod error;
pub mod filter;
pub mod includes;

// --- Re-Exports
pub use crate::error::{Error, Result};

#[cfg(feature = "with-sea-query")]
pub mod field;

#[cfg(feature = "with-sea-query")]
mod sea_utils;

#[cfg(feature = "with-sea-query")]
pub use sea_utils::SIden;

#[cfg(feature = "with-rusqlite")]
pub mod sqlite;
