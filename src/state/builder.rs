use super::State;
use crate::Config;
use anyhow::{Context, Result};
use std::{sync::Arc, time::Instant};
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

    pub const fn config(mut self, config: Config) -> Self {
        self.config = Some(config);

        self
    }

    pub const fn intents(mut self, intents: Intents) -> Self {
        self.intents = Some(intents);

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
        let http_builder = self
            .http
            .map_or_else(move || HttpBuilder::new().token(token), |builder| builder);
        let http = http_fn(http_builder);

        self.http = Some(http);

        self
    }

    pub async fn build(self) -> Result<(&'static State, Events)> {
        let token = self.config.unwrap_or_default().token.to_owned();
        let http_builder = self.http.unwrap_or_default();
        let cluster_builder = self.cluster.context("Need cluster to build state").unwrap();
        let cache_builder = self.cache.unwrap_or_default();

        let http = http_builder.token(token).build();
        let cache = cache_builder.build();
        let cluster = cluster_builder.http_client(http.clone()).build().await?;
        let standby = Standby::new();

        let state: &'static State = Box::leak(Box::new(State {
            cache: Arc::new(cache),
            cluster: Arc::new(cluster.0),
            http: Arc::new(http),
            standby: Arc::new(standby),
            config: self.config.unwrap_or_default(),
            uptime: Instant::now(),
        }));

        Ok((state, cluster.1))

        // Ok((
        //     Box::new(State {
        //         cache: Arc::new(cache),
        //         cluster: Arc::new(cluster.0),
        //         http: Arc::new(http),
        //         standby: Arc::new(standby),
        //         config: self.config.unwrap_or_default(),
        //         uptime: Instant::now(),
        //     }),
        //     cluster.1,
        // ))
    }
}
