pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
use modql::field::{Fields, HasFields};

#[derive(Debug, Default, Fields)]
#[modql(rel = "todo_table")]
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
	assert_eq!(Todo::field_names(), &["id", "special_title_col", "description"]);
	Ok(())
}

#[test]
fn test_struct_field_metas() -> Result<()> {
	// -- Exec
	let field_refs = Todo::field_metas();

	// -- Check
	let names: Vec<&'static str> = field_refs.iter().map(|&meta| meta.name()).collect();
	let rels: Vec<Option<&'static str>> = field_refs.iter().map(|fr| fr.rel).collect();
	assert_eq!(names, &["id", "special_title_col", "description"]);
	assert_eq!(
		rels,
		&[Some("todo_table"), Some("special_todo_table"), Some("todo_table")]
	);

	Ok(())
}
