
`.` minor | `+` Addition | `^` improvement | `!` Change | `*` Refactor

## 2024-02-21 - `0.3.10`

- `+` Add HasField::field_column_refs_with_rel
- `+` Derive FromSqliteValue - Add support for simple tuple struct
- `+` Derive Field - Add simple tuple struct support
- `!` Deprecate (warning) `FieldEnum` in favor of `FieldValue` (does enum and single tuple struct)

## 2024-02-04 - `0.3.9`

- `!` Rename FromSqliteRow::from_rusqlite_row to FromSqliteRow::from_sqlite_row)
- `!` Change sqlite::FromRow to FromSqliteRow
- `+` FromSqliteValue for enum 
- `+` Add `field::FieldEnum` derive to implement to seaqueryvalue for simple enum (also some code relayout)

## 2024-01-29 - `0.3.8`

- `^` sea-query - use `thread-safe` feature

## 2024-01-22 - `0.3.7`

- `+` `cast_as` to `filter`
- `!` Potential API break for user using `FieldNode` struct constructor (e.g., `FieldNode {...}`). New property `options: FieldNodeOptions`. Use `options: FieldNodeOptions::default()`. 
	- Using the `FieldNode::new(...)` functions and every other interface should be unchanged. 

## 2024-01-20 - `0.3.6`

- `+` Add `cast_as` to `field`

## 2024-01-13 - `0.3.5`

- `+` first pass at the `sqlite::FromRow` trait/macro for `rusqlite`
- `.` minor dependencies cleanup

## 2023-11-09 - `0.3.4`

- `+` Added `OpValString::ContainsAll`
- `!` For the OpValString, replace the `In` suffixes for `ContainsIn`, `StartsWithIn` with `Any` (e.g., `ContainsAny`)

## 2023-11-07 - `0.3.3`

- `-` with-sea-query - Fix "in" operator issues
- `-` fix OpValString containIn (was AND)

## 2023-11-06 - `0.3.2`

- `-` fix map opvals ( ..) support for numbers and bool

## 2023-11-05 - `0.3.1`

- `+` implements from Vec<F> for FilterGroups

## 2023-10-24 - `0.3.0`

- `!` - First v0.3.x release, with major update to API, with some breaking changes, and support for the `sea-query` and new `Fields` support.

## 2023-10-06 - `0.3.0-alpha.1`

- `!` - Major update to API, with some breaking changes, and support for the `sea-query` and new `Fields` support.

## 2023-04-15 - `0.2.0`

- `!` - Move `modql::filter::ListOptions` to `modql::filter::ListOptions`.
- `+` - Now primitive types `u64, u32, i64, i32, f64, f32`.
- `+` - Added many `From` traits.

## 2023-04-04 - `0.1.0`

- `!` - Major refactoring from `0.0.5`. 
- `!` - Moved from raw `Vec..` to specialized type `FilterGroups` and `FilterGroup`.
- `!` - Rename all of the `[Type]OpVal` to `OpVal[Type]` with full num type description. 
- `+` - Implemented lot of `From` traits.