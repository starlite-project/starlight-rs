use heed::Database;
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// An individual Map, wrapping around a [`Database`].
///
/// [`Database`]: heed::Database
pub struct StarMap<K, V>(Database<K, V>);

impl<K, V> StarMap<K, V> {
	pub(crate) const fn new(db: Database<K, V>) -> Self {
		Self(db)
	}
}

impl<K, V> Debug for StarMap<K, V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_tuple("StatMap").field(&"Database").finish()
	}
}

impl<K, V> Clone for StarMap<K, V> {
	fn clone(&self) -> Self {
		Self(self.0)
	}
}

impl<K, V> Copy for StarMap<K, V> {}
