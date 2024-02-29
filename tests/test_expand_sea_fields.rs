#![cfg(feature = "with-sea-query")]

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
use modql::field::{Fields, HasSeaFields};

#[derive(Debug, Default, Fields)]
pub struct Todo {
	pub id: i64,

	#[field(rel = "special_todo_table", name = "special_title_col")]
	pub title: String,

	#[field(name = "description")]
	pub desc: Option<String>,

	#[field(skip)]
	pub other: Option<String>,
}

#[test]
fn test_struct_field_names() -> Result<()> {
	// -- Exec
	let sea_idens = Todo::sea_idens();

	// -- Check
	let names = sea_idens.iter().map(|i| i.to_string()).collect::<Vec<_>>();
	let names = names.iter().map(|s| s.as_str()).collect::<Vec<_>>();
	assert_eq!(names, &["id", "special_title_col", "description"]);

	Ok(())
}
