use super::{role::RoleEntity, GuildEntity};
use crate::{
    entity::user::UserEntity,
    repository::{GetEntityFuture, ListEntitiesFuture},
    utils, Backend, Entity, Repository,
};
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, GuildId, RoleId, UserId},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmojiEntity {
    pub animated: bool,
    pub available: bool,
    pub guild_id: GuildId,
    pub id: EmojiId,
    pub managed: bool,
    pub name: String,
    pub require_colons: bool,
    pub role_ids: Vec<RoleId>,
    pub user_id: Option<UserId>,
}

impl From<(GuildId, Emoji)> for EmojiEntity {
    fn from((guild_id, emoji): (GuildId, Emoji)) -> Self {
        let user_id = emoji.user.map(|user| user.id);

        Self {
            animated: emoji.animated,
            available: emoji.available,
            guild_id,
            id: emoji.id,
            managed: emoji.managed,
            name: emoji.name,
            require_colons: emoji.require_colons,
            role_ids: emoji.roles,
            user_id,
        }
    }
}

impl PartialEq<Emoji> for EmojiEntity {
    fn eq(&self, other: &Emoji) -> bool {
        self.id == other.id
            && self.animated == other.animated
            && self.name == other.name
            && self.require_colons == other.require_colons
            && self.role_ids == other.roles
            && self.user_id == other.user.as_ref().map(|user| user.id)
            && self.available == other.available
    }
}

impl Entity for EmojiEntity {
    type Id = EmojiId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait EmojiRepository<B: Backend>: Repository<EmojiEntity, B> {
    fn guild(&self, emoji_id: EmojiId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        utils::relation_map(
            self.backend().emojis(),
            self.backend().guilds(),
            emoji_id,
            |emoji| emoji.guild_id,
        )
    }

    fn roles(&self, emoji_id: EmojiId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        utils::stream(
            self.backend().emojis(),
            self.backend().roles(),
            emoji_id,
            |emoji| emoji.role_ids.into_iter(),
        )
    }

    fn user(&self, emoji_id: EmojiId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        utils::relation_and_then(
            self.backend().emojis(),
            self.backend().users(),
            emoji_id,
            |emoji| emoji.user_id,
        )
    }
}
