#![feature(
	negative_impls,
	once_cell,
	option_result_unwrap_unchecked,
	maybe_uninit_uninit_array,
	maybe_uninit_array_assume_init,
	result_flattening
)]
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

pub mod components;
pub mod database;
pub mod helpers;
pub mod slashies;
pub mod state;
pub mod utils;

pub mod build_info {
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
