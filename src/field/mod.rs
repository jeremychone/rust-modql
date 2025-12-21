//! Requires feature `with-sea-query` and provides convenient sea-query serialization for field names and values.

mod error;
mod field_meta;
mod field_metas;
mod has_fields;
#[cfg(feature = "with-sea-query")]
mod sea;
#[cfg(feature = "with-rusqlite")]
mod sqlite;

pub use self::error::{Error, Result};
pub use field_meta::*;
pub use field_metas::*;
pub use has_fields::*;

#[cfg(feature = "modql-macros")]
pub use modql_macros::Fields;

#[cfg(all(feature = "modql-macros", feature = "with-sea-query"))]
pub use modql_macros::SeaFieldValue;

#[cfg(feature = "with-sea-query")]
pub use sea::*;

#[cfg(feature = "with-rusqlite")]
pub use sqlite::*;
