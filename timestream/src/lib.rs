#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::panic_in_result_fn,
	missing_copy_implementations
)]
#![deny(clippy::all)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::module_name_repetitions,
	clippy::struct_excessive_bools,
	clippy::suspicious_else_formatting
)]

mod schedule;

pub use self::schedule::{Schedule};
