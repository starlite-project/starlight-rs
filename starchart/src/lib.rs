#![deny(clippy::all, missing_docs)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::module_name_repetitions, clippy::unsafe_derive_deserialize)]
//! todo

mod chart;
mod map;
mod helpers;

pub use self::{
	chart::{StarChart, StarChartBuilder, LmdbFlags},
	map::StarMap,
	helpers::{Key, Value}
};
