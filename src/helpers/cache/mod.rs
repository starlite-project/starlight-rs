#![allow(dead_code)]

use crate::state::State;
use std::result::Result as StdResult;
use twilight_model::{
    id::{GuildId, RoleId},
    user::CurrentUser,
};

mod error;

pub use self::error::CacheHelperError;

pub type Result<T> = StdResult<T, CacheHelperError>;

#[derive(Debug)]
pub struct CacheHelper<'a> {
    state: &'a State,
}

impl<'a> CacheHelper<'a> {
    pub const fn new(state: &'a State) -> Self {
        Self { state }
    }

    pub async fn current_user(&self) -> Result<CurrentUser> {
        let current_user_cache = self.state.cache.current_user();

        if current_user_cache.is_some() {
            current_user_cache.ok_or(CacheHelperError)
        } else {
            Ok(crate::model!(self.state.http.current_user()))
        }
    }

    pub async fn role(&self, guild_id: GuildId, role_id: RoleId) -> Result<Role> {
        let role_cache = self.state.cache.role(role_id);

        if role_cache.is_some() {
            role_cache.ok_or_else
        }
    }
}
