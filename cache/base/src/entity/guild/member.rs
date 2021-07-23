use super::role::RoleEntity;
use crate::{
    repository::{GetEntityFuture, ListEntitiesFuture},
    utils, Backend, Entity, Repository,
};
use twilight_model::{
    gateway::payload::MemberUpdate,
    guild::Member,
    id::{GuildId, RoleId, UserId},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl PartialEq<Member> for MemberEntity {
    fn eq(&self, other: &Member) -> bool {
        (
            self.deaf,
            self.joined_at.as_ref(),
            self.mute,
            &self.nick,
            self.pending,
            self.premium_since.as_ref(),
            &self.role_ids,
            self.user_id,
        ) == (
            other.deaf,
            other.joined_at.as_ref(),
            other.mute,
            &other.nick,
            other.pending,
            other.premium_since.as_ref(),
            &other.roles,
            self.user_id,
        )
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
