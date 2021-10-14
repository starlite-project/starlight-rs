use supernova::ModelError;
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
	#[error(transparent)]
	Other(#[from] ModelError),
}

impl CacheHelperError {
	pub(super) const fn model_not_found() -> Self {
		Self::ModelNotFound
	}
}
