use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub struct EventError {
    pub kind: EventErrorType,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

#[derive(Debug, Clone, Copy)]
pub enum EventErrorType {
    Unknown,
}

impl EventError {
    #[must_use]
    pub const fn kind(&self) -> EventErrorType {
        self.kind
    }

    #[must_use]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    pub fn into_parts(self) -> (EventErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for EventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            EventErrorType::Unknown => {
                if let Some(source) = &self.source {
                    Display::fmt(source, f)
                } else {
                    write!(f, "Unknown error")
                }
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
