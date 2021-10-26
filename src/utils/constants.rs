use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashiesErrorMessages {
	Unknown,
	GuildOnly,
	CantGetUser,
	InteractionError
}

impl Default for SlashiesErrorMessages {
	fn default() -> Self {
		Self::Unknown
	}
}

impl Display for SlashiesErrorMessages {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Unknown => f.write_str("An unknown error has occurred"),
			Self::GuildOnly => f.write_str("This command can only be used in a guild"),
			Self::CantGetUser => f.write_str("An error occurred getting the user"),
			Self::InteractionError => f.write_str("An error occurred during the interaction")
		}
	}
}
