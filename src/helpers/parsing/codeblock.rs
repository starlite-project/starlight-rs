use std::{fmt::Write, str::FromStr};

use crate::prelude::*;

#[derive(Debug, Error, Clone, Copy)]
pub enum CodeBlockError {
	#[error("missing code block")]
	Missing,
	#[error("malformed code block")]
	Malformed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeBlock {
	pub code: String,
	pub language: Option<String>,
	pub rest: String,
}

impl Display for CodeBlock {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("```")?;
		Display::fmt(self.language.as_deref().unwrap_or(""), f)?;
		f.write_char('\n')?;
		Display::fmt(&self.code, f)?;
		f.write_str("\n```")
	}
}

impl FromStr for CodeBlock {
	type Err = CodeBlockError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let rest;
		let mut code_block = if let Some(code_block) = s.strip_prefix("```") {
			let code_block_end = code_block.find("```").ok_or(CodeBlockError::Malformed)?;

			rest = &code_block[(code_block_end + 3)..];
			let mut code_block = &code_block[..code_block_end];

			let mut language = None;
			if let Some(first_newline) = code_block.find('\n') {
				if !code_block[..first_newline].contains(char::is_whitespace) {
					language = Some(&code_block[..first_newline]);
					code_block = &code_block[(first_newline + 1)..];
				}
			}

			let code_block = code_block.trim_start_matches('\n').trim_end_matches('\n');

			Self {
				code: code_block.to_owned(),
				language: language.map(ToOwned::to_owned),
				rest: rest.to_owned(),
			}
		} else if let Some(code_line) = s.strip_prefix('`') {
			let code_line_end = code_line.find('`').ok_or(CodeBlockError::Malformed)?;

			rest = &code_line[(code_line_end + 1)..];
			let code_line = &code_line[..code_line_end];

			Self {
				code: code_line.to_owned(),
				language: None,
				rest: rest.to_owned(),
			}
		} else {
			return Err(CodeBlockError::Missing);
		};

		if code_block.code.is_empty() {
			Err(CodeBlockError::Malformed)
		} else {
			code_block.code = code_block.code.trim_end_matches('\u{200a}').to_owned();

			Ok(code_block)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{CodeBlock, CodeBlockError};

	#[test]
	fn test_parse() -> Result<(), CodeBlockError> {
		let message = "```js\nconst value = 0;\n```\n\nhello, world!".to_owned();

		let code_block: CodeBlock = message.parse()?;

		assert_eq!(
			code_block,
			CodeBlock {
				code: "const value = 0;".to_owned(),
				language: Some("js".into()),
				rest: "\n\nhello, world!".to_owned(),
			}
		);

		Ok(())
	}
}
