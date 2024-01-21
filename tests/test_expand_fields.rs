#![cfg(feature = "with-sea-query")]

use modql::field::Fields;

#[derive(Debug, Default, Fields)]
pub struct Todo {
	pub id: i64,

	#[field(table = "special_todo_table", column = "special_title_column")]
	pub title: String,

	#[field(name = "description")]
	pub desc: Option<String>,

	#[field(skip)]
	pub other: Option<String>,
}
