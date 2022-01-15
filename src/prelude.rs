// any `Result` or `Error` types should be prefixed with their respective module name.
// any traits that are only used for their methods should be renamed to _
pub use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	io::Result as IoResult,
};


pub use miette::{miette as error, IntoDiagnostic as _, Result, WrapErr as _};
pub use reqwest::header;
pub use serde::{Deserialize, Serialize};
#[cfg(not(debug_assertions))]
pub use starchart::backend::TomlBackend;
#[cfg(debug_assertions)]
pub use starchart::backend::TomlPrettyBackend as TomlBackend;
pub use thiserror::Error;
pub use tracing::{event, instrument, Level};
pub use twilight_http::Error as HttpError;

pub use crate::state::{Context, QuickAccess};
