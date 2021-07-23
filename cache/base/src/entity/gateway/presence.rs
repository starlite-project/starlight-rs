use crate::{Backend, Entity, Repository};
use twilight_model::{
    gateway::{
        payload::PresenceUpdate,
        presence::{Activity, ClientStatus, Presence, Status, UserOrId},
    },
    id::{GuildId, UserId},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceEntity {
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub guild_id: GuildId,
    pub status: Status,
    pub user_id: UserId,
}

impl From<Presence> for PresenceEntity {
    fn from(presence: Presence) -> Self {
        Self {
            activities: presence.activities,
            client_status: presence.client_status,
            guild_id: presence.guild_id,
            status: presence.status,
            user_id: get_user_id(&presence.user),
        }
    }
}

impl From<PresenceUpdate> for PresenceEntity {
    fn from(mut presence: PresenceUpdate) -> Self {
        let mut activities = Vec::new();

        if let Some(game) = presence.game {
            activities.push(game);
        }

        activities.append(&mut presence.activities);

        Self {
            activities,
            client_status: presence.client_status,
            guild_id: presence.guild_id,
            status: presence.status,
            user_id: get_user_id(&presence.user),
        }
    }
}

impl PartialEq<Presence> for PresenceEntity {
    fn eq(&self, other: &Presence) -> bool {
        (
            &self.activities,
            &self.client_status,
            self.guild_id,
            self.status,
            self.user_id,
        ) == (
            &other.activities,
            &other.client_status,
            other.guild_id,
            other.status,
            get_user_id(&other.user),
        )
    }
}

impl Entity for PresenceEntity {
    type Id = (GuildId, UserId);

    fn id(&self) -> Self::Id {
        (self.guild_id, self.user_id)
    }
}

pub trait PresenceRepository<B: Backend>: Repository<PresenceEntity, B> {}

#[inline]
const fn get_user_id(user_or_id: &UserOrId) -> UserId {
    match user_or_id {
        UserOrId::User(user) => user.id,
        UserOrId::UserId { id } => *id,
    }
}
