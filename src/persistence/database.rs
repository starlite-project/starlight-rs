use super::settings::{ GuildSettings, SettingsHelper};
use anyhow::Result;
use std::{fmt::{Debug, Formatter, Result as FmtResult}, ops::Deref};
use structsy::Structsy;
use sysinfo::ProcessExt;
use tracing::{event, Level};

macro_rules! define {
	($db: expr, $($structs: ty),*) => {
		$(
			if !$db.is_defined::<$structs>()? {
				event!(Level::DEBUG, "defining struct {}", stringify!($structs));
				$db.define::<$structs>()?;
			}
		)*
	}
}

#[derive(Clone)]
pub struct Database(Structsy);

impl Database {
	pub fn open() -> Result<Self> {
		let db_path = {
			let process = crate::utils::get_current_process()?;

			let mut path = process.exe().to_path_buf();

			path.pop();

			path.push("star-db.stry");

			event!(Level::DEBUG, path = %path.display(), "database location");

			path
		};

		let db = Structsy::open(db_path)?;

		define!(db, GuildSettings);

		Ok(Self(db))
	}

	pub fn helper<'db, T: SettingsHelper<'db>>(&'db self) -> T {
		T::new(self)
	}

	// pub fn guilds(&self) -> GuildHelper {
	// 	GuildHelper::new(self)
	// }
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