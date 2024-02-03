//! Requires feature `with-sea-query` and provides convenient sea-query serialization for field names and values.

mod error;
#[allow(clippy::module_inception)] // ok, because internal and flatten below.
mod field;
mod fields;
mod has_fields;

pub use self::error::{Error, Result};
pub use field::*;
pub use fields::*;
pub use has_fields::*;
pub use modql_macros::FieldEnum;
pub use modql_macros::Fields;
