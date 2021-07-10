use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, GuildId, RoleId, UserId},
};

use crate::{
    entity::user::UserEntity,
    repository::{GetEntityFuture, ListEntitiesFuture},
    utils, Backend, Entity, Repository,
};

use super::{role::RoleEntity, GuildEntity};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
