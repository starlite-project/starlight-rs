use serde_json::Error as JsonError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

#[derive(Debug)]
pub struct LanguageError {
    pub(super) kind: LanguageErrorType,
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
}

impl LanguageError {
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> LanguageErrorType {
        self.kind
    }

    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (LanguageErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for LanguageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            LanguageErrorType::Io => f.write_str("an IO error occured"),
            LanguageErrorType::Deser => f.write_str("a deserialization error occured"),
            LanguageErrorType::InvalidParams { expected, found } => {
                f.write_str("invalid number of params were provided, expected ")?;
                Display::fmt(&expected, f)?;
                f.write_str(" found ")?;
                Display::fmt(&found, f)
            }
            LanguageErrorType::DirectoryFound => f.write_str("an invalid file type was found"),
        }
    }
}

impl Error for LanguageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

impl From<IoError> for LanguageError {
    fn from(err: IoError) -> Self {
        Self {
            kind: LanguageErrorType::Io,
            source: Some(Box::new(err)),
        }
    }
}

impl From<JsonError> for LanguageError {
    fn from(err: JsonError) -> Self {
        Self {
            kind: LanguageErrorType::Deser,
            source: Some(Box::new(err)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LanguageErrorType {
    Io,
    Deser,
    InvalidParams { expected: usize, found: usize },
    DirectoryFound,
}
