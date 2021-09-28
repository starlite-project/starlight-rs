#![feature(doc_cfg)]
#![deny(clippy::all)]
#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	clippy::str_to_string,
	clippy::string_to_string,
	missing_copy_implementations
)]
// Allowed bc it needs to be derived to allow it to be derived in separate structs
// But it's also implemented as it is a smart pointer
#![allow(clippy::derive_hash_xor_eq)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
	cmp::Ordering,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	hash::{Hash, Hasher},
	io::{Read, Write},
	marker::PhantomData,
};
use structsy::{
	internal::{Description, EmbeddedDescription, EmbeddedFilterBuilder, FilterDefinition},
	PersistentEmbedded, SRes,
};

mod describer_impls;
mod transformer_impls;

pub mod external {
	pub use structsy::internal::{
		Description, EnumDescription, FieldDescription, PersistentEmbedded, StructDescription,
	};
}

pub trait Transformer {
	type DataType: PersistentEmbedded;

	fn transform(&self) -> Self::DataType;

	fn revert(value: &Self::DataType) -> Self;
}

pub trait Describer {
	fn description() -> Description;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Data<V, T> {
	inner: V,
	_marker: PhantomData<T>,
}

pub type DataTransformer<V> = Data<<V as Transformer>::DataType, V>;

impl<V, T> Data<V, T>
where
	T: Transformer<DataType = V>,
{
	pub fn new(value: &T) -> Self {
		Self {
			inner: value.transform(),
			_marker: PhantomData,
		}
	}

	pub fn data(&self) -> &V {
		&self.inner
	}

	pub fn raw(self) -> V {
		self.inner
	}

	pub fn value(&self) -> T {
		T::revert(&self.inner)
	}
}

impl<V, T> PersistentEmbedded for Data<V, T>
where
	V: PersistentEmbedded,
	T: Transformer<DataType = V>,
{
	fn read(read: &mut dyn Read) -> SRes<Self>
	where
		Self: Sized,
	{
		let inner = V::read(read)?;

		Ok(Self {
			inner,
			_marker: PhantomData,
		})
	}

	fn write(&self, write: &mut dyn Write) -> SRes<()> {
		self.inner.write(write)
	}
}

impl<V, T> EmbeddedDescription for Data<V, T>
where
	V: PersistentEmbedded,
	T: Transformer<DataType = V> + Describer,
{
	fn get_description() -> Description {
		T::description()
	}
}

impl<V, T> FilterDefinition for Data<V, T>
where
	T: Describer,
{
	type Filter = EmbeddedFilterBuilder<Self>;
}

impl<V, T> From<T> for Data<V, T>
where
	T: Transformer<DataType = V>,
{
	fn from(value: T) -> Self {
		Self {
			inner: value.transform(),
			_marker: PhantomData,
		}
	}
}

impl<V, T> Debug for Data<V, T>
where
	T: Transformer<DataType = V> + Debug,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		// It's a smart pointer, so we don't want to reveal the inner workings
		self.value().fmt(f)
	}
}

impl<V, T> Display for Data<V, T>
where
	T: Transformer<DataType = V> + Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.value().fmt(f)
	}
}

impl<V, T> Default for Data<V, T>
where
	T: Transformer<DataType = V> + Default,
{
	fn default() -> Self {
		let inner = T::default().transform();

		Self {
			inner,
			_marker: PhantomData,
		}
	}
}

impl<V: Clone, T> Clone for Data<V, T> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			_marker: PhantomData,
		}
	}
}

impl<V: Copy, T> Copy for Data<V, T> {}

impl<V, T> PartialEq<T> for Data<V, T>
where
	T: Transformer<DataType = V> + PartialEq,
{
	fn eq(&self, other: &T) -> bool {
		self.value().eq(other)
	}
}

impl<V, T> PartialOrd<T> for Data<V, T>
where
	T: Transformer<DataType = V> + PartialOrd,
{
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		self.value().partial_cmp(other)
	}
}

impl<V, T> Hash for Data<V, T>
where
	T: Transformer<DataType = V> + Hash,
{
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value().hash(state);
	}
}

impl<V, T> Serialize for Data<V, T>
where
	T: Transformer<DataType = V> + Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.value().serialize(serializer)
	}
}

impl<'de, V, T> Deserialize<'de> for Data<V, T>
where
	T: Transformer<DataType = V> + Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let value = T::deserialize(deserializer)?;

		Ok(Self::from(value))
	}
}

unsafe impl<V: Send, T> Send for Data<V, T> {}
unsafe impl<V: Sync, T> Sync for Data<V, T> {}

#[cfg(test)]
mod tests {
	use super::{Data, Transformer};

	#[derive(Debug, Default, Clone, Copy, PartialEq)]
	struct Id(pub u64);

	impl Transformer for Id {
		type DataType = u64;

		fn transform(&self) -> Self::DataType {
			self.0
		}

		fn revert(value: &Self::DataType) -> Self {
			Self(*value)
		}
	}

	#[test]
	fn persistent_embedded() {
		let value = Id::default();

		let wrapper = Data::from(value);

		assert_eq!(*wrapper.data(), 0);
	}
}
