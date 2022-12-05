use core::fmt;

#[derive(Debug)]
pub enum OrderBy {
	Asc(String),
	Desc(String),
}

impl fmt::Display for OrderBy {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		match self {
			OrderBy::Asc(val) => {
				fmt.write_str(val);
				fmt.write_str(" ");
				fmt.write_str("ASC");
			}
			OrderBy::Desc(val) => {
				fmt.write_str(val);
				fmt.write_str(" ");
				fmt.write_str("DESC");
			}
		};

		Ok(())
	}
}

impl<T: AsRef<str>> From<T> for OrderBy {
	fn from(val: T) -> Self {
		let raw: &str = val.as_ref();

		if let Some(stripped) = raw.strip_prefix('!') {
			OrderBy::Desc(stripped.to_string())
		} else {
			OrderBy::Asc(raw.to_string())
		}
	}
}

// impl From<&str> for OrderBy {
// 	fn from(raw: &str) -> Self {
// 		if let Some(stripped) = raw.strip_prefix('!') {
// 			OrderBy::Desc(stripped.to_string())
// 		} else {
// 			OrderBy::Asc(raw.to_string())
// 		}
// 	}
// }

#[derive(Debug)]
pub struct OrderBys(Vec<OrderBy>);

impl OrderBys {
	pub fn new(v: Vec<OrderBy>) -> Self {
		OrderBys(v)
	}
	pub fn order_bys(self) -> Vec<OrderBy> {
		self.0
	}
}

// NOTE: If we want the Vec<T> and T, we have to make the individual from
//       specific to the type. Otherwise, conflict.

impl From<&str> for OrderBys {
	fn from(val: &str) -> Self {
		OrderBys(vec![val.into()])
	}
}
impl From<&String> for OrderBys {
	fn from(val: &String) -> Self {
		OrderBys(vec![val.into()])
	}
}
impl From<String> for OrderBys {
	fn from(val: String) -> Self {
		OrderBys(vec![val.into()])
	}
}

impl<T: AsRef<str>> From<Vec<T>> for OrderBys {
	fn from(val: Vec<T>) -> Self {
		let d = val.into_iter().map(|o| OrderBy::from(o)).collect::<Vec<_>>();
		OrderBys(d)
	}
}
