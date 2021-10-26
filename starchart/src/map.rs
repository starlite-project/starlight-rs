use crate::{ChartError, Value};
use heed::{
	types::{SerdeBincode, SerdeJson},
	Database, Env,
};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// An individual Map, wrapping around a [`Database`].
///
/// [`Database`]: heed::Database
pub struct StarMap<'a, S>
where
	S: Value,
{
	inner: Database<SerdeJson<S::Key>, SerdeBincode<S>>,
	env: &'a Env,
}

impl<'a, S: Value> StarMap<'a, S> {
	pub(crate) fn new(db: Database<SerdeJson<S::Key>, SerdeBincode<S>>, env: &'a Env) -> Self {
		Self { inner: db, env }
	}

	/// Get a value by the [`Key`].
	///
	/// [`Key`]: crate::Key
	pub fn get(self, key: &S::Key) -> Option<S> {
		let inner = self.inner;

		let rtxn = self.env.read_txn().ok()?;

		inner.get(&rtxn, key).ok()?
	}

	/// Updates a [`Value`].
	///
	/// [`Value`]: crate::Value
	///
	/// # Errors
	///
	/// See [`Error`].
	///
	/// [`Error`]: heed::Error
	pub fn update(self, value: &S) -> Result<(), ChartError> {
		let inner = self.inner;

		let mut wtxn = self.env.write_txn()?;

		inner.put(&mut wtxn, &value.key(), value)?;

		wtxn.commit()?;

		Ok(())
	}

	/// Updates many [`Value`]s in a single transaction.
	///
	/// # Errors
	///
	/// See [`Error`].
	///
	/// [`Error`]: heed::Error
	pub fn update_many(self, values: &[S]) -> Result<(), ChartError> {
		let inner = self.inner;

		let mut wtxn = self.env.write_txn()?;

		for value in values {
			inner.put(&mut wtxn, &value.key(), value)?;
		}

		wtxn.commit()?;

		Ok(())
	}

	/// Checks if a [`Value`] exists. Uses [`get`] internally.
	///
	/// [`get`]: StarMap::get
	pub fn contains(self, key: &S::Key) -> bool {
		matches!(self.get(key), Some(_))
	}
}

impl<'a, S> Debug for StarMap<'a, S>
where
	S: Value,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_tuple("StarMap").field(&"Database").finish()
	}
}

impl<'a, S> Clone for StarMap<'a, S>
where
	S: Value,
{
	fn clone(&self) -> Self {
		Self {
			inner: self.inner,
			env: self.env,
		}
	}
}

impl<'a, S> Copy for StarMap<'a, S> where S: Value {}
