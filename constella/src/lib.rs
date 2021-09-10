#![feature(doc_cfg)]
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]

use std::{
	cmp::Ordering,
	fmt::{Debug, Formatter, Result as FmtResult},
	io::{Read, Write},
	marker::PhantomData,
};
use structsy::{PersistentEmbedded, SRes};

mod implementors;

pub trait Transformer {
	type DataType: PersistentEmbedded;

	fn transform(&self) -> Self::DataType;

	fn revert(value: &Self::DataType) -> Self;
}

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

impl<V: Debug, T> Debug for Data<V, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Data")
			.field("inner", &self.inner)
			.field("_marker", &"_")
			.finish()
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

impl<V: PartialEq, T> PartialEq<V> for Data<V, T> {
    fn eq(&self, other: &V) -> bool {
        self.inner.eq(other)
    }
}

impl<V: PartialOrd, T> PartialOrd<V> for Data<V, T> {
    fn partial_cmp(&self, other: &V) -> Option<Ordering> {
        self.inner.partial_cmp(other)
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

        assert_eq!(wrapper, 0);

        let value_reverted = wrapper.value();

        assert_eq!(value_reverted, value);
    }
}