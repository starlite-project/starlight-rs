#![deny(clippy::all, missing_docs)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![allow(clippy::module_name_repetitions)]
//! todo

mod chart;
mod map;

pub use self::{
	chart::{StarChart, StarChartBuilder},
	map::StarMap,
};
