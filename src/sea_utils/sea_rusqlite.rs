//! JC-2025-07-01 NOTE:
//! This is to see if we can avoid the dependency catch-22 with the rusqlite and sea-query by inlining the binding lib
//! https://github.com/SeaQL/sea-query/blob/master/sea-query-rusqlite/src/lib.rs

use rusqlite::{
	Result, ToSql,
	types::{Null, ToSqlOutput},
};
use sea_query::{OptionEnum, Value};
use sea_query::{QueryBuilder, query::*};

#[derive(Clone, Debug, PartialEq)]
pub struct RusqliteValue(pub sea_query::Value);
#[derive(Clone, Debug, PartialEq)]
pub struct RusqliteValues(pub Vec<RusqliteValue>);

impl RusqliteValues {
	pub fn as_params(&self) -> Vec<&dyn ToSql> {
		self.0
			.iter()
			.map(|x| {
				let y: &dyn ToSql = x;
				y
			})
			.collect()
	}
}

pub trait RusqliteBinder {
	fn build_rusqlite<T: QueryBuilder>(&self, query_builder: T) -> (String, RusqliteValues);
}

macro_rules! impl_rusqlite_binder {
	($l:ident) => {
		impl RusqliteBinder for $l {
			fn build_rusqlite<T: QueryBuilder>(&self, query_builder: T) -> (String, RusqliteValues) {
				let (query, values) = self.build(query_builder);
				(
					query,
					RusqliteValues(values.into_iter().map(RusqliteValue).collect()),
				)
			}
		}
	};
}

impl_rusqlite_binder!(SelectStatement);
impl_rusqlite_binder!(UpdateStatement);
impl_rusqlite_binder!(InsertStatement);
impl_rusqlite_binder!(DeleteStatement);
impl_rusqlite_binder!(WithQuery);

impl ToSql for RusqliteValue {
	fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
		macro_rules! opt_string_to_sql {
			( $v: expr ) => {
				match $v {
					Some(v) => Ok(ToSqlOutput::from(v)),
					None => Null.to_sql(),
				}
			};
		}

		match &self.0 {
			Value::Bool(v) => v.to_sql(),
			Value::TinyInt(v) => v.to_sql(),
			Value::SmallInt(v) => v.to_sql(),
			Value::Int(v) => v.to_sql(),
			Value::BigInt(v) => v.to_sql(),
			Value::TinyUnsigned(v) => v.to_sql(),
			Value::SmallUnsigned(v) => v.to_sql(),
			Value::Unsigned(v) => v.to_sql(),
			//-- JC-2026-11-09: Make an approximation for now, to i64
			//                  This is not a perfect options, but panic should not either
			Value::BigUnsigned(v) => match v {
				Some(v) => {
					// Safely converts to i64, fallback to i64::MAX if it overflows u64
					let clamped_val = i64::try_from(*v).unwrap_or(i64::MAX);
					Ok(ToSqlOutput::from(clamped_val))
				}
				None => Null.to_sql(),
			},
			Value::Float(v) => v.to_sql(),
			Value::Double(v) => v.to_sql(),
			Value::String(v) => match v {
				Some(v) => v.as_str().to_sql(),
				None => Null.to_sql(),
			},
			Value::Enum(v) => match v {
				OptionEnum::Some(v) => v.value.as_ref().to_sql(),
				OptionEnum::None(_) => Null.to_sql(),
			},
			Value::Char(v) => opt_string_to_sql!(v.map(|v| v.to_string())),
			Value::Bytes(v) => match v {
				Some(v) => v.as_slice().to_sql(),
				None => Null.to_sql(),
			},
		}
	}
}
