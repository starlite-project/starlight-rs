#![allow(dead_code)]
use std::{convert::Into, error::Error};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Event,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;
use futures::StreamExt;

#[derive(Debug, Clone)]
pub struct Client {
    cache: InMemoryCache,
    cluster: Cluster,
    http: HttpClient,
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    shard_scheme: Option<ShardScheme>,
    token: Option<String>,
    cache_resource_type: Option<ResourceType>,
    intents: Option<Intents>,
}

impl Client {
    pub fn builder(token: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new().token(token)
    }

    pub async fn connect(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let cluster_spawn = self.cluster.clone();

        tokio::spawn(async move { cluster_spawn.up().await });

        let mut events = self.cluster.events();

        while let Some(data) = events.next().await {
            self.cache.update(&data.1);

            tokio::spawn(Client::handle_event(data, self.http.clone()));
        }

        Ok(())
    }

    async fn handle_event(
        data: (u64, Event),
        _http: HttpClient
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (shard_id, event) = data;
        match event {
            Event::ShardConnected(_) => crate::log!("Connected with shard {}", shard_id),
            _ => {}
        }

        Ok(())
    }
}

impl ClientBuilder {
    pub const fn new() -> Self {
        Self {
            shard_scheme: None,
            token: None,
            cache_resource_type: None,
            intents: None,
        }
    }

    pub fn token(mut self, str: impl Into<String>) -> Self {
        let token = str.into();

        let token = if token.starts_with("Bot ") {
            token
        } else {
            format!("Bot {}", token)
        };

        self.token = Some(token);

        self
    }

    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = Some(intents);

        self
    }

    pub fn shard_scheme(mut self, scheme: ShardScheme) -> Self {
        self.shard_scheme = Some(scheme);

        self
    }

    pub fn resource_type(mut self, resource_type: ResourceType) -> Self {
        self.cache_resource_type = Some(resource_type);

        self
    }

    pub async fn build(self) -> Result<Client, Box<dyn Error + Send + Sync>> {
        let shard_scheme = self.shard_scheme.unwrap_or(ShardScheme::Auto);
        let intents = self.intents.unwrap_or_else(Intents::all);
        let token = self.token.expect("Expected a token");
        let resource_type = self.cache_resource_type.unwrap_or_else(ResourceType::all);

        let cache = InMemoryCache::builder()
            .resource_types(resource_type)
            .build();

        let cluster = Cluster::builder(token.clone(), intents)
            .shard_scheme(shard_scheme)
            .build()
            .await?;

        let http = HttpClient::new(&token);

        Ok(Client {
            cache,
            cluster,
            http,
        })
    }
}
