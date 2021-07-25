#![allow(dead_code)]
use star_lang::I18nMap;
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

mod builder;
pub mod events;

use crate::lib::slashies::commands::commands;

pub use self::builder::StateBuilder;

use super::GenericResult;

#[derive(Debug, Clone)]
pub struct State {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
    pub i18n: I18nMap,
}

impl State {
    pub async fn connect(&self) -> GenericResult<()> {
        let cluster_spawn = self.cluster.clone();

        let id = self.http.current_user().await?.id;
        self.http.set_application_id(id.0.into());

        tracing::event!(tracing::Level::INFO, "setting slash commands");

        let guild_id = std::env::var("TEST_GUILD_ID").unwrap_or(String::from("0")).parse::<u64>()?;

        if guild_id == 0 {
            self.http.set_global_commands(commands())?.await?;
        } else {
            self.http.set_guild_commands(guild_id.into(), commands())?.await?;
        }

        tokio::spawn(async move {
            cluster_spawn.up().await;
        });

        Ok(())
    }

    pub fn handle_event(&self, event: &Event) {
        self.cache.update(event);
        self.standby.process(event);
    }
}
