**STATUS** Still experimental.

**ModQL** Rust implementation for **Model Query Language** support. 

tl;dr - **ModQL** is a normalized declarative model agnostic language.  

See [joql](http://joql.org) for the JSON Representation of **modql**


First `OpVal` stands for `OperatorValue`. For example, `StringOpVal::Eq("my name".to_string())`.
There is to facade of ModQL. 



- First the way to define a filter for a particular model. This is typically done with `[Type]OpVals` such as `StringOpVals`, 
and those are just container type for their counter unit part `[Type]OpVal`

```rs
#[derive(Default, Debug, FilterNodes)]
struct TicketFilter {
	id: Option<IntOpVals>,
	name: Option<StringOpVals>,
}

// build it like this
let filter = TicketFilter {
	id: Some(IntOpVal::Gt(1).into()),
	..Default::default()
};
```

- And the side to implement the filter against a specific data interface (e.g., sql, file system, S3, ...), 
which uses the `FilterNode`, `OrGroups`, `OpVal` (which is a enum containing `StringOpVal` or `InOpval`)

