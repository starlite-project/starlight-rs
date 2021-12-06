use std::{ops::Deref, sync::Arc};

use futures_util::StreamExt;
use tracing::{event, Level};
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{shard::Events, Event, Shard};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

use self::events::handle;
pub use self::{builder::ContextBuilder, config::Config};
use crate::{helpers::Helpers, prelude::*};

mod builder;
mod config;
mod events;

#[derive(Debug, Clone, Copy)]
pub struct Context(pub &'static State);

impl Context {
	pub async fn connect(self) -> MietteResult<()> {
		let id = Config::application_id()?;
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
		}

		event!(Level::INFO, "setting slash commands");

		self.0.shard.start().await.into_diagnostic()?;
		event!(Level::INFO, "shard connected");

		Ok(())
	}

	pub async fn process(self, mut events: Events) {
		event!(Level::INFO, "started main event stream loop");
		while let Some(val) = events.next().await {
			self.handle_event(&val);
			tokio::spawn(handle(self, val));
		}
		event!(Level::ERROR, "event stream exhausted (shouldn't happen)");
	}

	pub const fn helpers(self) -> Helpers {
		Helpers::new(self)
	}

	pub fn shutdown(self) {
		self.0.shard.shutdown();
	}

	pub fn handle_event(&self, event: &Event) {
		self.0.cache.update(event);
		self.0.standby.process(event);
	}
}

impl Deref for Context {
	type Target = State;

	fn deref(&self) -> &Self::Target {
		self.0
	}
}

#[derive(Debug, Clone)]
pub struct State {
	cache: Arc<Cache>,
	cdn: reqwest::Client,
	shard: Arc<Shard>,
	http: Arc<HttpClient>,
	standby: Arc<Standby>,
	config: Config,
}

impl State {
	#[must_use]
	pub fn cache(&self) -> &Cache {
		&*self.cache
	}

	#[must_use]
	pub fn shard(&self) -> &Shard {
		&*self.shard
	}

	#[must_use]
	pub fn http(&self) -> &HttpClient {
		&*self.http
	}

	#[must_use]
	pub const fn cdn(&self) -> &reqwest::Client {
		&self.cdn
	}

	#[must_use]
	pub fn standby(&self) -> &Standby {
		&*self.standby
	}

	#[must_use]
	pub const fn config(&self) -> Config {
		self.config
	}
}

pub trait QuickAccess {
	fn context(&self) -> Context;

	fn cache(&self) -> &Cache {
		self.context().0.cache()
	}

	fn shard(&self) -> &Shard {
		self.context().0.shard()
	}

	fn http(&self) -> &HttpClient {
		self.context().0.http()
	}

	fn cdn(&self) -> &reqwest::Client {
		self.context().0.cdn()
	}

	fn standby(&self) -> &Standby {
		self.context().0.standby()
	}

	fn config(&self) -> Config {
		self.context().0.config()
	}
}
