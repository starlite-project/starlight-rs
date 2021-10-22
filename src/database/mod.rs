#![allow(missing_copy_implementations)]

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub mod entity;

#[derive(Debug, Clone)]
pub struct StarChart {
	inner: DatabaseConnection,
}

impl StarChart {
	#[cfg(feature = "docker")]
	pub async fn new(url: &str) -> Result<Self, DbErr> {
		let url = {
			// Preallocate base, 11 for 'postgres://', plus the url len.
			let mut base = String::with_capacity(url.len() + 11);

			base.push_str("postgres://");
			base.push_str(url);

			base
		};

		let opts = ConnectOptions::from(url);

		let inner = Database::connect(opts).await?;

		Ok(Self { inner })
	}

	#[cfg(not(feature = "docker"))]
	pub async fn new(url: &str) -> Result<Self, DbErr> {
		let url = {
			// Preallocate base, 9 for 'sqlite://' plus the url len.
			let mut base = String::with_capacity(url.len() + 9);

			base.push_str("sqlite://");
			base.push_str(url);

			base
		};

		let opts = ConnectOptions::from(url);

		let inner = Database::connect(opts).await?;

		Ok(Self { inner })
	}

    async fn create_tables(&self) -> Result<(), DbErr> {
        Ok(())
    }
}
