#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::struct_excessive_bools)]

pub mod entity;
pub mod repository;

mod backend;
mod utils;

pub use self::{backend::Backend, entity::Entity, repository::Repository};
