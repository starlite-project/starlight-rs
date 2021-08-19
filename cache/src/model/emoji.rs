use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, RoleId},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachedEmoji {
    pub id: EmojiId,
    pub animated: bool,
    pub name: String,
    pub managed: bool,
    pub require_colons: bool,
    pub roles: Vec<RoleId>,
    pub available: bool,
}

impl PartialEq<Emoji> for CachedEmoji {
    fn eq(&self, other: &Emoji) -> bool {
        self.id == other.id
            && self.animated == other.animated
            && self.managed == other.managed
            && self.name == other.name
            && self.require_colons == other.require_colons
            && self.roles == other.roles
            && self.available == other.available
    }
}

impl From<Emoji> for CachedEmoji {
    fn from(value: Emoji) -> Self {
        Self {
            id: value.id,
            animated: value.animated,
            managed: value.managed,
            name: value.name,
            require_colons: value.require_colons,
            roles: value.roles,
            available: value.available,
        }
    }
}

impl From<CachedEmoji> for Emoji {
    fn from(value: CachedEmoji) -> Self {
        Self {
            id: value.id,
            animated: value.animated,
            managed: value.managed,
            name: value.name,
            require_colons: value.require_colons,
            roles: value.roles,
            user: None,
            available: value.available,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CachedEmoji;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::{
        guild::Emoji,
        id::{EmojiId, RoleId},
    };

    assert_fields!(
        CachedEmoji: id,
        animated,
        name,
        managed,
        require_colons,
        roles
    );

    assert_impl_all!(
        CachedEmoji: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Serialize
    );

    #[test]
    fn eq_emoji() {
        let emoji = Emoji {
            id: EmojiId(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![],
            user: None,
            available: true,
        };
        let cached = CachedEmoji {
            id: EmojiId(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![],
            available: true,
        };

        assert_eq!(cached, emoji);
    }

    #[test]
    fn from_emoji() {
        let emoji = Emoji {
            id: EmojiId(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![RoleId(123)],
            available: true,
            user: None,
        };

        assert_eq!(
            CachedEmoji::from(emoji),
            CachedEmoji {
                id: EmojiId(123),
                animated: true,
                name: "foo".to_owned(),
                managed: false,
                require_colons: true,
                roles: vec![RoleId(123)],
                available: true
            }
        );
    }

    #[test]
    fn into_emoji() {
        let cached_emoji = CachedEmoji {
            id: EmojiId(123),
            animated: true,
            name: "foo".to_owned(),
            managed: false,
            require_colons: true,
            roles: vec![],
            available: true,
        };

        let emoji: Emoji = cached_emoji.into();

        assert_eq!(
            emoji,
            Emoji {
                id: EmojiId(123),
                animated: true,
                name: "foo".to_owned(),
                managed: false,
                require_colons: true,
                roles: vec![],
                user: None,
                available: true,
            }
        )
    }
}
