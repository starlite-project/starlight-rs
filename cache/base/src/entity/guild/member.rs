use serde::{Deserialize, Serialize};
use twilight_model::{
    gateway::payload::MemberUpdate,
    guild::Member,
    id::{GuildId, RoleId, UserId},
};

use crate::{
    repository::{GetEntityFuture, ListEntitiesFuture},
    utils, Backend, Entity, Repository,
};

use super::role::RoleEntity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberEntity {
    pub deaf: bool,
    pub guild_id: GuildId,
    pub hoisted_role_id: Option<RoleId>,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<String>,
    pub role_ids: Vec<RoleId>,
    pub user_id: UserId,
}

impl From<Member> for MemberEntity {
    fn from(member: Member) -> Self {
        Self {
            deaf: member.deaf,
            guild_id: member.guild_id,
            hoisted_role_id: member.hoisted_role,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick,
            pending: member.pending,
            premium_since: member.premium_since,
            role_ids: member.roles,
            user_id: member.user.id,
        }
    }
}

impl MemberEntity {
    #[must_use]
    pub fn update(self, update: MemberUpdate) -> Self {
        Self {
            guild_id: update.guild_id,
            joined_at: Some(update.joined_at),
            nick: update.nick.or(self.nick),
            premium_since: update.premium_since.or(self.premium_since),
            role_ids: update.roles,
            user_id: update.user.id,
            ..self
        }
    }
}

impl Entity for MemberEntity {
    type Id = (GuildId, UserId);

    fn id(&self) -> Self::Id {
        (self.guild_id, self.user_id)
    }
}

pub trait MemberRepository<B: Backend>: Repository<MemberEntity, B> {
    fn hoisted_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> GetEntityFuture<'_, RoleEntity, B::Error> {
        utils::relation_and_then(
            self.backend().members(),
            self.backend().roles(),
            (guild_id, user_id),
            |member| member.hoisted_role_id,
        )
    }

    fn roles(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        utils::stream(
            self.backend().members(),
            self.backend().roles(),
            (guild_id, user_id),
            |member| member.role_ids.into_iter(),
        )
    }
}
