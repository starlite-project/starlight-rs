#![allow(dead_code)]
use super::{Config, GenericResult};
use crate::lib::slashies::commands::commands;
use star_lang::I18nMap;
use tracing::{event, Level};
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

mod builder;
pub mod events;

pub use self::builder::StateBuilder;

#[derive(Debug, Clone)]
pub struct State {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
    pub i18n: I18nMap,
    config: Config,
}

impl State {
    pub async fn connect(&self) -> GenericResult<()> {
        let cluster_spawn = self.cluster.clone();

        let id = self.http.current_user().await?.id;
        self.http.set_application_id(id.0.into());

        if self.config.remove_slash_commands {
            event!(Level::INFO, "removing all slash commands");
            if let Some(guild_id) = self.config.guild_id {
                self.http.set_guild_commands(guild_id, vec![])?.await
            } else {
                self.http.set_global_commands(vec![])?.await
            }?;

            std::process::exit(0);
        };

        event!(Level::INFO, "setting slash commands");
        if let Some(guild_id) = self.config.guild_id {
            self.http.set_guild_commands(guild_id, commands())?.await
        } else {
            self.http.set_global_commands(commands())?.await
        }?;

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
