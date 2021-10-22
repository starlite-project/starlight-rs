#![allow(missing_copy_implementations, dead_code)]

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::future::Future;

pub mod entity;

pub use entity::GuildSettings;

macro_rules! define {
	($db:expr, $($schemas: ty),*) => {
		$(
			<$schemas as $crate::database::entity::SchemaDefinition>::execute(&$db, &<$schemas as $crate::database::entity::CreateTable>::schema()).await?;
		)*
	}
}

#[derive(Debug, Clone)]
pub struct StarChart {
	inner: DatabaseConnection,
}

impl StarChart {
	#[cfg(feature = "docker")]
	pub fn new(url: &str) -> impl Future<Output = Result<Self, DbErr>> {

		let url = {
			// Preallocate base, 11 for 'postgres://', plus the url len.
			let mut base = String::with_capacity(url.len() + 11);

			base.push_str("postgres://");
			base.push_str(url);

			base
		};

		Self::_new(url)
	}

	#[cfg(not(feature = "docker"))]
	pub fn new(url: &str) -> impl Future<Output = Result<Self, DbErr>> {
		let url = {
			// Preallocate base, 9 for 'sqlite://' plus the url len.
			let mut base = String::with_capacity(url.len() + 9);

			base.push_str("sqlite://");
			base.push_str(url);

			base
		};

		Self::_new(url)
	}

	async fn _new<C: Into<ConnectOptions> + Send + Sync>(opts: C) -> Result<Self, DbErr> {
		let inner = Database::connect(opts).await?;

		let this = Self {inner};

		this.create_tables().await?;

		Ok(this)
	}

    async fn create_tables(&self) -> Result<(), DbErr> {
		define!(self.inner, GuildSettings);
        Ok(())
    }
}
