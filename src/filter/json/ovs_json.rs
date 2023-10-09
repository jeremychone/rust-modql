use crate::Result;
use serde_json::Value;

pub trait OpValueToOpValType {
	/// e.g., `{"$contains": "World", "$startsWith": "Hello"}
	fn op_value_to_op_val_type(op: &str, value: Value) -> Result<Self>
	where
		Self: Sized;
}
