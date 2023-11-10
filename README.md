# modql

**modql** is a set of types and utilities designed to structurally express model query filters and list options (e.g., `offset`, `limit`, `order_bys`). These can be easily represented in JSON.

In essence, it offers a MongoDB-like filter syntax that is storage-agnostic, has built-in support for [sea-query](https://crates.io/crates/sea-query), and can be expressed either in JSON or Rust types.

For example:

```rs
/// This is the model entity, annotated with Fields.
#[derive(Debug, Clone, modql::field::Fields, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,

    pub title: String,
    pub done: bool,
}

/// This is a Filter, with the modql::filter::OpVals... properties
#[derive(modql::filter::FilterNodes, Deserialize, Default, Debug)]
pub struct TaskFilter {
    project_id: Option<OpValsInt64>,
    title: Option<OpValsString>,
    done: Option<OpValsBool>,
}

// -- Parsing JSON representation to TaskFilter
// This condition requires all of these rules to match (AND).
let list_filter: TaskFilter = serde_json::from_value(json! ({
    "project_id": 123,
    "title": {"$startsWith": "Hello", "$contains": "World"} ,
}))?;

// -- modql ListOptions
let list_options: modql::filter::ListOptions =
    serde_json::from_value(json! ({
        "offset": 0,
        "limit": 2,
        "order_bys": "!title" // ! for descending
    }))?;
    
// -- Building a sea-query select query with those condition
// Convert the TaskFilter into sea-query condition
let cond: sea_query::Condition = filter.try_into()?;
let mut query = sea_query::Query::select();

// Select only the columns corresponding to the task type.
// This is determined by the modql::field::Fields annotation.
query.from(task_table).columns(Task::field_column_refs());

// Add the condition from the filter
query.cond_where(cond);

// Apply the list options
list_options.apply_to_sea_query(&mut query);

// and execute query
let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
let entities = sqlx::query_as_with::<_, E, _>(&sql, values)
    .fetch_all(db)
    .await?;
```

This crate is instrumental for JSON-RPC or other types of model APIs (e.g., the [joql pattern](https://joql.org)).

**IMPORTANT** v0.3.x represents the new version of modql, featuring the `with-sea-query` feature set. It is utilized in the [rust10x web-app production code blueprint Episode 02](https://rust10x.com/web-app). 
This version is somewhat incompatible with v0.2.x, mainly due to module reorganization. If you are using the rust10x/awesomeapp desktop app, please stick with v0.2.x for the time being. I plan to upgrade the codebase to v0.3.x soon.

[changelog](CHANGELOG.md)


## `OpVal[Type]` Conditional Operators

`OpVal[Type]` is a filter unit that allows the expression of an operator on a given value for a specified type.

The corresponding `OpVals[Type]`, with an "s", is typically used in filter properties, as it permits multiple operators for the same field.

The basic JSON representation of an `OpVal[Type]` follows the `{field_name: {$operator1: value1, $operator2: value2}}` format. For example:

```js
{
    "title": {"$startsWith": "Hello", "$contains": "World"} 
}
```

This expresses the conditions that both "startsWith" and "contains" must be met.

The following tables show the list of possible operators for each type.

### `OpValString` Operators

| Operator            | Meaning                                         | Example                                                  |
|---------------------|-------------------------------------------------|----------------------------------------------------------|
| `$eq`               | Exact match with one value                      | `{name: {"$eq": "Jon Doe"}}` same as `{name: "Jon Doe"}` |
| `$in`               | Exact match with within a list of values (or)   | `{name: {"$in": ["Alice", "Jon Doe"]}}`                  |
| `$not`              | Exclude any exact match                         | `{name: {"$not": "Jon Doe"}}`                            |
| `$notIn`            | Exclude any exact withing a list                | `{name: {"$notIn": ["Jon Doe"]}}`                        |
| `$contains`         | For string, does a contains                     | `{name: {"$contains": "Doe"}}`                           |
| `$containsAny`      | For string, match if contained in any of items  | `{name: {"$containsAny": ["Doe", "Ali"]}}`               |
| `$containsAll`      | For string, match if all items are in the src   | `{name: {"$containsAll": ["Hello", "World"]}}`           |
| `$notContains`      | Does not contain                                | `{name: {"$notContains": "Doe"}}`                        |
| `$notContainsAny`   | Does not call any of (none is contained)        | `{name: {"$notContainsAny": ["Doe", "Ali"]}}`            |
| `$startsWith`       | For string, does a startsWith                   | `{name: {"$startsWith": "Jon"}}`                         |
| `$startsWithAny`    | For string, match if startsWith in any of items | `{name: {"$startsWithAny": ["Jon", "Al"]}}`              |
| `$notStartsWith`    | Does not start with                             | `{name: {"$notStartsWith": "Jon"}}`                      |
| `$notStartsWithAny` | Does not start with any of the items            | `{name: {"$notStartsWithAny": ["Jon", "Al"]}}`           |
| `$endsWith`         | For string, does and end with                   | `{name: {"$endsWithAny": "Doe"}}`                        |
| `$endsWithAny`      | For string, does a contains  (or)               | `{name: {"$endsWithAny": ["Doe", "ice"]}}`               |
| `$notEndsWith`      | Does not end with                               | `{name: {"$notEndsWithAny": "Doe"}}`                     |
| `$notEndsWithAny`   | Does not end with any of the items              | `{name: {"$notEndsWithAny": ["Doe", "ice"]}}`            |
| `$lt`               | Lesser Than                                     | `{name: {"$lt": "C"}}`                                   |
| `$lte`              | Lesser Than or =                                | `{name: {"$lte": "C"}}`                                  |
| `$gt`               | Greater Than                                    | `{name: {"$gt": "J"}}`                                   |
| `$gte`              | Greater Than or =                               | `{name: {"$gte": "J"}}`                                  |
| `$null`             | If the value is null                            | `{name: {"$null": true}}`                                |

### `OpValInt32, OpValInt64, OpValFloat64`  Operators

| Operator | Meaning                                       | Example                                  |
|----------|-----------------------------------------------|------------------------------------------|
| `$eq`    | Exact match with one value                    | `{age: {"$eq": 24}}` same as `{age: 24}` |
| `$in`    | Exact match with within a list of values (or) | `{age: {"$in": [23, 24]}}`               |
| `$not`   | Exclude any exact match                       | `{age: {"$not": 24}}`                    |
| `$notIn` | Exclude any exact withing a list              | `{age: {"$notIn": [24]}}`                |
| `$lt`    | Lesser Than                                   | `{age: {"$lt": 30}}`                     |
| `$lte`   | Lesser Than or =                              | `{age: {"$lte": 30}}`                    |
| `$gt`    | Greater Than                                  | `{age: {"$gt": 30}}`                     |
| `$gte`   | Greater Than or =                             | `{age: {"$gte": 30}}`                    |
| `$null`  | If the value is null                          | `{name: {"$null": true}}`                |

### `OpValBool` Operators

| Operator | Meaning                    | Example                                      |
|----------|----------------------------|----------------------------------------------|
| `$eq`    | Exact match with one value | `{dev: {"$eq": true}}` same as `{dev: true}` |
| `$not`   | Exclude any exact match    | `{dev: {"$not": false}}`                     |
| `$null`  | If the value is null       | `{name: {"$null": true}}`                    |


## More Info

- `modql::filter` - Delivers a declarative structure that can be deserialized from JSON.
- `modql::field` - Provides a method to derive a `sea-query` compatible data structure from standard structs.

This introduces the following:

- `Task::field_column_refs() -> Vec<ColumnRef>`: Constructs `sea-query` select queries.
- `Task::field_idens() -> Vec<ColumnRef>`: Constructs `sea-query` select queries, suited for simpler cases.
- `task.all_fields().for_sea_insert() -> (Vec<DynIden>, Vec<SimpleExpr>)`: Used for `sea-query` inserts.
- `task.all_fields().for_sea_update() -> impl Iterator<Item = (DynIden, SimpleExpr)>`: Used for `sea-query` updates.

Additionally, it offers:

- `task.not_none_fields()`: Operates similarly to the above, but only for fields where their Option is not `None`.


### Rust types

On the Rust side, this can be expressed like this:

```rs
use modql::filter::{FilterGroups, FilterNode, OpValtring};

fn main() -> anyhow::Result<()> {
    let filter_nodes: Vec<FilterNode> = vec![
        (
            "title",
            OpValtring::ContainsAny(vec!["Hello".to_string(), "welcome".to_string()]),
        )
            .into(),
        ("done", true).into(),
    ];
    let filter_groups: FilterGroups = filter_nodes.into();

    println!("filter_groups:\n{filter_groups:#?}");

    Ok(())
}
```


A Model or Store layer can take the `filter_groups` and serialize them into their DSL (e.g., SQL for databases).

The Filter structure is as follows:

- `FilterGroups` is the top level and consists of multiple `FilterGroup` elements. `FilterGroup` elements are intended to be executed with an `OR` operation between them.
- Each `FilterGroup` contains a vector of `FilterNode` elements, which are intended to be executed with an `AND` operation.
- `FilterNode` contains a `context_path` (not used yet), `name` which represents the property name from where the value originates, and a `Vec<OpVal>`, representing the Operator Value.
- `OpVal` is an enum for type-specific `OpVal[Type]` entities, such as `OpValString` that holds the specific operation for that type along with the associated pattern value.