use heed::{flags::Flags, types::SerdeBincode, BytesEncode, Env, EnvOpenOptions, Result};
use serde::{Deserialize, Serialize};
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	path::Path,
};

use crate::StarMap;

/// A builder used to build a [`StarChart`].
#[must_use = "the builder has no use if not built"]
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct StarChartBuilder {
	size: Option<usize>,
	readers: Option<u32>,
	dbs: Option<u32>,
	flags: u32,
}

impl StarChartBuilder {
	/// Creates a new builder.
	pub const fn new() -> Self {
		Self {
			size: None,
			readers: None,
			dbs: None,
			flags: 0,
		}
	}

	/// Sets the `map_size` for the [`StarChart`].
	pub const fn size(mut self, size: usize) -> Self {
		self.size = Some(size);
		self
	}

	/// Sets the `max_readers` for the [`StarChart`].
	pub const fn readers(mut self, readers: u32) -> Self {
		self.readers = Some(readers);
		self
	}

	/// Sets the `max_dbs` for the [`StarChart`].
	pub const fn dbs(mut self, dbs: u32) -> Self {
		self.dbs = Some(dbs);
		self
	}

	/// Sets one or [more LMDB flags](http://www.lmdb.tech/doc/group__mdb__env.html).
	///
	/// # Safety
	///
	/// Values must be valid flags
	pub const unsafe fn flags(mut self, flags: Flags) -> Self {
		self.flags |= flags as u32;
		self
	}

	/// Builds the [`Env`] used with the [`StarChart`].
	///
	/// [`Env`]: heed::Env
	///
	/// # Errors
	///
	/// See [`Error`]
	///
	/// [`Error`]: heed::Error
	#[must_use = "building the starchart has no side effects"]
	pub fn build<P: AsRef<Path>>(self, path: P) -> Result<StarChart> {
		let mut env_options = EnvOpenOptions::new();
		if let Some(map_size) = self.size {
			env_options.map_size(map_size);
		}

		if let Some(readers) = self.readers {
			env_options.max_readers(readers);
		}

		if let Some(dbs) = self.dbs {
			env_options.max_dbs(dbs);
		}

		let env = env_options.open(path)?;

		Ok(StarChart(env))
	}
}

/// The database wrapper, giving ease of access methods for the inner [`Env`].
///
/// The [`Env`] is wrapped in an [`Arc`], so clones are cheap and don't cause data races.
///
/// [`Env`]: heed::Env
/// [`Arc`]: std::sync::Arc
#[derive(Clone)]
pub struct StarChart(Env);

impl StarChart {
	/// Creates a new [`StarChart`] without setting any options, equal to `StarChartBuilder::default().build(path)`.
	///
	/// # Errors
	///
	/// See [`StarChartBuilder::build`]
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		StarChartBuilder::default().build(path)
	}

	/// Creates a [`StarChartBuilder`].
	pub const fn builder() -> StarChartBuilder {
		StarChartBuilder::new()
	}

	/// Gets an existing [`StarMap`] for interacting with a single DB.
	#[must_use]
	pub fn get<'a, 'db, K, V>(
		&self,
		database_name: Option<&str>,
	) -> Option<StarMap<K, SerdeBincode<V>>>
	where
		K: 'static + BytesEncode<'a>,
		V: 'static + Serialize + Deserialize<'db>,
	{
		let db = self.0.open_database(database_name).ok()??;

		Some(StarMap::new(db))
	}

	/// Creates a new [`StarMap`] for interacting with a single DB.
	///
	/// # Errors
	///
	/// See [`Error`]
	///
	/// [`Error`]: heed::Error
	pub fn create<'a, 'db, K, V>(
		&self,
		database_name: Option<&str>,
	) -> Result<StarMap<K, SerdeBincode<V>>>
	where
		K: 'static + BytesEncode<'a>,
		V: 'static + Serialize + Deserialize<'db>,
	{
		let db = self.0.create_database(database_name)?;

		Ok(StarMap::new(db))
	}

	/// Acquires a DB by name, uses [`StarChart::get`] first, then [`StarChart::create`] if not found.
	///
	/// # Errors
	///
	/// See [`Error`]
	///
	/// [`Error`]: heed::Error
	pub fn acquire<'a, 'db, K, V>(
		&self,
		database_name: Option<&str>,
	) -> Result<StarMap<K, SerdeBincode<V>>>
	where
		K: 'static + BytesEncode<'a>,
		V: 'static + Serialize + Deserialize<'db>,
	{
		self.get(database_name)
			.map_or_else(|| self.create(database_name), Ok)
	}
}

impl Debug for StarChart {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_tuple("StarChart").field(&"Env").finish()
	}
}
