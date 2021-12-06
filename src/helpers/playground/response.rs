use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatResponse {
	pub success: bool,
	pub code: String,
	pub stdout: String,
	pub stderr: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaygroundResponse {
	pub success: bool,
	pub stdout: String,
	pub stderr: String,
}

impl<'de> Deserialize<'de> for PlaygroundResponse {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		pub enum RawPlayResponse {
			Err {
				error: String,
			},
			Ok {
				success: bool,
				stdout: String,
				stderr: String,
			},
		}

		Ok(match RawPlayResponse::deserialize(deserializer)? {
			RawPlayResponse::Ok {
				success,
				stdout,
				stderr,
			} => Self {
				success,
				stdout,
				stderr,
			},
			RawPlayResponse::Err { error } => Self {
				success: false,
				stdout: String::new(),
				stderr: error,
			},
		})
	}
}
