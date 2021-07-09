use crate::{
    repository::{ListEntitiesFuture, ListEntityIdsFuture, SingleEntityRepository},
    utils, Backend, Entity,
};
use serde::{Deserialize, Serialize};
use twilight_model::{
    id::{GuildId, UserId},
    user::{CurrentUser, PremiumType, UserFlags},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentUserEntity {
    pub avatar: Option<String>,
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub id: UserId,
    pub mfa_enabled: bool,
    pub name: String,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
    pub verified: Option<bool>,
}

impl From<CurrentUser> for CurrentUserEntity {
    fn from(user: CurrentUser) -> Self {
        Self {
            avatar: user.avatar,
            bot: user.bot,
            discriminator: user.discriminator,
            email: user.email,
            flags: user.flags,
            id: user.id,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type,
            public_flags: user.public_flags,
            verified: user.verified,
        }
    }
}

impl Entity for CurrentUserEntity {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait CurrentUserRepository<B: Backend>: SingleEntityRepository<CurrentUserEntity, B> {
    fn guild_ids(&self) -> ListEntityIdsFuture<'_, GuildId, B::Error>;
}
