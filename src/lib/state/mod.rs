#![allow(dead_code)]
use super::util::TypeMap;
use std::sync::{Arc, RwLock};
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

mod builder;
pub mod events;

pub use self::builder::StateBuilder;

#[derive(Debug, Clone)]
pub struct State {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
    pub data: Arc<RwLock<TypeMap>>,
}

impl State {
    pub async fn connect(&self) {
        let cluster_spawn = self.cluster.clone();

        tokio::spawn(async move {
            cluster_spawn.up().await;
        });
    }

    pub fn handle_event(&self, event: &Event) {
        self.cache.update(event);
        self.standby.process(event);
    }
}
