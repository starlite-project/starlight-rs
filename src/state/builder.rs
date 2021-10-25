use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use super::{ClientComponents, Config, State};
use miette::{IntoDiagnostic, Result, WrapErr};
use nebula::Leak;
use starchart::StarChartBuilder;
use supernova::cloned;
use thiserror::Error;
use tokio::time::Instant;
use twilight_cache_inmemory::InMemoryCacheBuilder as CacheBuilder;
use twilight_gateway::{
	cluster::{ClusterBuilder, Events},
	Intents,
};
use twilight_http::client::ClientBuilder as HttpBuilder;

#[derive(Debug, Error, Clone, Copy)]
pub enum StateBuilderError {
	#[error("intents not set")]
	Intents,
	#[error("config not set")]
	Config,
	#[error("cluster not built")]
	Cluster,
	#[error("database url not set")]
	Database,
}

#[derive(Debug, Default)]
pub struct StateBuilder {
	cluster: Option<ClusterBuilder>,
	cache: Option<CacheBuilder>,
	http: Option<HttpBuilder>,
	intents: Option<Intents>,
	config: Option<Config>,
	database: Option<StarChartBuilder>,
	database_path: Option<PathBuf>,
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
			database: None,
			database_path: None,
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

	pub fn database_builder<P, F>(mut self, path: P, database_fn: F) -> Result<Self>
	where
		P: AsRef<Path>,
		F: FnOnce(StarChartBuilder) -> StarChartBuilder,
	{
		self.database_path = Some(path.as_ref().to_path_buf());

		let database = database_fn(StarChartBuilder::new());

		self.database = Some(database);

		Ok(self)
	}

	pub fn cluster_builder<F>(mut self, cluster_fn: F) -> Result<Self>
	where
		F: FnOnce(ClusterBuilder) -> ClusterBuilder,
	{
		let intents = self
			.intents
			.ok_or(StateBuilderError::Intents)
			.into_diagnostic()
			.context("need intents to build cluster")?;
		let token = Config::token()?;

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
		let token = Config::token()?;
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
		let token = Config::token()?.to_owned();
		let http_builder = self
			.http
			.unwrap_or_else(cloned!((token) => move || HttpBuilder::new().token(token)));
		let cluster_builder: ClusterBuilder = self
			.cluster
			.ok_or(StateBuilderError::Cluster)
			.into_diagnostic()
			.context("need cluster to build state")?;
		let cache_builder = self.cache.unwrap_or_default();

		let http = Arc::new(http_builder.token(token).build());
		let cache = Arc::new(cache_builder.build());
		let (cluster, events) = cluster_builder
			.http_client(Arc::clone(&http))
			.build()
			.await
			.into_diagnostic()?;
		let standby = Arc::default();
		let database_builder: StarChartBuilder = self
			.database
			.ok_or(StateBuilderError::Database)
			.into_diagnostic()
			.context("need database to build state")?;
		let database_path: PathBuf = self
			.database_path
			.ok_or(StateBuilderError::Database)
			.into_diagnostic()
			.context("need database url to build state")?;

		let database = database_builder
			.build(database_path)
			.await
			.into_diagnostic()
			.context("failed to build database")?;

		let components = unsafe {
			ClientComponents {
				cache,
				cluster: Arc::new(cluster),
				standby,
				http,
				runtime: Instant::now(),
				config,
				database,
			}
			.leak()
		};

		Ok((State(components), events))
	}
}
