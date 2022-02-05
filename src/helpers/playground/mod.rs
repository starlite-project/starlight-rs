use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	str::FromStr,
};

use crate::prelude::*;

mod request;
mod response;
mod util;

pub use self::{request::*, response::*, util::*};

#[derive(Debug, Default, Error, Clone, Copy)]
#[error("an invalid type was given")]
pub struct InvalidTypeError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RustChannel {
	Stable,
	Beta,
	Nightly,
}

impl Default for RustChannel {
	fn default() -> Self {
		Self::Nightly
	}
}

impl Display for RustChannel {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Stable => f.write_str("stable"),
			Self::Beta => f.write_str("beta"),
			Self::Nightly => f.write_str("nightly"),
		}
	}
}

impl FromStr for RustChannel {
	type Err = InvalidTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"stable" => Ok(Self::Stable),
			"beta" => Ok(Self::Beta),
			"nightly" => Ok(Self::Nightly),
			_ => Err(InvalidTypeError::default()),
		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Edition {
	#[serde(rename = "2015")]
	E2015,
	#[serde(rename = "2018")]
	E2018,
	#[serde(rename = "2021")]
	E2021,
}

impl Default for Edition {
	fn default() -> Self {
		Self::E2018
	}
}

impl Display for Edition {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::E2015 => f.write_str("2015"),
			Self::E2018 => f.write_str("2018"),
			Self::E2021 => f.write_str("2021"),
		}
	}
}

impl FromStr for Edition {
	type Err = InvalidTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"2015" => Ok(Self::E2015),
			"2018" => Ok(Self::E2018),
			"2021" => Ok(Self::E2021),
			_ => Err(InvalidTypeError::default()),
		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CrateType {
	#[serde(rename = "bin")]
	Binary,
	#[serde(rename = "lib")]
	Library,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildMode {
	Debug,
	Release,
}

impl Default for BuildMode {
	fn default() -> Self {
		Self::Debug
	}
}

impl Display for BuildMode {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Debug => f.write_str("debug"),
			Self::Release => f.write_str("release"),
		}
	}
}

impl FromStr for BuildMode {
	type Err = InvalidTypeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"debug" => Ok(Self::Debug),
			"release" => Ok(Self::Release),
			_ => Err(InvalidTypeError::default()),
		}
	}
}
