use crate::{ChartError, Value};
use heed::{
	types::{SerdeBincode, SerdeJson},
	Database, Env,
};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// An individual Map, wrapping around a [`Database`].
///
/// [`Database`]: heed::Database
pub struct StarMap<'a, S>(Database<SerdeJson<S::Key>, SerdeBincode<S>>, &'a Env)
where
	S: Value;

impl<'a, S: Value> StarMap<'a, S> {
	pub(crate) fn new(db: Database<SerdeJson<S::Key>, SerdeBincode<S>>, env: &'a Env) -> Self {
		Self(db, env)
	}

	/// todo
	pub fn get(self, key: &S::Key) -> Option<S> {
		let inner = self.0;

		let rtxn = self.1.read_txn().ok()?;

		inner.get(&rtxn, key).ok()?
	}

	/// todo
	///
	/// # Errors
	///
	/// todo
	pub fn update(self, value: &S) -> Result<(), ChartError> {
		let inner = self.0;

		let mut wtxn = self.1.write_txn()?;

		inner.put(&mut wtxn, &value.key(), value)?;

		wtxn.commit()?;

		Ok(())
	}

	/// todo
	///
	/// # Errors
	///
	/// todo
	pub fn update_bulk(self, values: &[S]) -> Result<(), ChartError> {
		let inner = self.0;

		let mut wtxn = self.1.write_txn()?;

		for value in values {
			inner.put(&mut wtxn, &value.key(), value)?;
		}

		wtxn.commit()?;

		Ok(())
	}

	/// todo
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
		Self(self.0, self.1)
	}
}

impl<'a, S> Copy for StarMap<'a, S> where S: Value {}
