use serde::{Deserialize, Serialize};

use super::{BuildMode, CrateType, Edition, RustChannel};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlaygroundRequest<'a> {
	pub channel: RustChannel,
	pub edition: Edition,
	pub code: &'a str,
	#[serde(rename = "crateType")]
	pub crate_type: CrateType,
	pub mode: BuildMode,
	pub tests: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MiriRequest<'a> {
	pub edition: Edition,
	pub code: &'a str,
}

pub type MacroExpansionRequest<'a> = MiriRequest<'a>;

pub type FormatRequest<'a> = MiriRequest<'a>;
