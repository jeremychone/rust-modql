

## 2023-04-04 - `0.1.0`

- `!` - Major refactoring from `0.0.5`. 
	- Moved from raw `Vec..` to specialized type `FilterGroups` and `FilterGroup`.
	- Rename all of the `[Type]OpVal` to `OpVal[Type]` with full num type description. 
	- Implemented lot of `From` trait.