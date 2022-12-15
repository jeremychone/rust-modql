use modql::{FilterNodes, IntOpVals, IntoFilterNodes, StringOpVals};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, FilterNodes)]
struct MyFilter {
	id: Option<IntOpVals>,
	name: Option<StringOpVals>,
}

#[test]
fn test_des_string_simple() -> anyhow::Result<()> {
	let json = r#"
	{
		"name": "Hello"
	}
	"#
	.to_string();

	let json: Value = serde_json::from_str(&json)?;
	let my_filter: MyFilter = serde_json::from_value(json)?;

	assert!(format!("{my_filter:?}").contains("id: None, name: Some(StringOpVals([Eq(\"Hello\")]))"));

	Ok(())
}

#[test]
fn test_des_string_map() -> anyhow::Result<()> {
	let json = r#"
{"name": {
	"$contains": "World",
	"$startsWith": "Hello"
}
}"#;

	let my_filter: MyFilter = serde_json::from_str(json)?;

	let mut nodes = my_filter.filter_nodes(None);

	assert_eq!(nodes.len(), 1, "number of filter node should be 1");
	let node = nodes.pop().unwrap();
	assert_eq!(format!("{:?}", node.opvals[0]), "String(Contains(\"World\"))");
	assert_eq!(format!("{:?}", node.opvals[1]), "String(StartsWith(\"Hello\"))");
	// assert_eq!(node.opvals[0])

	Ok(())
}

#[test]
fn test_des_number_simple() -> anyhow::Result<()> {
	let json = r#"
	{
		"id": 123
	}
	"#;

	let my_filter: MyFilter = serde_json::from_str(json)?;
	assert!(format!("{my_filter:?}").contains("{ id: Some(IntOpVals([Eq(123)])), name: None }"));

	Ok(())
}

#[test]
fn test_des_number_map() -> anyhow::Result<()> {
	let json = r#"
	{
		"id": {"$gt": 100}
	}
	"#;

	let my_filter: MyFilter = serde_json::from_str(json)?;
	assert!(format!("{my_filter:?}").contains("{ id: Some(IntOpVals([Gt(100)])), name: None }"));

	Ok(())
}
