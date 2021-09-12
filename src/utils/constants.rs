use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy)]
pub enum SlashiesErrorMessages {
    GuildOnly,
    CantGetUser
}

impl Display for SlashiesErrorMessages {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::GuildOnly => f.write_str("This command can only be used in a guild"),
            Self::CantGetUser => f.write_str("An error occurred getting the user"),
        }
    }
}