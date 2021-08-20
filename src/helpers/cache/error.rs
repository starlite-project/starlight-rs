use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
pub struct CacheHelperError;

impl Display for CacheHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("An error occurred getting the item from cache (this shouldn't happen)")
    }
}

impl Error for CacheHelperError {}
