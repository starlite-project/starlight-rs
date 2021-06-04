use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::{
    channel::{
        CategoryChannel, Channel, Group, GuildChannel, PrivateChannel, TextChannel, VoiceChannel,
    },
    guild::{Emoji, Member, Role},
    id::{ChannelId, EmojiId, RoleId, UserId},
    user::{CurrentUser, User},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MentionFormat<T>(T);

impl Display for MentionFormat<ChannelId> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!("<#{}>", self.0))
    }
}

impl Display for MentionFormat<EmojiId> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!("<:emoji:{}>", self.0))
    }
}

impl Display for MentionFormat<RoleId> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!("<@&{}>", self.0))
    }
}

impl Display for MentionFormat<UserId> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!("<@{}>", self.0))
    }
}

impl<T, M: Mention<T>> Mention<T> for &'_ M {
    fn mention(&self) -> MentionFormat<T> {
        (*self).mention()
    }
}

pub trait Mention<T = Self> {
    fn mention(&self) -> MentionFormat<T>;
}

impl Mention for ChannelId {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(*self)
    }
}

impl Mention<ChannelId> for CategoryChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

impl Mention<ChannelId> for Channel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id())
    }
}

impl Mention<UserId> for CurrentUser {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

impl Mention for EmojiId {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(*self)
    }
}

impl Mention<EmojiId> for Emoji {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(self.id)
    }
}

impl Mention<ChannelId> for Group {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

impl Mention<ChannelId> for GuildChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id())
    }
}

impl Mention<UserId> for Member {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.user.id)
    }
}

impl Mention<ChannelId> for PrivateChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

impl Mention for RoleId {
    fn mention(&self) -> MentionFormat<RoleId> {
        MentionFormat(*self)
    }
}

impl Mention<RoleId> for Role {
    fn mention(&self) -> MentionFormat<RoleId> {
        MentionFormat(self.id)
    }
}

impl Mention<ChannelId> for TextChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

impl Mention for UserId {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(*self)
    }
}

impl Mention<UserId> for User {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

impl Mention<ChannelId> for VoiceChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::Mention;
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    #[test]
    fn mention_format_channel_id() {
        assert_eq!("<#123>", ChannelId(123).mention().to_string());
    }

    #[test]
    fn mention_format_emoji_id() {
        assert_eq!("<:emoji:123>", EmojiId(123).mention().to_string());
    }

    #[test]
    fn mention_format_role_id() {
        assert_eq!("<@&123>", RoleId(123).mention().to_string());
    }

    #[test]
    fn mention_format_user_id() {
        assert_eq!("<@123>", UserId(123).mention().to_string());
    }
}
