pub enum OpValue {
	String(StringOpValue),
	Int(IntOpValue),
	Float(FloatOpValue),
}

pub enum StringOpValue {
	Eq(String),
	Not(String),
	In(Vec<String>),
	NotIn(Vec<String>),
	Lt(String),
	Lte(String),
	Gt(String),
	Gte(String),
	Empty(bool),

	Contains(String),
	NotContains(String),
	ContainsIn(Vec<String>),
	NotContainsIn(Vec<String>),
	StartsWith(String),
	NotStartsWith(String),
	StartsWithIn(Vec<String>),
	NotStartsWithIn(Vec<String>),
	EndsWith(String),
	NotEndsWith(String),
	EndsWithIn(Vec<String>),
	NotEndsWithIn(Vec<String>),
}

pub enum IntOpValue {
	Eq(i64),
	Not(i64),
	In(Vec<i64>),
	NotIn(Vec<i64>),
	Lt(i64),
	Lte(i64),
	Gt(i64),
	Gte(i64),
	Empty(bool),
}

pub enum FloatOpValue {
	Eq(f64),
	Not(f64),
	In(Vec<f64>),
	NotIn(Vec<f64>),
	Lt(f64),
	Lte(f64),
	Gt(f64),
	Gte(f64),
	Empty(bool),
}
