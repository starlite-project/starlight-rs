use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultHandling {
	None,
	Discard,
	Print,
}

impl ResultHandling {
	#[must_use]
	pub fn apply(self, code: &str) -> Cow<'_, str> {
		if code.contains("fn main") {
			return Cow::Borrowed(code);
		}

		let after_crate_attrs = match self {
			Self::None => "fn main() -> Result<(), Box<std::error::Error>> {\n",
			Self::Discard => "fn main() -> Result<(), Box<std::error::Error>> { let _ = {\n",
			Self::Print => {
				"fn main() -> Result<(), Box<std::error::Error>> { println!(\"{:?}\", {\n"
			}
		};

		let after_code = match self {
			Self::None => "\nOk(()) }",
			Self::Discard => "};\nOk(()) }",
			Self::Print => "});\nOk(()) }",
		};

		Cow::Owned(parse_attributes(code, after_crate_attrs, after_code))
	}
}

#[must_use]
pub fn parse_attributes(code: &str, after_crate_attrs: &str, after_code: &str) -> String {
	let mut lines = code.lines().peekable();

	let mut output = String::new();

	while let Some(line) = lines.peek() {
		let line = line.trim();
		if line.starts_with("#![") {
			output.push_str(line);
			output.push('\n');
		} else if line.is_empty() {
			// noop
		} else {
			break;
		}
		lines.next();
	}

	output.push_str(after_crate_attrs);

	for line in lines {
		output.push_str(line);
		output.push('\n');
	}

	output.push_str(after_code);

	output
}

pub(super) fn extract_relevant_lines<'a>(
	mut stderr: &'a str,
	strip_start_tokens: &[&str],
	strip_end_tokens: &[&str],
) -> &'a str {
	if let Some(start_token_pos) = strip_start_tokens
		.iter()
		.filter_map(|t| stderr.rfind(t))
		.max()
	{
		stderr = match stderr[start_token_pos..].find('\n') {
			Some(line_end) => &stderr[(line_end + start_token_pos + 1)..],
			None => "",
		};
	}

	if let Some(end_token_pos) = strip_end_tokens
		.iter()
		.filter_map(|t| stderr.rfind(t))
		.min()
	{
		stderr = match stderr[..end_token_pos].rfind('\n') {
			Some(prev_line_end) => &stderr[..=prev_line_end],
			None => "",
		};
	}

	stderr = stderr.trim_start_matches('\n');
	while stderr.ends_with("\n\n") {
		stderr = &stderr[..(stderr.len() - 1)];
	}

	stderr
}
