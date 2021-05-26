#![allow(dead_code)]
use futures::StreamExt;
use twilight_cache_inmemory::{InMemoryCache as Cache, InMemoryCacheBuilder as CacheBuilder};
use twilight_gateway::{
    cluster::{Cluster, ClusterBuilder},
    Event,
};
use twilight_http::{client::ClientBuilder as HttpClientBuilder, Client as HttpClient};
use twilight_model::gateway::Intents;

#[derive(Debug, Clone)]
pub struct Client {
    cache: Cache,
    cluster: Cluster,
    http: HttpClient,
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
        println!("Got past cluster clone");

        tokio::spawn(async move { cluster_spawn.up().await }).await?;

        println!("Got past cluster spawn");

        let mut events = self.cluster.events();

        println!("Got past cluster events");

        while let Some(data) = events.next().await {
            self.cache.update(&data.1);

            println!("Got past events.next()");

            tokio::spawn(Client::handle_event(data));
        }

        Ok(())
    }

    async fn handle_event(data: (u64, Event)) -> super::GenericResult<()> {
        let (shard_id, event) = data;

        println!("Got past handle_event");

        if let Event::ShardConnected(_) = event {
            crate::log!("Connected with shard {}", shard_id);
        }

        Ok(())
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            cluster: None,
            cache: None,
            http: None,
            token: None,
            intents: None,
        }
    }

    pub fn token(mut self, token: impl ToString) -> Self {
        let token_string = token.to_string();

        let token_string = if token_string.starts_with("Bot ") {
            token_string
        } else {
            format!("Bot {}", token_string)
        };

        self.token = Some(token_string);

        self
    }

    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = Some(intents);

        self
    }

    pub fn cluster_builder(
        mut self,
        cluster_fn: &(dyn Fn(ClusterBuilder) -> ClusterBuilder),
    ) -> Self {
        let intents = self.intents.expect("Need intents to build cluster");
        let token = self.token.clone().expect("Need token to build cluster");

        let cluster_builder = (token, intents).into();

        let built = cluster_fn(cluster_builder);

        self.cluster = Some(built);

        self
    }

    pub fn cache_builder(mut self, cache_fn: &(dyn Fn(CacheBuilder) -> CacheBuilder)) -> Self {
        let cache_builder = CacheBuilder::new();

        let built = cache_fn(cache_builder);

        self.cache = Some(built);

        self
    }

    pub fn http_builder(
        mut self,
        http_fn: &(dyn Fn(HttpClientBuilder) -> HttpClientBuilder),
    ) -> Self {
        let http_builder = HttpClientBuilder::new();

        let built = http_fn(http_builder);

        self.http = Some(built);

        self
    }

    pub async fn build(self) -> super::GenericResult<Client> {
        let http_builder = self.http.unwrap_or_default();
        let cluster_builder = self.cluster.expect("Failed to get cluster_builder");
        let cache_builder = self.cache.unwrap_or_default();

        let http = http_builder.build();
        let cache = cache_builder.build();
        let cluster = cluster_builder.http_client(http.clone()).build().await?;

        Ok(Client {
            http,
            cache,
            cluster,
        })
    }
}
