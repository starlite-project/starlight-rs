#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]

pub mod model;

mod config;

pub use self::{config::{Config, ResourceType}};