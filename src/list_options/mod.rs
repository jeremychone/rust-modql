mod order_by;

pub use order_by::*;
use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListOptions {
	pub limit: Option<i64>,
	pub offset: Option<i64>,
	pub order_bys: Option<OrderBys>,
}

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

// region:    --- with-sea-query
#[cfg(feature = "with-sea-query")]
mod with_sea_query {
	use super::*;
	use sea_query::SelectStatement;

	impl ListOptions {
		pub fn apply_to_sea_query(self, select_query: &mut SelectStatement) {
			fn as_positive_u64(num: i64) -> u64 {
				if num < 0 {
					0
				} else {
					num as u64
				}
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
