use super::{Components, Config, State};
use crate::persistence::Database;
use anyhow::{Context, Result};
use supernova::cloned;
use tokio::time::Instant;
use twilight_cache_inmemory::InMemoryCacheBuilder as CacheBuilder;
use twilight_gateway::{
	cluster::{ClusterBuilder, Events},
	Intents,
};
use twilight_http::client::ClientBuilder as HttpBuilder;
use twilight_standby::Standby;

#[derive(Debug, Default)]
pub struct StateBuilder {
	cluster: Option<ClusterBuilder>,
	cache: Option<CacheBuilder>,
	http: Option<HttpBuilder>,
	intents: Option<Intents>,
	config: Option<Config>,
}

impl StateBuilder {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			cluster: None,
			cache: None,
			http: None,
			intents: None,
			config: None,
		}
	}

	pub const fn config(mut self, config: Config) -> Result<Self> {
		self.config = Some(config);

		Ok(self)
	}

	pub const fn intents(mut self, intents: Intents) -> Result<Self> {
		self.intents = Some(intents);

		Ok(self)
	}

	pub fn cluster_builder<F>(mut self, cluster_fn: F) -> Result<Self>
	where
		F: FnOnce(ClusterBuilder) -> ClusterBuilder,
	{
		let intents = self
			.intents
			.context("need intents to build cluster")?;
		let token = self
			.config
			.context("need config to build cluster")?
			.token;

		let cluster = cluster_fn((token, intents).into());

		self.cluster = Some(cluster);

		Ok(self)
	}

	pub fn cache_builder<F>(mut self, cache_fn: F) -> Result<Self>
	where
		F: FnOnce(CacheBuilder) -> CacheBuilder,
	{
		let built = cache_fn(CacheBuilder::default());

		self.cache = Some(built);

		Ok(self)
	}

	pub fn http_builder<F>(mut self, http_fn: F) -> Result<Self>
	where
		F: FnOnce(HttpBuilder) -> HttpBuilder,
	{
		let token = self
			.config
			.context("need config to build http")?
			.token;
		let http_builder = self.http.map_or_else(
			move || HttpBuilder::new().token(token.to_owned()),
			|builder| builder,
		);
		let http = http_fn(http_builder);

		self.http = Some(http);

		Ok(self)
	}

	pub async fn build(self) -> Result<(State, Events)> {
		let config = self.config.unwrap_or_default();
		let token = config.token.to_owned();
		let http_builder = self
			.http
			.unwrap_or_else(cloned!(token => move || HttpBuilder::new().token(token)));
		let cluster_builder = self.cluster.context("Need cluster to build state")?;
		let cache_builder = self.cache.unwrap_or_default();

		let http = http_builder.token(token).build();
		let cache = cache_builder.build();
		let cluster = cluster_builder.http_client(http.clone()).build().await?;
		let standby = Standby::new();

		let components: &'static Components = Box::leak(Box::new(Components {
			cache,
			cluster: cluster.0,
			standby,
			http,
			runtime: Instant::now(),
			config,
			database: Database::open()?,
		}));

		Ok((State(components), cluster.1))
	}
}
