use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_http::{response::DeserializeBodyError, Error as HttpError};

#[derive(Debug)]
pub struct CacheHelperError {
    pub kind: CacheHelperErrorType,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl CacheHelperError {
    #[must_use]
    pub const fn kind(&self) -> CacheHelperErrorType {
        self.kind
    }

    #[must_use]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    #[must_use]
    pub fn into_parts(self) -> (CacheHelperErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    pub(super) fn model_not_found() -> Self {
        Self {
            kind: CacheHelperErrorType::ModelNotFound,
            source: None,
        }
    }
}

impl Display for CacheHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            CacheHelperErrorType::DeserializeBody => f.write_str("failed to deserialize the body"),
            CacheHelperErrorType::Http => f.write_str("an http error occurred"),
            CacheHelperErrorType::ModelNotFound => f.write_str("the requested model was not found"),
        }
    }
}

impl Error for CacheHelperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

impl From<DeserializeBodyError> for CacheHelperError {
    fn from(err: DeserializeBodyError) -> Self {
        Self {
            kind: CacheHelperErrorType::DeserializeBody,
            source: Some(Box::new(err)),
        }
    }
}

impl From<HttpError> for CacheHelperError {
    fn from(err: HttpError) -> Self {
        Self {
            kind: CacheHelperErrorType::Http,
            source: Some(Box::new(err)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum CacheHelperErrorType {
    DeserializeBody,
    Http,
    ModelNotFound,
}
