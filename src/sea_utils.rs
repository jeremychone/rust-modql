use sea_query::Iden;

pub struct StringIden(pub String);

impl Iden for StringIden {
	fn unquoted(&self, s: &mut dyn std::fmt::Write) {
		s.write_str(&self.0).expect("StringIden fail to write_str");
	}
}

/// Static str sea-query `Iden` struct and implementation.
#[derive(Debug)]
pub struct SIden(pub &'static str);
impl Iden for SIden {
	fn unquoted(&self, s: &mut dyn std::fmt::Write) {
		s.write_str(self.0).expect("SIden write_str fatal error");
	}
}
