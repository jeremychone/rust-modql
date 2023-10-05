use sea_query::Condition;

pub trait SeaFilter {
	fn into_sea_condition(self) -> Condition;
}
