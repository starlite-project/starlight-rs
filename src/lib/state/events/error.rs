use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use star_error::StarError;

#[derive(Debug)]
pub struct EventError {
    pub kind: EventErrorType,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

#[derive(Debug, Clone)]
pub enum EventErrorType {
    EventFailed { message: String },
}

impl StarError for EventError {
    type Kind = EventErrorType;

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

impl Display for EventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            EventErrorType::EventFailed { ref message } => {
                f.write_str("event failed with message: ")?;
                Display::fmt(message, f)
            }
        }
    }
}

impl Error for EventError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}
