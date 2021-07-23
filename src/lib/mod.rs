pub mod commands;
pub mod state;

pub type GenericResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
