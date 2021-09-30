#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	missing_copy_implementations,
	clippy::str_to_string,
	clippy::string_to_string
)]
#![deny(clippy::all)]

mod id;

pub use self::id::{Id, IdKey, ToIdKey};
