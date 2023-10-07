
`.` minor | `+` Addition | `^` improvement | `!` Change | `*` Refactor


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