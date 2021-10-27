#![feature(once_cell)]
#![deny(clippy::all, missing_docs)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::module_name_repetitions, clippy::unsafe_derive_deserialize)]
//! todo

use heed::Error;
use std::io;
use thiserror::Error;

mod chart;
mod helpers;
mod map;

/// An error that occurred when creating the database.
#[derive(Debug, Error)]
pub enum ChartError {
	/// An LMDB error occurred.
	#[error("an lmdb error occurred {0}")]
	Lmdb(#[from] Error),
	/// An IO error occurred.
	#[error("an io error occurred {0}")]
	Io(#[from] io::Error),
}

// SAFETY: idk, I'll look again later, maybe I'll switch to Sled
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for ChartError {}
unsafe impl Sync for ChartError {}

pub use self::{
	chart::{LmdbFlags, StarChart, StarChartBuilder},
	helpers::{Key, Value},
	map::StarMap,
};
