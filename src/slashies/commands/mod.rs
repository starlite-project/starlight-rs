#[path = "crate.rs"]
mod krate;
mod ping;

pub use self::{krate::Crate, ping::Ping};
