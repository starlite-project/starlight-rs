#![feature(negative_impls, once_cell, const_fn_trait_bounds)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![deny(clippy::all)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::module_name_repetitions,
	clippy::struct_excessive_bools
)]

use std::{
	alloc::{GlobalAlloc, Layout},
	fs::Metadata,
	io::Result,
	path::Path,
	sync::atomic::{AtomicUsize, Ordering},
};

pub mod components;
pub mod ext_traits;
pub mod helpers;
pub mod slashies;
pub mod state;

pub use ext_traits::*;

pub fn get_binary_metadata() -> Result<Metadata> {
	Path::new("./target")
		.join(if cfg!(debug_assertions) {
			"debug"
		} else {
			"release"
		})
		.join(format!(
			"starlight-rs{}",
			if cfg!(windows) { ".exe" } else { "" }
		))
		.canonicalize()?
		.metadata()
}

pub struct Trallocator<A: GlobalAlloc>(pub A, AtomicUsize);

impl<A: GlobalAlloc> Trallocator<A> {
	pub const fn new(a: A) -> Self {
		Self(a, AtomicUsize::new(0))
	}

	pub fn reset(&self) {
		self.1.store(0, Ordering::SeqCst);
	}

	pub fn get(&self) -> usize {
		self.1.load(Ordering::SeqCst)
	}
}

unsafe impl<A: GlobalAlloc> GlobalAlloc for Trallocator<A> {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		self.1.fetch_add(layout.size(), Ordering::SeqCst);
		self.0.alloc(layout)
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		self.0.dealloc(ptr, layout);
		self.1.fetch_sub(layout.size(), Ordering::SeqCst);
	}
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
macro_rules! list_models {
	($request:expr) => {
		crate::finish_request!($request, models)
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
