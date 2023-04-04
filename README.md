**STATUS** Still experimental. In version 0.y.z, breaking changes will occur in .y, and additions/fixes in .z. Once it reaches x.y.z, it will follow SemVer best practices.

> _tl;dr_ - **modql** is a normalized declarative model and store agnostic query language.  

[changelog](CHANGELOG.md)

### Overview 

One modql representation is [joql](http://joql.org), which is the JSON serialization of this model. 

`modql` has the [joql](http://joql.org) deserializer to build the filter, and provides the raw construct to make them directly from Rust types. 

In short, `modql` allows to express of a store model-specific filter and include rules which can be implemented for various store. For example, it can express: 

- Filter & include all of the `Ticket` 
	- with the `title` containing `"hello"` or `"welcome"` (<< Filters)
	- with the `done` flag `true`
	- and fetch only the property `id`, `title` and the `done` properties. (<< Includes, not implemented yet)

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