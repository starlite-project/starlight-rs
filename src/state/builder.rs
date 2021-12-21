use std::{
	env::VarError,
	path::{Path, PathBuf},
	sync::Arc,
};

use starchart::{backend::RonBackend, Starchart};
use starlight_macros::cloned;
use thiserror::Error;
use twilight_cache_inmemory::InMemoryCacheBuilder;
use twilight_gateway::{
	shard::{Events, ShardBuilder},
	Intents,
};
use twilight_http::client::ClientBuilder;

use super::{Config, Context, State};
use crate::prelude::*;

#[derive(Debug, Error)]
pub enum ContextBuildError {
	#[error("intents not set")]
	Intents,
	#[error("shard builder not set")]
	Shard,
	#[error("database path not set")]
	Database,
}

#[derive(Debug, Default)]
#[must_use = "a context builder has no side effects"]
pub struct ContextBuilder {
	shard: Option<ShardBuilder>,
	cache: Option<InMemoryCacheBuilder>,
	http: Option<ClientBuilder>,
	intents: Option<Intents>,
	cdn: Option<reqwest::ClientBuilder>,
	config: Option<Config>,
	database_path: Option<PathBuf>,
}

impl ContextBuilder {
	pub const fn new() -> Self {
		Self {
			shard: None,
			cache: None,
			http: None,
			intents: None,
			config: None,
			cdn: None,
			database_path: None,
		}
	}

	pub const fn config(mut self, config: Config) -> Self {
		self.config = Some(config);

		self
	}

	pub const fn intents(mut self, intents: Intents) -> Self {
		self.intents = Some(intents);

		self
	}

	pub fn shard_builder<F>(mut self, shard_builder: F) -> MietteResult<Self>
	where
		F: FnOnce(ShardBuilder) -> ShardBuilder,
	{
		let intents = self
			.intents
			.ok_or(ContextBuildError::Intents)
			.into_diagnostic()
			.context("need intents to build shard")?;
		let token = Config::token().into_diagnostic()?;

		let shard = shard_builder((token, intents).into());

		self.shard = Some(shard);

		Ok(self)
	}

	pub const fn cache(mut self, cache_builder: InMemoryCacheBuilder) -> Self {
		self.cache = Some(cache_builder);

		self
	}

	pub fn database_path<T: AsRef<Path>>(mut self, p: T) -> Self {
		let path = p.as_ref().to_path_buf();

		self.database_path = Some(path);

		self
	}

	pub fn cdn_builder<F>(mut self, cdn_builder: F) -> Result<Self, reqwest::Error>
	where
		F: FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder,
	{
		self.cdn = Some(cdn_builder(reqwest::ClientBuilder::new()));

		Ok(self)
	}

	pub fn http_builder<F>(mut self, http_builder_fn: F) -> Result<Self, VarError>
	where
		F: FnOnce(ClientBuilder) -> ClientBuilder,
	{
		let token = Config::token()?;
		let http_builder = self.http.map_or_else(
			move || ClientBuilder::new().token(token.to_owned()),
			|builder| builder,
		);

		let http = http_builder_fn(http_builder);

		self.http = Some(http);

		Ok(self)
	}

	pub async fn build(self) -> MietteResult<(Context, Events)> {
		let config = self.config.unwrap_or_default();
		let token = Config::token().into_diagnostic()?.to_owned();
		let http_builder = self
			.http
			.unwrap_or_else(cloned!(token => move || ClientBuilder::new().token(token)));
		let shard_builder: ShardBuilder = self
			.shard
			.ok_or(ContextBuildError::Shard)
			.into_diagnostic()
			.context("need cluster to build state")?;
		let cdn_builder = self.cdn.unwrap_or_default();
		let db_path = self
			.database_path
			.ok_or(ContextBuildError::Database)
			.into_diagnostic()
			.context("need database path to build state")?;

		let cache_builder = self.cache.unwrap_or_default();

		let http = Arc::new(http_builder.token(token).build());
		let cache = Arc::new(cache_builder.build());
		let (shard, events) = shard_builder.http_client(Arc::clone(&http)).build();
		let cdn = cdn_builder.build().into_diagnostic()?;
		let standby = Arc::default();
		let backend = RonBackend::new(db_path).into_diagnostic()?;

		let database = Starchart::new(backend).await.into_diagnostic()?;

		let components = Box::leak(Box::new(State {
			cache,
			shard: Arc::new(shard),
			standby,
			http,
			cdn,
			config,
			database
		}));

		Ok((Context(components), events))
	}
}
