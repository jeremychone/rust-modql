**STATUS** The current main branch aligns with the `v0.3.0-alpha` stream, featuring `with-sea-query` support and new `Fields` `#[derive(Fields)]` capabilities.

> _tl;dr_ - **modql** is a normalized, declarative model and store-agnostic query language.

[changelog](CHANGELOG.md)

### Overview 

One representation of modql is [joql](http://joql.org), which is the JSON serialization of this model.

`modql` has a [joql](http://joql.org) deserializer to build filters and offers the raw constructs to create them directly from Rust types.

In short, `modql` allows expression of store model-specific filters and include rules that can be implemented for various stores. For instance, it can express:

- Filter & include all of the `Ticket`:
	- with the `title` containing either `"hello"` or `"welcome"` (<< Filters)
	- with the `done` flag set to `true`
	- fetching only the properties: `id`, `title`, and `done`. (<< Includes, not yet implemented)

### Overview

- `modql::filter` - Provides a declarative structure that can be deserialized from JSON.
- `modql::field` - Offers a way to extract a `sea-query` compatible data structure from standard structs.

NOTE: For now, ensure you activate the `with-sea-query` feature.


```rust
#[derive(Debug, Clone, modql::field::Fields, FromRow, Serialize)]
pub struct Task {
	pub id: i64,
	pub project_id: i64,
	pub title: String,
	pub done: bool,
}

#[derive(modql::filter::FilterNodes, Deserialize, Default)]
pub struct TaskFilter {
	project_id: Option<OpValsInt64>,
	title: Option<OpValsString>,
	done: Option<OpValsBool>,
}

// ...
let filter_json = json! ({
	"title": {"$contains": "title_contains_ok 02"},
	"done": false
});
let filter: TaskFilters = serde_json::from_value(filter_json)?;
```

This will provide the following: 

- `Task::field_column_refs()  -> Vec<ColumnRef>` to build `sea-query` select queries. 
- `Task::field_idens() -> Vec<ColumnRef>`  to build `sea-query` select queries (for simpler case);
- `task.all_fields().unzip() - (Vec<DynIden>, Vec<SimpleExpr>)` for `sea-query` insert.
- `task.all_fields().zip() - impl Iterator<Item = (DynIden, SimpleExpr)>` for `sea-query` update.

Will also provide: 

- `task.not_none_fields()` for same as above, but only the fields for which their Option is not None. 

### Json example

The JSON/joql representation of this would be something like: 
```ts
{
	$filters: {
		"title": {$containsIn: ["hello", "welcome"]},
		"done": true, // equivalent: "done": {$eq: true}
	},
	$includes: { // not implemented yet
		"id": true,
		"title": true,
		"done": true,
	}
}
```

### Rust types

On the Rust side, this can be expressed like this:

```rs
use modql::filter::{FilterGroups, FilterNode, OpValString};

fn main() -> anyhow::Result<()> {
	println!("->> hello! {}", 111);

	let filter_nodes: Vec<FilterNode> = vec![
		(
			"title",
			OpValString::ContainsIn(vec!["Hello".to_string(), "welcome".to_string()]),
		)
			.into(),
		("done", true).into(),
	];
	let filter_groups: FilterGroups = filter_nodes.into();

	println!("filter_groups:\n{filter_groups:#?}");

	Ok(())
}
```

Then, a Model or Store layer can get the filter_groups, and serialize to their DSL (i.e. SQL for database)

The Filter structure is as followed: 

- `FilterGroups` is the top level and is a group of `FilterGroup`. `FilterGroup` are intended to be executed as `OR` between them. 
- `FilterGroup` contains a vector of `FilterNode` that are intended to be executed as `AND`
- `FilterNode` contains a `context_path` (not used yet), `name` which is the property name that the value will come from, and a `Vec<OpVal>`, which is the Operator Value. 
- `OpVal` is an enum for type specific `OpVal[Type]` like `OpValString` that contains the specific operation for this type with the associated pattern value. 

### FilterNodes macro

A convenient `FilterNodes` implements the various functions to get the `into` `FilterNode` `FilterGroups`

```rs
use modql::filter::{FilterGroups, FilterNodes, OpValBool, OpValString, OpValsBool, OpValsString};

#[derive(FilterNodes)]
struct MyFilter {
	done: Option<OpValsBool>,
	name: Option<OpValsString>,
}

let filter = MyFilter {
	done: Some(OpValBool::Eq(true).into()),
	name: Some(
		vec![
			OpValString::Contains("Hello".to_string()),
			OpValString::Contains("welcome".to_string()),
		]
		.into(),
	),
};

let filter_groups: FilterGroups = filter.into();

println!("filter_groups:\n{filter_groups:#?}");
```

- And the side to implement the filter against a specific data interface (e.g., sql, file system, S3, ...), 
which uses the `FilterNode`, `OrGroups`, `OpVal` (which is a enum containing `StringOpVal` or `InOpval`)