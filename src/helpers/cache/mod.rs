#![allow(dead_code)]

use crate::state::State;
use std::result::Result as StdResult;
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;
use twilight_model::{
    channel::{Channel, Group, GuildChannel, PrivateChannel},
    guild::Role,
    id::{ChannelId, EmojiId, GuildId, RoleId, UserId},
    user::{CurrentUser, User},
};

mod error;
mod models;

pub use self::{
    error::CacheHelperError,
    models::{EmojiHelper, MemberHelper},
};

pub type Result<T> = StdResult<T, CacheHelperError>;

#[derive(Debug)]
pub struct CacheHelper<'a> {
    state: &'a State,
}

impl<'a> CacheHelper<'a> {
    #[must_use = "a CacheHelper does nothing on it's own"]
    pub const fn new(state: &'a State) -> Self {
        Self { state }
    }

    #[must_use]
    pub fn cache(&self) -> &InMemoryCache {
        &self.state.cache
    }

    #[must_use]
    pub fn http(&self) -> &Client {
        &self.state.http
    }

    pub async fn current_user(&self) -> Result<CurrentUser> {
        if let Some(user) = self.cache().current_user() {
            Ok(user)
        } else {
            Ok(crate::model!(self.http().current_user()))
        }
    }

    pub async fn role(&self, guild_id: GuildId, role_id: RoleId) -> Result<Role> {
        if let Some(role) = self.cache().role(role_id) {
            Ok(role)
        } else {
            let models: Vec<Role> = crate::list_models!(self.http().roles(guild_id));
            models
                .iter()
                .find(|role| role.id == role_id)
                .cloned()
                .ok_or_else(CacheHelperError::model_not_found)
        }
    }

    pub async fn roles(&self, guild_id: GuildId) -> Result<Vec<Role>> {
        if let Some(role_ids) = self.cache().guild_roles(guild_id) {
            let mut roles = Vec::with_capacity(role_ids.len());
            for role_id in role_ids.iter().copied() {
                match self.cache().role(role_id) {
                    Some(role) => roles.push(role),
                    // Break so that we don't iterate through all the role IDs if we can't get them all
                    None => break,
                }
            }
            if roles.len() == role_ids.len() {
                Ok(roles)
            } else {
                Ok(crate::list_models!(self.http().roles(guild_id)))
            }
        } else {
            Ok(crate::list_models!(self.http().roles(guild_id)))
        }
    }

    pub async fn emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> Result<EmojiHelper> {
        if let Some(emoji) = self.cache().emoji(emoji_id) {
            Ok(emoji.into())
        } else {
            Ok(crate::model!(self.http().emoji(guild_id, emoji_id)).into())
        }
    }

    pub async fn emojis(&self, guild_id: GuildId) -> Result<Vec<EmojiHelper>> {
        if let Some(emoji_ids) = self.cache().guild_emojis(guild_id) {
            let mut emojis = Vec::with_capacity(emoji_ids.len());
            for emoji_id in emoji_ids.iter().copied() {
                match self.cache().emoji(emoji_id) {
                    Some(emoji) => emojis.push(emoji.into()),
                    None => break,
                }
            }

            if emojis.len() == emoji_ids.len() {
                Ok(emojis)
            } else {
                Ok(crate::list_models!(self.http().emojis(guild_id))
                    .into_iter()
                    .map(EmojiHelper::from)
                    .collect())
            }
        } else {
            Ok(crate::list_models!(self.http().emojis(guild_id))
                .into_iter()
                .map(EmojiHelper::from)
                .collect())
        }
    }

    pub async fn user(&self, user_id: UserId) -> Result<User> {
        if let Some(user) = self.cache().user(user_id) {
            Ok(user)
        } else {
            Ok(crate::model!(self.http().user(user_id)))
        }
    }

    pub async fn guild_channel(&self, channel_id: ChannelId) -> Result<GuildChannel> {
        if let Some(channel) = self.cache().guild_channel(channel_id) {
            Ok(channel)
        } else {
            let model: Channel = crate::model!(self.http().channel(channel_id));
            match model {
                Channel::Guild(guild) => Ok(guild),
                _ => Err(CacheHelperError::model_not_found()),
            }
        }
    }

    pub async fn private_channel(&self, channel_id: ChannelId) -> Result<PrivateChannel> {
        if let Some(channel) = self.cache().private_channel(channel_id) {
            Ok(channel)
        } else {
            let model: Channel = crate::model!(self.http().channel(channel_id));
            match model {
                Channel::Private(private) => Ok(private),
                _ => Err(CacheHelperError::model_not_found()),
            }
        }
    }

    pub async fn group_channel(&self, channel_id: ChannelId) -> Result<Group> {
        if let Some(channel) = self.cache().group(channel_id) {
            Ok(channel)
        } else {
            let model: Channel = crate::model!(self.http().channel(channel_id));
            match model {
                Channel::Group(group) => Ok(group),
                _ => Err(CacheHelperError::model_not_found()),
            }
        }
    }

    pub async fn member(&self, guild_id: GuildId, user_id: UserId) -> Result<MemberHelper> {
        if let Some(member) = self.cache().member(guild_id, user_id) {
            Ok(member.into())
        } else {
            Ok(crate::model!(self.http().guild_member(guild_id, user_id)).into())
        }
    }

    pub async fn members(&self, guild_id: GuildId) -> Result<Vec<MemberHelper>> {
        if let Some(member_ids) = self.cache().guild_members(guild_id) {
            let mut members = Vec::with_capacity(member_ids.len());
            for member_id in member_ids.iter().copied() {
                match self.cache().member(guild_id, member_id) {
                    Some(member) => members.push(member.into()),
                    None => break,
                }
            }

            if members.len() == member_ids.len() {
                Ok(members)
            } else {
                Ok(crate::list_models!(self.http().guild_members(guild_id))
                    .into_iter()
                    .map(MemberHelper::from)
                    .collect())
            }
        } else {
            Ok(crate::list_models!(self.http().guild_members(guild_id))
                .into_iter()
                .map(MemberHelper::from)
                .collect())
        }
    }
}
