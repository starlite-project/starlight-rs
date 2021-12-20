#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::panic_in_result_fn,
	missing_copy_implementations,
	missing_debug_implementations
)]
#![deny(clippy::all)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::module_name_repetitions,
	clippy::struct_excessive_bools,
	clippy::suspicious_else_formatting,
	clippy::no_effect_underscore_binding
)]
#![cfg_attr(test, allow(clippy::panic_in_result_fn))]

pub mod helpers;
pub mod prelude;
pub mod slashies;
pub mod state;
pub mod utils;
