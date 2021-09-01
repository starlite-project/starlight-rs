use super::Config;
use super::{Components, State};
use anyhow::{Context, Result};
use heed::EnvOpenOptions;
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};
use tokio::{fs::create_dir_all, time::Instant};
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
	database: Option<EnvOpenOptions>,
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

	pub fn database_builder<F>(mut self, database_fn: F) -> Self
	where
		F: FnOnce(EnvOpenOptions) -> EnvOpenOptions,
	{
		self.database = Some(database_fn(EnvOpenOptions::new()));

		self
	}

	pub fn cluster_builder<F>(mut self, cluster_fn: F) -> Self
	where
		F: FnOnce(ClusterBuilder) -> ClusterBuilder,
	{
		let intents = self
			.intents
			.context("need intents to build cluster")
			.unwrap();
		let token = self
			.config
			.context("need config to build cluster")
			.unwrap()
			.token;

		let cluster = cluster_fn((token, intents).into());

		self.cluster = Some(cluster);

		self
	}

	pub fn cache_builder<F>(mut self, cache_fn: F) -> Self
	where
		F: FnOnce(CacheBuilder) -> CacheBuilder,
	{
		let built = cache_fn(CacheBuilder::default());

		self.cache = Some(built);

		self
	}

	pub fn http_builder<F>(mut self, http_fn: F) -> Self
	where
		F: FnOnce(HttpBuilder) -> HttpBuilder,
	{
		let token = self
			.config
			.context("need config to build http")
			.unwrap()
			.token;
		let http_builder = self.http.map_or_else(
			move || HttpBuilder::new().token(token.to_owned()),
			|builder| builder,
		);
		let http = http_fn(http_builder);

		self.http = Some(http);

		self
	}

	pub async fn build(self) -> Result<(State, Events)> {
		let env_path = {
			let system = System::new_all();

			let mut exe_path = system
				.process(get_current_pid().expect("failed to get pid"))
				.expect("failed to get process")
				.exe()
				.to_path_buf();

			exe_path.pop();

			exe_path.push("star-db.mdb");

			create_dir_all(&exe_path).await?;

			exe_path
		};

		let token = self.config.unwrap_or_default().token.to_owned();
		let http_builder = self.http.unwrap_or_default();
		let cluster_builder = self.cluster.context("Need cluster to build state").unwrap();
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
			config: self.config.unwrap_or_default(),
			database: self
				.database
				.unwrap_or_else(EnvOpenOptions::new)
				.open(env_path)
				.unwrap(),
		}));

		Ok((State(components), cluster.1))
	}
}
