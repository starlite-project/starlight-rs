pub mod client;
pub mod client_old;
pub mod util;

pub type GenericResult<T> = Result<T, Box<dyn std::error::Error + Send +Sync>>;