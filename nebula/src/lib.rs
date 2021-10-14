#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	missing_copy_implementations,
	clippy::str_to_string,
	clippy::string_to_string
)]
#![deny(clippy::all)]
#![allow(clippy::module_name_repetitions)]

mod id;

pub use self::id::{Id};

pub trait Leak {
	fn leak(self) -> &'static Self;
}

impl<T> Leak for T {
	fn leak(self) -> &'static Self {
		Box::leak(Box::new(self))
	}
}
