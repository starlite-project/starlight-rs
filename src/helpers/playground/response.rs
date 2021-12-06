use serde::{Deserialize, Deserializer, Serialize};

use super::extract_relevant_lines;

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

impl PlaygroundResponse {
	pub fn format(&mut self, show_compiler_warnings: bool) {
		let compiler_output = extract_relevant_lines(
			&self.stderr,
			&["Compiling playground"],
			&[
				"warning emitted",
				"warnings emitted",
				"warning: `playground` (bin \"playground\") generated",
				"error: could not compile",
				"error: aborting",
				"Finished ",
			],
		);

		let output = if self.stderr.contains("Running `target") {
			let program_stderr = extract_relevant_lines(&self.stderr, &["Running `target"], &[]);

			if show_compiler_warnings {
				match (compiler_output, program_stderr) {
					("", "") => String::new(),
					(warnings, "") => warnings.to_owned(),
					("", stderr) => stderr.to_owned(),
					(warnings, stderr) => [warnings, stderr].join("\n"),
				}
			} else {
				program_stderr.to_owned()
			}
		} else {
			compiler_output.to_owned()
		};

		self.stderr = output;
	}
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
