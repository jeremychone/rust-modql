use crate::OrderBys;

#[derive(Default, Debug)]
pub struct ListOptions {
	pub limit: Option<i64>,
	pub offset: Option<i64>,
	pub order_bys: Option<OrderBys>,
}
