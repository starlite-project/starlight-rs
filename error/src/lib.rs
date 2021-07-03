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

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use super::StarError;
    use std::{
        error::Error,
        fmt::{Display, Formatter, Result as FmtResult},
    };

    #[derive(Debug, Clone, Copy)]
    struct NullError;

    impl Display for NullError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("null error")
        }
    }

    impl Error for NullError {}

    #[derive(Debug, Clone, PartialEq)]
    enum TestErrorType {
        Blank,
        Message(String),
    }

    impl Default for TestErrorType {
        fn default() -> Self {
            Self::Blank
        }
    }

    #[derive(Debug, Default)]
    struct TestError {
        kind: TestErrorType,
        source: Option<Box<dyn Error + Send + Sync>>,
    }

    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match &self.kind {
                TestErrorType::Blank => f.write_str("blank error"),
                TestErrorType::Message(msg) => {
                    f.write_str("error message")?;
                    Display::fmt(msg, f)
                }
            }
        }
    }

    impl Error for TestError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source
                .as_ref()
                .map(|source| &**source as &(dyn Error + 'static))
        }
    }

    impl From<NullError> for TestError {
        fn from(err: NullError) -> Self {
            Self {
                kind: TestErrorType::Blank,
                source: Some(Box::new(err)),
            }
        }
    }

    impl StarError for TestError {
        type Kind = TestErrorType;

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

    #[test]
    fn default() {
        let default = TestError::default();

        assert_eq!(default.kind(), TestErrorType::Blank);
    }
}
