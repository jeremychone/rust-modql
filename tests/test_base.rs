#![allow(unused)] // silence unused warnings while exploring (to comment out)

use modql::{IncludeNode, IncludeValue, Includes};

#[test]
fn test_1() {
	let includes_data = IncludeValue::Nodes(vec![("test_name", true).into()]);

	let i = Includes::new(includes_data);
	println!("Hello -- {i:?}")
}
