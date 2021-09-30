use super::settings::{ClientSettings, GuildSettings, SettingsHelper};
use miette::{IntoDiagnostic, Result};
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	ops::Deref,
};
use structsy::Structsy;
use sysinfo::ProcessExt;
use tracing::{event, Level};

macro_rules! define {
	($db: expr, $($structs: ty),*) => {
		$(
			if !$db.is_defined::<$structs>().into_diagnostic()? {
				event!(Level::DEBUG, "defining struct {}", stringify!($structs));
				$db.define::<$structs>().into_diagnostic()?;
			}
		)*
	}
}

#[derive(Clone)]
pub struct Database(Structsy);

impl Database {
	pub fn open() -> Result<Self> {
		let db_path = {
			let process = crate::utils::get_current_process().into_diagnostic()?;

			let mut path = process.exe().to_path_buf();

			path.pop();

			path.push("star-db.stry");

			event!(Level::DEBUG, path = %path.display(), "database location");

			path
		};

		let db = Structsy::open(db_path).into_diagnostic()?;

		define!(db, GuildSettings, ClientSettings);

		Ok(Self(db))
	}

	#[must_use]
	pub fn helper<'db, T: SettingsHelper<'db>>(&'db self) -> T {
		T::new(self)
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

impl Deref for Database {
	type Target = Structsy;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
