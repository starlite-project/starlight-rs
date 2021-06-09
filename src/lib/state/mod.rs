#![allow(dead_code)]
use futures::StreamExt;
use std::sync::Arc;
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::Cluster;
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

mod builder;
mod events;

pub use self::builder::StateBuilder;

#[derive(Debug, Clone)]
pub struct State {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
}

impl State {
    pub async fn connect(&self) {
        let cluster_spawn = self.cluster.clone();

        tokio::spawn(async move {
            cluster_spawn.up().await;
        });
    }

    pub async fn start(state_ptr: Self) -> super::GenericResult<()> {
        let state = Arc::new(state_ptr);

        let mut events = state.cluster.events();

        while let Some((_, event)) = events.next().await {
            state.cache.update(&event);
            state.standby.process(&event);
            let state_clone = Arc::clone(&state);

            tokio::spawn(async move {
                if let Err(err) = events::handle(event, state_clone).await {
                    crate::error!("{:?}", err);
                }
            });
        }

        Ok(())
    }
}
