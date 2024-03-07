//! Requires feature `with-sea-query` and provides convenient sea-query serialization for field names and values.

mod error;
mod has_fields;
#[cfg(feature = "with-sea-query")]
mod sea;

pub use self::error::{Error, Result};
pub use has_fields::*;
pub use modql_macros::Fields;

#[cfg(feature = "with-sea-query")]
pub use modql_macros::SeaFieldValue;

#[cfg(feature = "with-sea-query")]
pub use sea::*;
