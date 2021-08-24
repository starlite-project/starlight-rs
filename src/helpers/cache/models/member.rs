use serde::{Deserialize, Serialize};
use twilight_cache_inmemory::model::CachedMember;
use twilight_model::{
	guild::Member,
	id::{GuildId, RoleId, UserId},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberHelper {
	pub deaf: Option<bool>,
	pub guild_id: GuildId,
	pub joined_at: Option<String>,
	pub mute: Option<bool>,
	pub nick: Option<String>,
	pub pending: bool,
	pub premium_since: Option<String>,
	pub roles: Vec<RoleId>,
	pub user_id: UserId,
}

impl PartialEq<Member> for MemberHelper {
	fn eq(&self, other: &Member) -> bool {
		(
			self.deaf,
			self.joined_at.as_ref(),
			self.mute,
			&self.nick,
			self.pending,
			self.premium_since.as_ref(),
			&self.roles,
			self.user_id,
		) == (
			Some(other.deaf),
			other.joined_at.as_ref(),
			Some(other.mute),
			&other.nick,
			other.pending,
			other.premium_since.as_ref(),
			&other.roles,
			self.user_id,
		)
	}
}

impl PartialEq<CachedMember> for MemberHelper {
	fn eq(&self, other: &CachedMember) -> bool {
		(
			self.deaf,
			self.joined_at.as_ref(),
			self.mute,
			&self.nick,
			self.pending,
			self.premium_since.as_ref(),
			&self.roles,
			self.user_id,
		) == (
			other.deaf,
			other.joined_at.as_ref(),
			other.mute,
			&other.nick,
			other.pending,
			other.premium_since.as_ref(),
			&other.roles,
			other.user_id,
		)
	}
}

impl From<Member> for MemberHelper {
	fn from(member: Member) -> Self {
		Self {
			deaf: Some(member.deaf),
			guild_id: member.guild_id,
			joined_at: member.joined_at,
			mute: Some(member.mute),
			nick: member.nick,
			pending: member.pending,
			premium_since: member.premium_since,
			roles: member.roles,
			user_id: member.user.id,
		}
	}
}

impl From<CachedMember> for MemberHelper {
	fn from(member: CachedMember) -> Self {
		Self {
			deaf: member.deaf,
			guild_id: member.guild_id,
			joined_at: member.joined_at,
			mute: member.mute,
			nick: member.nick,
			pending: member.pending,
			premium_since: member.premium_since,
			roles: member.roles,
			user_id: member.user_id,
		}
	}
}
