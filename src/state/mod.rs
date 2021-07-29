#![allow(dead_code)]
use super::Config;
use crate::slashies::commands::commands;
use anyhow::Result;
use std::{sync::Arc, time::Instant};
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
    cache: Arc<Cache>,
    cluster: Arc<Cluster>,
    http: Arc<HttpClient>,
    standby: Arc<Standby>,
    config: Config,
    uptime: Instant,
}

impl State {
    pub async fn connect(&self) -> Result<()> {
        let cluster_spawn = self.cluster.clone();

        let id = self.http.current_user_application().await?.id;
        self.http.set_application_id(id);

        if self.config.remove_slash_commands {
            event!(Level::INFO, ?self.config.guild_id, "removing all slash commands");
            if let Some(guild_id) = self.config.guild_id {
                self.http.set_guild_commands(guild_id, vec![])?.await
            } else {
                self.http.set_global_commands(vec![])?.await
            }?;

            std::process::exit(0);
        };

        event!(Level::INFO, ?self.config.guild_id, "setting slash commands");
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

    pub fn cluster(&self) -> Arc<Cluster> {
        self.cluster.clone()
    }

    pub fn http(&self) -> Arc<HttpClient> {
        self.http.clone()
    }

    pub fn cache(&self) -> Arc<Cache> {
        self.cache.clone()
    }

    pub fn standby(&self) -> Arc<Standby> {
        self.standby.clone()
    }

    pub const fn uptime(&self) -> Instant {
        self.uptime
    }

    pub fn handle_event(&self, event: &Event) {
        self.cache.update(event);
        self.standby.process(event);
    }
}
