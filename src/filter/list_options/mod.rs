mod order_by;

pub use order_by::*;
use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListOptions {
	pub limit: Option<i64>,
	pub offset: Option<i64>,
	pub order_bys: Option<OrderBys>,
}

/// Constructors
impl ListOptions {
	pub fn from_limit(limit: i64) -> Self {
		Self {
			limit: Some(limit),
			..Default::default()
		}
	}

	pub fn from_offset_limit(offset: i64, limit: i64) -> Self {
		Self {
			limit: Some(limit),
			offset: Some(offset),
			..Default::default()
		}
	}

	pub fn from_order_bys(order_bys: impl Into<OrderBys>) -> Self {
		Self {
			order_bys: Some(order_bys.into()),
			..Default::default()
		}
	}
}

/// Builders with_
impl ListOptions {
	pub fn with_limit(mut self, limit: i64) -> Self {
		self.limit = Some(limit);
		self
	}

	pub fn with_offset(mut self, offset: i64) -> Self {
		self.offset = Some(offset);
		self
	}

	pub fn with_order_bys(mut self, order_bys: impl Into<OrderBys>) -> Self {
		self.order_bys = Some(order_bys.into());
		self
	}

	pub fn append_order_by(mut self, order_by: impl Into<OrderBy>) -> Self {
		let order_by = order_by.into();
		if let Some(order_bys) = &mut self.order_bys {
			order_bys.push(order_by);
		} else {
			self.order_bys = Some(OrderBys::from(order_by));
		}
		self
	}
}

// region:    --- Froms

impl From<OrderBys> for ListOptions {
	fn from(val: OrderBys) -> Self {
		Self {
			order_bys: Some(val),
			..Default::default()
		}
	}
}

impl From<OrderBys> for Option<ListOptions> {
	fn from(val: OrderBys) -> Self {
		Some(ListOptions {
			order_bys: Some(val),
			..Default::default()
		})
	}
}

impl From<OrderBy> for ListOptions {
	fn from(val: OrderBy) -> Self {
		Self {
			order_bys: Some(OrderBys::from(val)),
			..Default::default()
		}
	}
}

impl From<OrderBy> for Option<ListOptions> {
	fn from(val: OrderBy) -> Self {
		Some(ListOptions {
			order_bys: Some(OrderBys::from(val)),
			..Default::default()
		})
	}
}

// endregion: --- Froms

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use sea_query::SelectStatement;

	impl ListOptions {
		pub fn apply_to_sea_query(self, select_query: &mut SelectStatement) {
			fn as_positive_u64(num: i64) -> u64 {
				if num < 0 { 0 } else { num as u64 }
			}
			if let Some(limit) = self.limit {
				select_query.limit(as_positive_u64(limit)); // Note: Negative == 0
			}

			if let Some(offset) = self.offset {
				select_query.offset(as_positive_u64(offset)); // Note: Negative == 0
			}

			if let Some(order_bys) = self.order_bys {
				for (col, order) in order_bys.into_sea_col_order_iter() {
					select_query.order_by(col, order);
				}
			}
		}
	}
}
// endregion: --- with-sea-query
