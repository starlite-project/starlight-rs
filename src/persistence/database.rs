use std::fmt::{Debug, Formatter, Result as FmtResult};
use structsy::Structsy;

#[derive(Clone)]
pub struct Database(Structsy);

impl Debug for Database {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_tuple("Database").field(&"..").finish()
	}
}
