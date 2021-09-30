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

pub use self::id::{Id, IdKey, ToIdKey};
