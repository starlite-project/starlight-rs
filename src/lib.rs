#![feature(negative_impls, once_cell)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
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

#[macro_export]
macro_rules! debug_unreachable {
	() => {
		$crate::debug_unreachable!("entered unreachable code")
	};
	($e:expr) => {
		if cfg!(not(debug_assertions)) {
			unsafe { std::hint::unreachable_unchecked() };
		} else {
			panic!($e)
		}
	};
}

#[macro_export]
macro_rules! model {
	($request:expr) => {
		crate::finish_request!($request, model)
	};
}

#[macro_export]
macro_rules! text {
	($request:expr) => {
		crate::finish_request!($request, text)
	};
}

#[macro_export]
macro_rules! bytes {
	($request:expr) => {
		crate::finish_request!($request, bytes)
	};
}

#[macro_export]
macro_rules! finish_request {
	($request:expr, $type:ident) => {
		$request.exec().await?.$type().await?
	};
}
