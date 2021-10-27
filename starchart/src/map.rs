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
	pub fn get(self, key: S::Key) -> Option<S> {
		let inner = self.inner;

		let rtxn = self.env.read_txn().ok()?;

		inner.get(&rtxn, &key).ok()?
	}

	/// Acquires a [`Value`], using [`StarMap::get`] first, then resorts to [`StarMap::create`]
	/// if not found.
	///
	/// # Errors
	///
	/// See [`Error`].
	///
	/// [`Error`]: heed::Error
	pub fn acquire(self, key: S::Key) -> Result<S, ChartError> {
		if self.contains(key) {
			Ok(self
				.get(key)
				.expect("failed to get value (this shoudn't happen)"))
		} else {
			self.create(key)
		}
	}

	/// Ensures a value exists in the Map, creating it if it doesn't exist.
	///
	/// # Errors
	///
	/// See [`Error`].
	///
	/// [`Error`]: heed::Error
	pub fn ensure(self, key: S::Key) -> Result<(), ChartError> {
		if self.contains(key) {
			return Ok(());
		}
		self.acquire(key)?;
		Ok(())
	}

	/// Creates a value, using [`Value::new`] internally.
	///
	/// [`Value::new`]: crate::Value::new
	///
	/// # Errors
	///
	/// See [`Error`].
	///
	/// [`Error`]: heed::Error
	pub fn create(self, key: S::Key) -> Result<S, ChartError> {
		let inner = self.inner;
		{
			let mut wtxn = self.env.write_txn()?;

			let value = S::new(key);

			inner.put(&mut wtxn, &key, &value)?;

			wtxn.commit()?;
		}

		let rtxn = self.env.read_txn()?;

		Ok(inner
			.get(&rtxn, &key)?
			.expect("failed to find value (this shouldn't happen)"))
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
	pub fn contains(self, key: S::Key) -> bool {
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

// need to implement clone manually so that the compiler doesn't add the S: Clone bound, as it's not needed
#[allow(clippy::expl_impl_clone_on_copy)]
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
