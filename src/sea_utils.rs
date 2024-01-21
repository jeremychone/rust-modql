use sea_query::Iden;

/// String sea-query `Iden` wrapper
#[derive(Debug)]
pub struct StringIden(pub String);

impl Iden for StringIden {
	fn unquoted(&self, s: &mut dyn std::fmt::Write) {
		// Should never fail, but just in case, we do not crash, just print.
		if let Err(err) = s.write_str(&self.0) {
			println!("modql StringIden fail write_str. Cause: {err}");
		}
	}
}

/// Static str sea-query `Iden` wrapper
#[derive(Debug)]
pub struct SIden(pub &'static str);

impl Iden for SIden {
	fn unquoted(&self, s: &mut dyn std::fmt::Write) {
		// Should never fail, but just in case, we do not crash, just print.
		if let Err(err) = s.write_str(self.0) {
			println!("modql SIden fail write_str. Cause: {err}");
		}
	}
}
