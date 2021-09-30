use thiserror::Error;
use twilight_http::{response::DeserializeBodyError, Error as HttpError};

#[derive(Debug, Error)]
pub enum CacheHelperError {
	#[error("failed to deserialize the body")]
	DeserializeBody(#[from] DeserializeBodyError),
	#[error("an http error occurred")]
	Http(#[from] HttpError),
	#[error("the requested model was not found")]
	ModelNotFound,
}

impl CacheHelperError {
	pub(super) fn model_not_found() -> Self {
		Self::ModelNotFound
	}
}
