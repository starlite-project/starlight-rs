use super::{MentionIter, MentionType, ParseMentionError, ParseMentionErrorType};
use std::str::Chars;
use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

pub trait ParseMention: private::Sealed {
    const SIGILS: &'static [&'static str];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized;

    #[must_use = "you must use the iterator to lazily"]
    fn iter(buf: &str) -> MentionIter<'_, Self>
    where
        Self: Sized,
    {
        MentionIter::new(buf)
    }
}

impl ParseMention for ChannelId {
    const SIGILS: &'static [&'static str] = &["#"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS).map(|(id, _)| ChannelId(id))
    }
}

impl ParseMention for EmojiId {
    const SIGILS: &'static [&'static str] = &[":"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS).map(|(id, _)| EmojiId(id))
    }
}

impl ParseMention for MentionType {
    const SIGILS: &'static [&'static str] = &["#", ":", "@&", "@!", "@"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        let (id, found) = parse_id(buf, Self::SIGILS)?;

        for sigil in ChannelId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::Channel(ChannelId(id)));
            }
        }

        for sigil in EmojiId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::Emoji(EmojiId(id)));
            }
        }

        for sigil in RoleId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::Role(RoleId(id)));
            }
        }

        for sigil in UserId::SIGILS {
            if *sigil == found {
                return Ok(MentionType::User(UserId(id)));
            }
        }

        unreachable!("mention type must have been found");
    }
}

impl ParseMention for RoleId {
    const SIGILS: &'static [&'static str] = &["@&"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS).map(|(id, _)| RoleId(id))
    }
}

impl ParseMention for UserId {
    const SIGILS: &'static [&'static str] = &["@!", "@"];

    fn parse(buf: &str) -> Result<Self, ParseMentionError<'_>>
    where
        Self: Sized,
    {
        parse_id(buf, Self::SIGILS).map(|(id, _)| UserId(id))
    }
}

fn parse_id<'a>(
    buf: &'a str,
    sigils: &'a [&'a str],
) -> Result<(u64, &'a str), ParseMentionError<'a>> {
    let mut chars = buf.chars();

    let c = chars.next();

    if c.map_or(true, |c| c != '<') {
        return Err(ParseMentionError {
            kind: ParseMentionErrorType::LeadingArrow { found: c },
            source: None,
        });
    }

    let maybe_sigil = sigils.iter().find(|sigil| {
        if chars.as_str().starts_with(*sigil) {
            for _ in 0..sigil.chars().count() {
                chars.next();
            }

            return true;
        }

        false
    });

    let sigil = if let Some(sigil) = maybe_sigil {
        *sigil
    } else {
        return Err(ParseMentionError {
            kind: ParseMentionErrorType::Sigil {
                expected: sigils,
                found: chars.next(),
            },
            source: None,
        });
    };

    if sigil == ":" && !emoji_sigil_present(&mut chars) {
        return Err(ParseMentionError {
            kind: ParseMentionErrorType::PartMissing {
                found: 1,
                expected: 2,
            },
            source: None,
        });
    }

    let remaining = chars
        .as_str()
        .find('>')
        .and_then(|idx| chars.as_str().get(..idx))
        .ok_or(ParseMentionError {
            kind: ParseMentionErrorType::TrailingArrow { found: None },
            source: None,
        })?;

    remaining
        .parse()
        .map(|id| (id, sigil))
        .map_err(|source| ParseMentionError {
            kind: ParseMentionErrorType::IdNotU64 { found: remaining },
            source: Some(Box::new(source)),
        })
}

fn emoji_sigil_present(chars: &mut Chars<'_>) -> bool {
    for c in chars {
        if c == ':' {
            return true;
        }
    }

    false
}

mod private {
    use super::super::MentionType;
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    pub trait Sealed {}

    impl Sealed for ChannelId {}
    impl Sealed for EmojiId {}
    impl Sealed for MentionType {}
    impl Sealed for RoleId {}
    impl Sealed for UserId {}
}

#[cfg(test)]
mod tests {
    use super::ParseMention;
    use crate::parse::{MentionType, ParseMentionErrorType};
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    #[test]
    fn sigils() {
        assert_eq!(&["#"], ChannelId::SIGILS);
        assert_eq!(&[":"], EmojiId::SIGILS);
        assert_eq!(&["#", ":", "@&", "@!", "@"], MentionType::SIGILS);
        assert_eq!(&["@&"], RoleId::SIGILS);
        assert_eq!(&["@!", "@"], UserId::SIGILS);
    }

    #[test]
    fn parse_channel_id() {
        assert_eq!(ChannelId(123), ChannelId::parse("<#123>").unwrap());

        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["#"],
                found: Some('@'),
            },
            ChannelId::parse("<@123>").unwrap_err().kind()
        );
    }

    #[test]
    fn parse_emoji_id() {
        assert_eq!(EmojiId(123), EmojiId::parse("<:name:123>").unwrap());

        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &[":"],
                found: Some('@')
            },
            EmojiId::parse("<@123>").unwrap_err().kind()
        );
    }

    #[test]
    fn parse_mention_type() {
        assert_eq!(
            MentionType::Channel(ChannelId(123)),
            MentionType::parse("<#123>").unwrap()
        );

        assert_eq!(
            MentionType::Emoji(EmojiId(123)),
            MentionType::parse("<:name:123>").unwrap()
        );

        assert_eq!(
            MentionType::Role(RoleId(123)),
            MentionType::parse("<@&123>").unwrap()
        );

        assert_eq!(
            MentionType::User(UserId(123)),
            MentionType::parse("<@123>").unwrap()
        );

        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["#", ":", "@&", "@!", "@"],
                found: Some(';')
            },
            MentionType::parse("<;123>").unwrap_err().kind()
        );
    }

    #[test]
    fn parse_role_id() {
        assert_eq!(RoleId(123), RoleId::parse("<@&123>").unwrap());

        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["@&"],
                found: Some('@')
            },
            RoleId::parse("<@123>").unwrap_err().kind()
        );
    }

    #[test]
    fn parse_user_id() {
        assert_eq!(UserId(123), UserId::parse("<@123>").unwrap());

        assert_eq!(
            &ParseMentionErrorType::IdNotU64 { found: "&123" },
            UserId::parse("<@&123>").unwrap_err().kind()
        );
    }

    #[test]
    fn parse_id_wrong_sigil() {
        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["@"],
                found: Some('#')
            },
            super::parse_id("<#123>", &["@"]).unwrap_err().kind()
        );

        assert_eq!(
            &ParseMentionErrorType::Sigil {
                expected: &["#"],
                found: None
            },
            super::parse_id("<", &["#"]).unwrap_err().kind()
        );
    }
}
