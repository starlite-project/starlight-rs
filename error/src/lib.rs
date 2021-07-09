#![warn(clippy::pedantic, clippy::nursery)]
use std::error::Error;

pub trait StarError: Error {
    type Kind;

    #[must_use = "retrieving the kind has no effect if left unused"]
    fn kind(&self) -> Self::Kind;

    #[must_use = "consuming the error into its source has no effect if left unused"]
    fn into_source(self) -> Option<Box<dyn Error + Send + Sync>>;

    #[must_use = "consuming the error into its parts has no effect if left unused"]
    fn into_parts(self) -> (Self::Kind, Option<Box<dyn Error + Send + Sync>>);
}
