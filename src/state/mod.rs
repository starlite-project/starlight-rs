#![allow(dead_code)]
use std::ops::Deref;

use super::Config;
use crate::slashies::{commands::get_slashies, interaction::Interaction};
use anyhow::Result;
use futures::StreamExt;
use tracing::{event, Level};
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{cluster::Events, Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_standby::Standby;

mod builder;
pub mod events;

pub use self::builder::StateBuilder;

#[derive(Debug, Clone, Copy)]
pub struct State(&'static Components, pub Config);

impl State {
    pub async fn connect(self) -> Result<()> {
        let cluster_spawn = self.0.cluster.clone();

        let id = self
            .http
            .current_user_application()
            .exec()
            .await?
            .model()
            .await?
            .id;
        self.http.set_application_id(id);

        if self.1.remove_slash_commands {
            event!(Level::INFO, "removing all slash commands");
            if let Some(guild_id) = self.1.guild_id {
                self.http.set_guild_commands(guild_id, &[])?.exec().await
            } else {
                self.http.set_global_commands(&[])?.exec().await
            }?;

            std::process::exit(0);
        };

        event!(Level::INFO, "setting slash commands");
        if let Some(guild_id) = self.1.guild_id {
            self.http
                .set_guild_commands(guild_id, &get_slashies())?
                .exec()
                .await
        } else {
            self.http.set_global_commands(&get_slashies())?.exec().await
        }?;

        tokio::spawn(async move {
            cluster_spawn.up().await;
        });

        Ok(())
    }

    pub const fn interaction<'a>(self, command: &'a ApplicationCommand) -> Interaction<'a> {
        Interaction {
            state: self,
            command,
        }
    }

    pub async fn process(self, mut events: Events) {
        event!(Level::INFO, "started main event stream loop");
        while let Some((_, event)) = events.next().await {
            self.handle_event(&event);
            tokio::spawn(crate::state::events::handle(event, self));
        }
        event!(Level::ERROR, "event stream exhausted (shouldn't happen)");
    }

    pub fn shutdown(self) {
        self.cluster.down();
    }

    pub fn handle_event(&self, event: &Event) {
        self.0.cache.update(event);
        self.0.standby.process(event);
    }
}

impl Deref for State {
    type Target = Components;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Components {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
}
