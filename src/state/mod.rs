#![allow(dead_code)]
use crate::{
	persistence::Database,
	slashies::{commands::get_slashies, interaction::Interaction},
};
use futures::StreamExt;
use miette::{IntoDiagnostic, Result};
use std::ops::Deref;
use tokio::time::Instant;
use tracing::{event, Level};
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{cluster::Events, Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_standby::Standby;

mod builder;
mod config;
mod events;

pub use self::{builder::StateBuilder, config::Config};

#[derive(Debug, Clone, Copy)]
pub struct State(&'static Components);

impl State {
	pub async fn connect(self) -> Result<()> {
		let id = self.0.config.get_user_id()?.into();
		self.http.set_application_id(id);

		if self.0.config.remove_slash_commands {
			event!(Level::INFO, "removing all slash commands");
			if let Some(guild_id) = self.0.config.guild_id {
				self.http
					.set_guild_commands(guild_id, &[])
					.into_diagnostic()?
					.exec()
					.await
			} else {
				self.http
					.set_global_commands(&[])
					.into_diagnostic()?
					.exec()
					.await
			}
			.into_diagnostic()?;

			std::process::exit(0);
		};

		event!(Level::INFO, "setting slash commands");
		if let Some(guild_id) = self.0.config.guild_id {
			self.http
				.set_guild_commands(guild_id, &get_slashies())
				.into_diagnostic()?
				.exec()
				.await
		} else {
			self.http
				.set_global_commands(&get_slashies())
				.into_diagnostic()?
				.exec()
				.await
		}
		.into_diagnostic()?;

		self.cluster.up().await;
		event!(Level::INFO, "all shards connected");

		Ok(())
	}

	#[must_use]
	pub const fn interaction(self, command: &ApplicationCommand) -> Interaction {
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
		self.0.cluster.down();
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
	pub runtime: Instant,
	pub config: Config,
	pub database: Database,
}
