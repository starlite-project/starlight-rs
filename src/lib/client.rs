#![allow(dead_code)]
use futures::StreamExt;
use twilight_cache_inmemory::{InMemoryCache as Cache, InMemoryCacheBuilder as CacheBuilder};
use twilight_gateway::{
    cluster::{Cluster, ClusterBuilder},
    Event,
};
use twilight_http::{client::ClientBuilder as HttpClientBuilder, Client as HttpClient};
use twilight_model::gateway::Intents;
use twilight_standby::Standby;

#[derive(Debug, Clone)]
pub struct Client {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    cluster: Option<ClusterBuilder>,
    cache: Option<CacheBuilder>,
    http: Option<HttpClientBuilder>,
    token: Option<String>,
    intents: Option<Intents>,
}

impl Client {
    pub async fn connect(&self) -> super::GenericResult<()> {
        let cluster_spawn = self.cluster.clone();

        cluster_spawn.up().await;

        let mut events = self.cluster.events();

        while let Some(data) = events.next().await {
            self.cache.update(&data.1);
            self.standby.process(&data.1);

            tokio::spawn(Self::handle_event(data));
        }

        Ok(())
    }

    async fn handle_event(data: (u64, Event)) -> super::GenericResult<()> {
        let (shard_id, event) = data;

        match event {
            Event::ShardConnected(_) => crate::log!("Connected with shard {}", shard_id),
            Event::Ready(info) => {
                let info = *info;
                let username = info.user.name;
                let discriminator = info.user.discriminator;
                let id = info.user.id;
                crate::log!("Ready as user {}#{} ({})", username, discriminator, id);
            }
            _ => {}
        }

        Ok(())
    }
}

impl ClientBuilder {
    pub const fn new() -> Self {
        Self {
            cluster: None,
            cache: None,
            http: None,
            token: None,
            intents: None,
        }
    }

    pub fn token(mut self, token: impl std::fmt::Display) -> Self {
        let token_string = token.to_string();

        let token_string = if token_string.starts_with("Bot ") {
            token_string
        } else {
            format!("Bot {}", token_string)
        };

        self.token = Some(token_string);

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
        let intents = self.intents.expect("Need intents to build cluster");
        let token = self.token.clone().expect("Need token to build cluster");

        let cluster = cluster_fn((token, intents).into());

        self.cluster = Some(cluster);

        self
    }

    pub fn cache_builder<F>(mut self, cache_fn: F) -> Self
    where
        F: FnOnce(CacheBuilder) -> CacheBuilder,
    {
        let built = cache_fn(CacheBuilder::new());

        self.cache = Some(built);

        self
    }

    pub fn http_builder<F>(mut self, http_fn: F) -> Self
    where
        F: FnOnce(HttpClientBuilder) -> HttpClientBuilder,
    {
        let token = self.token.clone().expect("Need token to build http");
        let http = http_fn(HttpClientBuilder::new()).token(token);

        self.http = Some(http);

        self
    }

    pub async fn build(self) -> super::GenericResult<Client> {
        let http_builder = self.http.unwrap_or_default();
        let cluster_builder = self.cluster.expect("Failed to get cluster_builder");
        let cache_builder = self.cache.unwrap_or_default();

        let http = http_builder.build();
        let cache = cache_builder.build();
        let cluster = cluster_builder.build().await?;
        let standby = Standby::new();

        Ok(Client {
            cache,
            cluster,
            http,
            standby,
        })
    }
}
