#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::struct_excessive_bools)]

pub mod cache;
pub mod entity;
pub mod repository;

mod backend;
mod utils;

pub use self::{backend::Backend, cache::Cache, entity::Entity, repository::Repository};
