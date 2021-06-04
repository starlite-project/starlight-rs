use super::ParseMention;
use std::{iter::Iterator, marker::PhantomData, str::CharIndices};

#[derive(Debug, Clone)]
pub struct MentionIter<'a, T> {
    buf: &'a str,
    chars: CharIndices<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T> MentionIter<'a, T> {
    #[must_use]
    pub(in crate::parse) fn new(buf: &'a str) -> Self {
        let chars = buf.char_indices();

        Self {
            buf,
            chars,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.buf
    }
}

impl<'a, T: ParseMention> Iterator for MentionIter<'a, T> {
    type Item = (T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let start = match self.chars.next()? {
                (idx, '<') => idx,
                _ => continue,
            };

            let mut found = false;

            for sigil in T::SIGILS {
                if self.chars.as_str().starts_with(sigil) {
                    found = true;

                    for _ in 0..sigil.chars().count() {
                        self.chars.next();
                    }
                }
            }

            if !found {
                continue;
            }

            let end = match self.chars.find(|c| c.1 == '>') {
                Some((idx, _)) => idx,
                None => continue,
            };

            let buf = self.buf.get(start..=end)?;

            if let Ok(id) = T::parse(buf) {
                return Some((id, start, end));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{MentionType, ParseMention};
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    #[test]
    fn iter_channel_id() {
        let mut iter = ChannelId::iter("<#123>");
        assert_eq!(ChannelId(123), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_multiple_ids() {
        let buf = "one <@123>two<#456><@789> ----";
        let mut iter = UserId::iter(buf);
        assert_eq!(UserId(123), iter.next().unwrap().0);
        let (mention, start, end) = iter.next().unwrap();
        assert_eq!(UserId(789), mention);
        assert_eq!(19, start);
        assert_eq!(24, end);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_emoji_ids() {
        let mut iter = EmojiId::iter("some <:name:123> emojis <:emoji:456>");

        assert_eq!(EmojiId(123), iter.next().unwrap().0);
        assert_eq!(EmojiId(456), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_mention_type() {
        let mut iter = MentionType::iter("<#12><:name:34><@&56><@!78><@90>");
        assert_eq!(MentionType::Channel(ChannelId(12)), iter.next().unwrap().0);
        assert_eq!(MentionType::Emoji(EmojiId(34)), iter.next().unwrap().0);
        assert_eq!(MentionType::Role(RoleId(56)), iter.next().unwrap().0);
        assert_eq!(MentionType::User(UserId(78)), iter.next().unwrap().0);
        assert_eq!(MentionType::User(UserId(90)), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_role_ids() {
        let mut iter = RoleId::iter("some <@&123> roles <@&456>");
        assert_eq!(RoleId(123), iter.next().unwrap().0);
        assert_eq!(RoleId(456), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_user_ids() {
        let mut iter = UserId::iter("some <@123>users<@456>");
        assert_eq!(UserId(123), iter.next().unwrap().0);
        assert_eq!(UserId(456), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_no_id() {
        let mention = "this is not <# actually a mention";
        let mut iter = ChannelId::iter(mention);

        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_ignores_other_types() {
        let mention = "<#123> <:name:456> <@&789>";
        let mut iter = UserId::iter(mention);

        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_as_str() {
        let buf = "a buf";
        let mut iter = RoleId::iter(buf);

        assert_eq!(buf, iter.as_str());
        assert!(iter.next().is_none());
        assert_eq!(buf, iter.as_str());
    }
}
