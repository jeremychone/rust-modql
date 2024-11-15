
`.` minor | `+` Addition | `^` improvement | `!` Change | `*` Refactor

## 2024-11-15 - `0.4.1-WIP`

- `^` Update `sea-query` and `rustsqlite` to version `0.32`
- `!` Remove `cast_column_as` from filter, it's now on field

## 2024-09-23 - `0.4.0`

- `.` update to sea-query-rusqlite 0.6
- `.` add rustfmt.toml
- `^` Update sea-query version `0.31`
- `+` Add CaseInsensitive for StringOpVals (`StartsWithCt` .. )
- `+` Add ILIKE for postgres (case-insensitive LIKE)

## 2024-06-26 - `0.4.0-rc.8`

- `.` update version to 0.4.0-rc.8 (with sea-query 31.0-rc.9)
- `.` minor code clean

## 2024-06-13 - `0.4.0-rc.7`

- `+` `T::field_metas()`, `FieldMetas`, and `FieldMeta`  when `#[derive(Fields)]`
- `!` `FieldRef` in favor of `FieldMeta`
- `+` `SqliteFromRow::sqlite_from_row_partial(row, prop_names)` to retrieve partial objects.
- `!` Sqlite types & derives rename
	- Traits:
		- Now: `SqliteFromRow`, before: `FromSqliteRow`
			- Now: `fn sqlite_from_row`, before: `fn from_sqlite_row...`
	- derive:
		- Now: `SqliteFromValue`, before: `FromSqliteRow`
		- Now: `SqliteFromValue`, before: `FromSqliteValue`
		- Now: `ToSqliteValue`, before: `SqliteToValue`

## 2024-05-09 - `0.4.0-rc.6 & rc.5`

- `^` filter - add support for `#[modql(rel=...)]` at the Filter struct level
- `.` cleanup
- `-` filter - fix rel missing from FilterNode to SeaCondExpr
- `.` update to v0.4.0-rc.5
- `^` sea-query - impl IdenStatic for SIden (and SIden: Clone + Copy)

## 2024-04-18 - `0.4.0-rc.4`

- SEE: Major refactor/cleanup (see [v0.3.x to v0.4.x document](MIGRATION-v03x-v04x.md)
- `+` ToSqliteValue - added ToSqliteValue for simple enum and single tuple struct
- `!` SeaField::new takes an Into<SimpleExpr> as second arg now
- `^` SeaField added From<SeaField> for SeaFields, and simple From (static str, SimpleExpr) for SeaField
- `^` SeaField added ::siden(..) for static str column name
- `^` SeaFields - add append and append_siden
- `!` filter - rename context_path to rel
- `^` SeaField - add new_concrete

## 2024-03-07 - `0.4.0-rc.2`

- `!` Major refactor/cleanup (see [v0.3.x to v0.4.x document](MIGRATION-v03x-v04x.md)

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
