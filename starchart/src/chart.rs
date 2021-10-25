use crate::{StarMap, Value};
use bitflags::bitflags;
use heed::{flags::Flags, Env, EnvOpenOptions, Result};
use serde::{Deserialize, Serialize};
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	path::Path,
};

bitflags! {
	/// [`Flags`] to use for LMDB.
	/// 
	/// [`Flags`]: heed::flags::Flags
	#[derive(Serialize, Deserialize)]
	pub struct LmdbFlags: u32 {
		#[allow(missing_docs)]
		const FIXED_MAP = 1;
		#[allow(missing_docs)]
		const NO_SUB_DIR = 1 << 1;
		#[allow(missing_docs)]
		const NO_SYNC = 1 << 2;
		#[allow(missing_docs)]
		const RD_ONLY = 1 << 3;
		#[allow(missing_docs)]
		const NO_META_SYNC = 1 << 4;
		#[allow(missing_docs)]
		const WRITE_MAP = 1 << 5;
		#[allow(missing_docs)]
		const MAP_ASYNC = 1 << 6;
		#[allow(missing_docs)]
		const NO_TLS = 1 << 7;
		#[allow(missing_docs)]
		const NO_LOCK = 1 << 8;
		#[allow(missing_docs)]
		const NO_RD_AHEAD = 1 << 9;
		#[allow(missing_docs)]
		const NO_MEM_INIT = 1 << 10;
	}
}

impl LmdbFlags {
	fn to_heed_flags(self) -> Vec<Flags> {
		if self.is_empty() {
			return Vec::new();
		}

		if self.is_all() {
			return Vec::from([
				Flags::MdbFixedmap,
				Flags::MdbNoSubDir,
				Flags::MdbNoSync,
				Flags::MdbRdOnly,
				Flags::MdbNoMetaSync,
				Flags::MdbWriteMap,
				Flags::MdbMapAsync,
				Flags::MdbNoTls,
				Flags::MdbNoLock,
				Flags::MdbNoRdAhead,
				Flags::MdbNoMemInit,
			]);
		}

		let mut output = Vec::new();
		if self.contains(Self::FIXED_MAP) {
			output.push(Flags::MdbFixedmap);
		}
		if self.contains(Self::NO_SUB_DIR) {
			output.push(Flags::MdbNoSubDir);
		}
		if self.contains(Self::NO_SYNC) {
			output.push(Flags::MdbNoSync);
		}
		if self.contains(Self::RD_ONLY) {
			output.push(Flags::MdbRdOnly);
		}
		if self.contains(Self::NO_META_SYNC) {
			output.push(Flags::MdbNoMetaSync);
		}
		if self.contains(Self::WRITE_MAP) {
			output.push(Flags::MdbWriteMap);
		}
		if self.contains(Self::MAP_ASYNC) {
			output.push(Flags::MdbMapAsync);
		}
		if self.contains(Self::NO_TLS) {
			output.push(Flags::MdbNoTls);
		}
		if self.contains(Self::NO_LOCK) {
			output.push(Flags::MdbNoLock);
		}
		if self.contains(Self::NO_RD_AHEAD) {
			output.push(Flags::MdbNoRdAhead);
		}
		if self.contains(Self::NO_MEM_INIT) {
			output.push(Flags::MdbNoMemInit);
		}
		output
	}
}

impl Default for LmdbFlags {
	fn default() -> Self {
		Self::empty()
	}
}

/// A builder used to build a [`StarChart`].
#[must_use = "the builder has no use if not built"]
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct StarChartBuilder {
	size: Option<usize>,
	readers: Option<u32>,
	dbs: Option<u32>,
	flags: LmdbFlags,
}

impl StarChartBuilder {
	/// Creates a new builder.
	pub const fn new() -> Self {
		Self {
			size: None,
			readers: None,
			dbs: None,
			flags: LmdbFlags::empty(),
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
	pub fn flags(mut self, flags: LmdbFlags) -> Self {
		self.flags |= flags;
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
		let flags = self.flags.to_heed_flags();

		for flag in flags {
			unsafe {
				env_options.flag(flag);
			}
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
	pub fn get<'a, S>(&'a self, database_name: Option<&str>) -> Option<StarMap<'a, S>>
	where
		S: Value + 'static,
	{
		let db = self.0.open_database(database_name).ok()??;

		Some(StarMap::new(db, &self.0))
	}

	/// Creates a new [`StarMap`] for interacting with a single DB.
	///
	/// # Errors
	///
	/// See [`Error`]
	///
	/// [`Error`]: heed::Error
	pub fn create<'a, S>(&'a self, database_name: Option<&str>) -> Result<StarMap<S>>
	where
		S: Value + 'static,
	{
		let db = self.0.create_database(database_name)?;

		Ok(StarMap::new(db, &self.0))
	}

	/// Acquires a DB by name, uses [`StarChart::get`] first, then [`StarChart::create`] if not found.
	///
	/// # Errors
	///
	/// See [`Error`]
	///
	/// [`Error`]: heed::Error
	pub fn acquire<S>(&self, database_name: Option<&str>) -> Result<StarMap<S>>
	where
		S: Value + 'static,
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
