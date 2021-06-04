mod error;
mod r#impl;
mod iter;

pub use self::{
    error::{ParseMentionError, ParseMentionErrorType},
    iter::MentionIter,
    r#impl::ParseMention,
};

use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MentionType {
    Channel(ChannelId),
    Emoji(EmojiId),
    Role(RoleId),
    User(UserId),
}

impl Display for MentionType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Channel(id) => Display::fmt(id, f),
            Self::Emoji(id) => Display::fmt(id, f),
            Self::Role(id) => Display::fmt(id, f),
            Self::User(id) => Display::fmt(id, f),
        }
    }
}
