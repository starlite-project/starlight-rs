use twilight_model::application::interaction::application_command::CommandOptionValue;

pub trait CommandParse<T> {
	// to avoid naming conflicts with anything else that could use parse..
	fn parse_option(self) -> Option<T>;
}

impl CommandParse<String> for CommandOptionValue {
	fn parse_option(self) -> Option<String> {
		if let Self::String(v) = self {
			Some(v)
		} else {
			None
		}
	}
}

impl CommandParse<i64> for CommandOptionValue {
	fn parse_option(self) -> Option<i64> {
		if let Self::Integer(v) = self {
			Some(v)
		} else {
			None
		}
	}
}
