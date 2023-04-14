mod order_by;

pub use order_by::*;

#[derive(Default, Debug)]
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
