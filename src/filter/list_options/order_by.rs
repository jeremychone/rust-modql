// region:    --- OrderBy
#[derive(Debug, Clone)]
pub enum OrderBy {
	Asc(String),
	Desc(String),
}

impl core::fmt::Display for OrderBy {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			OrderBy::Asc(val) => {
				let content = quote_first_part(val);
				fmt.write_str(&content)?;
				fmt.write_str(" ")?;
				fmt.write_str("ASC")?;
			}
			OrderBy::Desc(val) => {
				let content = quote_first_part(val);
				fmt.write_str(&content)?;
				fmt.write_str(" ")?;
				fmt.write_str("DESC")?;
			}
		};

		Ok(())
	}
}

// region:    --- Formatter
// NOTE: Probably need to be generalized for other construct of this crate
fn quote_first_part(s: &str) -> String {
	if let Some((first, rest)) = s.split_once(' ') {
		format!("{} {}", quote_piece(first), rest)
	} else {
		quote_piece(s)
	}
}

fn quote_piece(piece: &str) -> String {
	if piece.contains('.') {
		piece
			.split('.')
			.map(|part| format!("\"{part}\"",))
			.collect::<Vec<_>>()
			.join(".")
	} else {
		format!("\"{piece}\"",)
	}
}
// endregion: --- Formatter

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

// endregion: --- OrderBy

// region:    --- OrderBys
#[derive(Debug, Clone)]
pub struct OrderBys(Vec<OrderBy>);

impl OrderBys {
	pub fn new(v: Vec<OrderBy>) -> Self {
		OrderBys(v)
	}
	pub fn order_bys(self) -> Vec<OrderBy> {
		self.0
	}

	pub fn join_for_sql(&self) -> String {
		self.0.iter().map(|o| format!("{o}")).collect::<Vec<_>>().join(", ")
	}
}

/// Builders with_
impl OrderBys {
	pub fn push(&mut self, order_by: impl Into<OrderBy>) {
		self.0.push(order_by.into());
	}

	pub fn append(mut self, order_by: impl Into<OrderBy>) -> Self {
		self.0.push(order_by.into());
		self
	}
}

// This will allow us to iterate over &OrderBys
impl<'a> IntoIterator for &'a OrderBys {
	type Item = &'a OrderBy;
	type IntoIter = std::slice::Iter<'a, OrderBy>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

// This will allow us to iterate over OrderBys directly (consuming it)
impl IntoIterator for OrderBys {
	type Item = OrderBy;
	type IntoIter = std::vec::IntoIter<OrderBy>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
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

impl From<OrderBy> for OrderBys {
	fn from(val: OrderBy) -> Self {
		OrderBys(vec![val])
	}
}

impl<T: AsRef<str>> From<Vec<T>> for OrderBys {
	fn from(val: Vec<T>) -> Self {
		let d = val.into_iter().map(|o| OrderBy::from(o)).collect::<Vec<_>>();
		OrderBys(d)
	}
}

// endregion: --- OrderBys

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use crate::sea_utils::StringIden;
	use sea_query::IntoColumnRef;

	impl OrderBys {
		pub fn into_sea_col_order_iter(self) -> impl Iterator<Item = (sea_query::ColumnRef, sea_query::Order)> {
			self.0.into_iter().map(OrderBy::into_sea_col_order)
		}
	}

	impl OrderBy {
		pub fn into_sea_col_order(self) -> (sea_query::ColumnRef, sea_query::Order) {
			let (col, order) = match self {
				OrderBy::Asc(col) => (StringIden(col), sea_query::Order::Asc),
				OrderBy::Desc(col) => (StringIden(col), sea_query::Order::Desc),
			};

			(col.into_column_ref(), order)
		}
	}
}
// endregion: --- with-sea-query
