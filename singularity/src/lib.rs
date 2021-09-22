#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![cfg_attr(backtrace, feature(backtrace))]

#[macro_use]
mod backtrace;
mod chain;
mod error;
mod ptr;
mod wrapper;
