pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.
use modql::field::Fields;

#[derive(Debug, Default, Fields)]
#[modql(rel = "todo_table", names_as_consts)]
pub struct Todo {
	pub id: i64,

	#[field(rel = "special_todo_table", name = "special_title_col")]
	pub title: String,

	#[field(name = "description")]
	pub desc: Option<String>,

	#[field(skip)]
	pub other: Option<String>,
}

#[derive(Debug, Default, Fields)]
#[modql(names_as_consts = "COL_")]
pub struct Project {
	pub id: i64,

	#[field(name = "pname")]
	pub name: Option<String>,
}

#[derive(Debug, Default, Fields)]
#[modql(names_as_consts = "COL")]
pub struct Label {
	pub id: i64,

	pub name: Option<String>,
}

#[test]
fn test_struct_const_names_no_prefix() -> Result<()> {
	assert_eq!(Todo::ID, "id");
	assert_eq!(Todo::TITLE, "special_title_col");

	Ok(())
}

#[test]
fn test_struct_const_names_full_prefix() -> Result<()> {
	assert_eq!(Project::COL_ID, "id");
	assert_eq!(Project::COL_NAME, "pname");

	Ok(())
}

#[test]
fn test_struct_const_names_simple_prefix() -> Result<()> {
	assert_eq!(Label::COL_ID, "id");
	assert_eq!(Label::COL_NAME, "name");

	Ok(())
}
