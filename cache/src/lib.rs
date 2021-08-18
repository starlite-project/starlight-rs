#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]

mod config;

pub use self::{config::{Config, ResourceType}};