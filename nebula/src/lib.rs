#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	missing_copy_implementations,
	clippy::str_to_string,
	clippy::string_to_string
)]
#![deny(clippy::all, missing_docs)]
#![allow(clippy::module_name_repetitions)]
//! Crate for utility types used in the starlight discord bot.

mod id;

pub use self::id::Id;

/// Leaks a value, causing it to be a static reference, but requiring manual cleanup.
/// 
/// The current implementation uses [`box leaking`].
/// 
/// [`box leaking`]: std::boxed::Box::leak
pub unsafe trait Leak {
	/// Leaks the value out, causing it to require manual cleanup later on.
	unsafe fn leak(self) -> &'static Self;
}

unsafe impl<T> Leak for T {
	unsafe fn leak(self) -> &'static Self {
		Box::leak(Box::new(self))
	}
}
