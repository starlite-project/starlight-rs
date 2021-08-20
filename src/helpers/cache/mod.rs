#![allow(dead_code)]

use crate::state::State;
use std::result::Result as StdResult;
use twilight_model::{
    guild::Role,
    id::{EmojiId, GuildId, RoleId, UserId},
    user::{CurrentUser, User},
};

mod error;
mod models;

pub use self::{error::CacheHelperError, models::EmojiHelper};

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

    pub async fn current_user(&self) -> Result<CurrentUser> {
        if let Some(user) = self.state.cache.current_user() {
            Ok(user)
        } else {
            Ok(crate::model!(self.state.http.current_user()))
        }
    }

    pub async fn role(&self, guild_id: GuildId, role_id: RoleId) -> Result<Role> {
        if let Some(role) = self.state.cache.role(role_id) {
            Ok(role)
        } else {
            let models: Vec<Role> = crate::list_models!(self.state.http.roles(guild_id));
            models
                .iter()
                .find(|role| role.id == role_id)
                .cloned()
                .ok_or_else(CacheHelperError::model_not_found)
        }
    }

    pub async fn roles(&self, guild_id: GuildId) -> Result<Vec<Role>> {
        if let Some(role_ids) = self.state.cache.guild_roles(guild_id) {
            let mut roles = Vec::with_capacity(role_ids.len());
            for role_id in role_ids.iter().copied() {
                match self.state.cache.role(role_id) {
                    Some(role) => roles.push(role),
                    // Break so that we don't iterate through all the role IDs if we can't get them all
                    None => break,
                }
            }
            if roles.len() == role_ids.len() {
                Ok(roles)
            } else {
                Ok(crate::list_models!(self.state.http.roles(guild_id)))
            }
        } else {
            Ok(crate::list_models!(self.state.http.roles(guild_id)))
        }
    }

    pub async fn emoji(&self, guild_id: GuildId, emoji_id: EmojiId) -> Result<EmojiHelper> {
        if let Some(emoji) = self.state.cache.emoji(emoji_id) {
            Ok(emoji.into())
        } else {
            Ok(crate::model!(self.state.http.emoji(guild_id, emoji_id)).into())
        }
    }

    pub async fn emojis(&self, guild_id: GuildId) -> Result<Vec<EmojiHelper>> {
        if let Some(emoji_ids) = self.state.cache.guild_emojis(guild_id) {
            let mut emojis = Vec::with_capacity(emoji_ids.len());
            for emoji_id in emoji_ids.iter().copied() {
                match self.state.cache.emoji(emoji_id) {
                    Some(emoji) => emojis.push(emoji.into()),
                    None => break,
                }
            }

            if emojis.len() == emoji_ids.len() {
                Ok(emojis)
            } else {
                Ok(crate::list_models!(self.state.http.emojis(guild_id))
                    .into_iter()
                    .map(EmojiHelper::from)
                    .collect())
            }
        } else {
            Ok(crate::list_models!(self.state.http.emojis(guild_id))
                .into_iter()
                .map(EmojiHelper::from)
                .collect())
        }
    }

    pub async fn user(&self, user_id: UserId) -> Result<User> {
        if let Some(user) = self.state.cache.user(user_id) {
            Ok(user)
        } else {
            Ok(crate::model!(self.state.http.user(user_id)))
        }
    }
}
