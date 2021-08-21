#![allow(dead_code)]

use crate::state::State;
use std::result::Result as StdResult;
use tracing::{debug, instrument};
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::Client;
use twilight_model::{
    channel::{Channel, Group, GuildChannel, PrivateChannel},
    guild::Role,
    id::{ChannelId, EmojiId, GuildId, RoleId, UserId},
    user::{CurrentUser, User},
};

pub mod error;
pub mod models;

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

    #[instrument(skip(self))]
    pub async fn current_user(&self) -> Result<CurrentUser> {
        if let Some(user) = self.cache().current_user() {
            debug!("getting user from cache");
            Ok(user)
        } else {
            debug!("getting user from http");
            Ok(crate::model!(self.http().current_user()))
        }
    }

    #[instrument(skip(self))]
    pub async fn role(&self, guild_id: GuildId, role_id: RoleId) -> Result<Role> {
        if let Some(role) = self.cache().role(role_id) {
            debug!("getting role from cache");
            Ok(role)
        } else {
            debug!("getting role from http");
            let models: Vec<Role> = crate::list_models!(self.http().roles(guild_id));
            models
                .iter()
                .find(|role| role.id == role_id)
                .cloned()
                .ok_or_else(CacheHelperError::model_not_found)
        }
    }

    #[instrument(skip(self))]
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
                debug!("getting roles from cache");
                Ok(roles)
            } else {
                debug!("getting roles from http");
                Ok(crate::list_models!(self.http().roles(guild_id)))
            }
        } else {
            debug!("getting roles from http");
            Ok(crate::list_models!(self.http().roles(guild_id)))
        }
    }

    #[instrument(skip(self))]
    pub async fn emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> Result<EmojiHelper> {
        if let Some(emoji) = self.cache().emoji(emoji_id) {
            debug!("getting emoji from cache");
            Ok(emoji.into())
        } else {
            debug!("getting emoji from http");
            Ok(crate::model!(self.http().emoji(guild_id, emoji_id)).into())
        }
    }

    #[instrument(skip(self))]
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
                debug!("getting emojis from cache");
                Ok(emojis)
            } else {
                debug!("getting emojis from http");
                Ok(crate::list_models!(self.http().emojis(guild_id))
                    .into_iter()
                    .map(EmojiHelper::from)
                    .collect())
            }
        } else {
            debug!("getting emojis from http");
            Ok(crate::list_models!(self.http().emojis(guild_id))
                .into_iter()
                .map(EmojiHelper::from)
                .collect())
        }
    }

    #[instrument(skip(self))]
    pub async fn user(&self, user_id: UserId) -> Result<User> {
        if let Some(user) = self.cache().user(user_id) {
            debug!("getting user from cache");
            Ok(user)
        } else {
            debug!("getting user from http");
            Ok(crate::model!(self.http().user(user_id)))
        }
    }

    #[instrument(skip(self))]
    pub async fn guild_channel(&self, channel_id: ChannelId) -> Result<GuildChannel> {
        if let Some(channel) = self.cache().guild_channel(channel_id) {
            debug!("getting guild channel from cache");
            Ok(channel)
        } else {
            debug!("getting guild channel from http");
            let model: Channel = crate::model!(self.http().channel(channel_id));
            match model {
                Channel::Guild(guild) => Ok(guild),
                _ => Err(CacheHelperError::model_not_found()),
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn private_channel(&self, channel_id: ChannelId) -> Result<PrivateChannel> {
        if let Some(channel) = self.cache().private_channel(channel_id) {
            debug!("getting private channel from cache");
            Ok(channel)
        } else {
            debug!("getting private channel from http");
            let model: Channel = crate::model!(self.http().channel(channel_id));
            match model {
                Channel::Private(private) => Ok(private),
                _ => Err(CacheHelperError::model_not_found()),
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn group_channel(&self, channel_id: ChannelId) -> Result<Group> {
        if let Some(channel) = self.cache().group(channel_id) {
            debug!("getting group channel from cache");
            Ok(channel)
        } else {
            debug!("getting group channel from http");
            let model: Channel = crate::model!(self.http().channel(channel_id));
            match model {
                Channel::Group(group) => Ok(group),
                _ => Err(CacheHelperError::model_not_found()),
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn member(&self, guild_id: GuildId, user_id: UserId) -> Result<MemberHelper> {
        if let Some(member) = self.cache().member(guild_id, user_id) {
            debug!("getting member from cache");
            Ok(member.into())
        } else {
            debug!("getting member from http");
            Ok(crate::model!(self.http().guild_member(guild_id, user_id)).into())
        }
    }

    #[instrument(skip(self))]
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
                debug!("getting members from cache");
                Ok(members)
            } else {
                debug!("getting members from http");
                Ok(crate::list_models!(self.http().guild_members(guild_id))
                    .into_iter()
                    .map(MemberHelper::from)
                    .collect())
            }
        } else {
            debug!("getting members from http");
            Ok(crate::list_models!(self.http().guild_members(guild_id))
                .into_iter()
                .map(MemberHelper::from)
                .collect())
        }
    }

    #[instrument(skip(self))]
    pub async fn member_roles(&self, guild_id: GuildId, user_id: UserId) -> Result<Vec<Role>> {
        let guild_roles = self.roles(guild_id).await?;
        let member = self.member(guild_id, user_id).await?;

        let mut roles: Vec<Role> = guild_roles
            .iter()
            .filter(|role| member.roles.contains(&role.id))
            .cloned()
            .collect();

        if roles.is_empty() {
            debug!("returning default \"@everyone\" role");
            return Ok(vec![self.role(guild_id, guild_id.0.into()).await?]);
        }

        roles.sort();

        debug!("returning member roles");
        Ok(roles)
    }
}
