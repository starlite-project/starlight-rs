use serde_json::Error as JsonError;
use star_error::StarError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IoError,
};

#[derive(Debug)]
pub struct LanguageError {
    pub(crate) kind: LanguageErrorType,
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl StarError for LanguageError {
    type Kind = LanguageErrorType;

    fn kind(&self) -> Self::Kind {
        self.kind.clone()
    }

    fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    fn into_parts(self) -> (Self::Kind, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for LanguageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            LanguageErrorType::Io => f.write_str("an IO error occured"),
            LanguageErrorType::Deser => f.write_str("a deserialization error occured"),
            LanguageErrorType::InvalidParams { expected, found } => {
                f.write_str("invalid number of params were provided, expected ")?;
                Display::fmt(expected, f)?;
                f.write_str(" found ")?;
                Display::fmt(found, f)
            }
            LanguageErrorType::DirectoryFound => f.write_str("an invalid file type was found"),
            LanguageErrorType::LanguageNotFound(lang) => {
                f.write_str("the language ")?;
                Display::fmt(lang, f)?;
                f.write_str(" was not found")
            }
            LanguageErrorType::EntryNotFound(lang, entry) => {
                f.write_str("the entry ")?;
                Display::fmt(entry, f)?;
                f.write_str(" was not found in lang ")?;
                Display::fmt(lang, f)
            }
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

impl From<(usize, usize)> for LanguageError {
    fn from((expected, found): (usize, usize)) -> Self {
        Self {
            kind: LanguageErrorType::InvalidParams { expected, found },
            source: None,
        }
    }
}

impl From<String> for LanguageError {
    fn from(lang: String) -> Self {
        Self {
            kind: LanguageErrorType::LanguageNotFound(lang),
            source: None,
        }
    }
}

impl From<(String, String)> for LanguageError {
    fn from((lang, entry): (String, String)) -> Self {
        Self {
            kind: LanguageErrorType::EntryNotFound(lang, entry),
            source: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LanguageErrorType {
    Io,
    Deser,
    InvalidParams { expected: usize, found: usize },
    DirectoryFound,
    LanguageNotFound(String),
    EntryNotFound(String, String),
}

#[cfg(test)]
mod tests {
    use super::{LanguageError, LanguageErrorType};
    use star_error::StarError;
    use static_assertions::assert_impl_all;
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    assert_impl_all!(LanguageError: Debug, Display, Error, StarError);
    assert_impl_all!(LanguageErrorType: Clone, Debug);

    #[test]
    fn kind() {
        let err = LanguageError {
            kind: LanguageErrorType::Io,
            source: None,
        };

        assert_eq!(err.kind(), LanguageErrorType::Io);
    }
}
