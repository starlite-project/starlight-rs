#![feature(negative_impls, once_cell)]
#![warn(
	clippy::pedantic,
	clippy::nursery,
	clippy::suspicious,
	missing_copy_implementations
)]
#![deny(clippy::all)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::module_name_repetitions,
	clippy::struct_excessive_bools
)]

pub mod components;
pub mod ext_traits;
pub mod helpers;
pub mod persistence;
pub mod slashies;
pub mod state;
pub mod utils;

pub use ext_traits::*;

pub mod build_info {
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
