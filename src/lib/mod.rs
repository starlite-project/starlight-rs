pub mod state;
pub mod util;

pub type GenericResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
