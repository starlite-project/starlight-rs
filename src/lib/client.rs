#![allow(unused_imports, dead_code)]
use std::convert::Into;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
    cluster::{Cluster, ShardScheme},
    Event,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;

macro_rules! match_default {
    ($match_builder:expr, $default:expr) => {
        match $match_builder {
            Some(value) => value,
            None => $default
        };
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    cache: InMemoryCache,
    cluster: Cluster,
    client: HttpClient,
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
}

impl ClientBuilder {
    pub const fn new() -> Self {
        Self {
            shard_scheme: None,
            token: None,
            cache_resource_type: None,
            intents: None
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

    pub fn build(self) {
        let shard_scheme = match_default!(self.shard_scheme, ShardScheme::Auto);
        let intents = match_default!(self.intents, Intents::all());
        let token = self.token.expect("Expected a token to build a client");
        let cache_resource_type = match_default!(self.cache_resource_type, ResourceType::all());
    }
}
