#![allow(dead_code)]
use std::sync::Arc;

use super::Config;
use crate::lib::slashies::commands::commands;
use anyhow::Result;
use tracing::{event, Level};
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

mod builder;
pub mod events;

pub use self::builder::StateBuilder;

#[derive(Debug, Clone)]
struct StateRef {
    cache: Cache,
    cluster: Cluster,
    http: HttpClient,
    standby: Standby,
    config: Config,
}

#[derive(Debug, Clone)]
pub struct State(Arc<StateRef>);

impl State {
    pub async fn connect(&self) -> Result<()> {
        let cluster_spawn = self.0.cluster.clone();

        let id = self.0.http.current_user_application().await?.id;
        self.0.http.set_application_id(id);

        if self.0.config.remove_slash_commands {
            event!(Level::INFO, "removing all slash commands");
            if let Some(guild_id) = self.0.config.guild_id {
                self.0.http.set_guild_commands(guild_id, vec![])?.await
            } else {
                self.0.http.set_global_commands(vec![])?.await
            }?;

            std::process::exit(0);
        };

        event!(Level::INFO, "setting slash commands");
        if let Some(guild_id) = self.0.config.guild_id {
            self.0.http.set_guild_commands(guild_id, commands())?.await
        } else {
            self.0.http.set_global_commands(commands())?.await
        }?;

        tokio::spawn(async move {
            cluster_spawn.up().await;
        });

        Ok(())
    }

    pub fn cluster(&self) -> &Cluster {
        &self.0.cluster
    }

    pub fn http(&self)-> &HttpClient {
        &self.0.http
    }

    pub fn cache(&self) -> &Cache {
        &self.0.cache
    }

    pub fn handle_event(&self, event: &Event) {
        self.0.cache.update(event);
        self.0.standby.process(event);
    }
}
