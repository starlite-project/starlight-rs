use anyhow::Result;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use structsy::Structsy;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

#[derive(Clone)]
pub struct Database(Structsy);

impl Database {
	pub fn open() -> Result<Self> {
		let db_path = {
			let system = System::new();

			let process = system
				.process(get_current_pid().expect("failed to get pid"))
				.expect("failed to get process");

			let mut path = process.exe().to_path_buf();

			path.pop();

			path.push("star-db.stry");

			path
		};

		Ok(Self(Structsy::open(db_path)?))
	}
}

impl Default for Database {
	fn default() -> Self {
		Self::open().expect("failed to open database")
	}
}

impl Debug for Database {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_tuple("Database").field(&"_").finish()
	}
}
