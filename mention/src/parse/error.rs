use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub struct ParseMentionError<'a> {
    pub(super) kind: ParseMentionErrorType<'a>,
    pub(super) source: Option<Box<dyn Error + Send + Sync>>,
}

impl<'a> ParseMentionError<'a> {
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ParseMentionErrorType<'_> {
        &self.kind
    }

    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    #[must_use = "conssuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ParseMentionErrorType<'a>,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl Display for ParseMentionError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ParseMentionErrorType::IdNotU64 { found, .. } => f.write_fmt(format_args!(
                "id portion ('{}') of mention is not a u64",
                found
            )),
            ParseMentionErrorType::LeadingArrow { found } => {
                f.write_str("expected to find a leading arrow ('<') but instead ")?;
                if let Some(c) = found {
                    f.write_fmt(format_args!("found '{}'", c))
                } else {
                    f.write_str("found nothing")
                }
            }
            ParseMentionErrorType::PartMissing { expected, found } => f.write_fmt(format_args!(
                "expected {} parts but only found {}",
                expected, found
            )),
            ParseMentionErrorType::Sigil { expected, found } => {
                f.write_str("expected to find a mention sigil (")?;

                for (idx, sigil) in expected.iter().enumerate() {
                    f.write_fmt(format_args!("'{}'", sigil))?;

                    if idx < expected.len() - 1 {
                        f.write_str(", ")?;
                    }
                }

                f.write_str(") but instead found ")?;

                if let Some(c) = found {
                    f.write_fmt(format_args!("'{}'", c))
                } else {
                    f.write_str("nothing")
                }
            }
            ParseMentionErrorType::TrailingArrow {found} => {
                f.write_str("expected to find a trailing arrow ('>') but instead ")?;

                if let Some(c) = found {
                    f.write_fmt(format_args!("found '{}'", c))
                } else {
                    f.write_str("found nothing")
                }
            }
        }
    }
}

impl Error for ParseMentionError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)>{
        self.source
        .as_ref()
        .map(|source| &**source as &(dyn Error + 'static))
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseMentionErrorType<'a> {
    IdNotU64 {
        found: &'a str,
    },
    LeadingArrow {
        found: Option<char>,
    },
    PartMissing {
        expected: usize,
        found: usize,
    },
    Sigil {
        expected: &'a [&'a str],
        found: Option<char>,
    },
    TrailingArrow {
        found: Option<char>,
    },
}
