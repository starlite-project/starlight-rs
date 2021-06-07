#![allow(dead_code)]
use futures::StreamExt;
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

mod builder;
mod event_handler;

pub use self::builder::StateBuilder;
pub use self::event_handler::EventHandler;
pub struct State {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
    pub event_handler: Option<Box<dyn EventHandler + 'static>>,
}

impl State {
    pub async fn connect(self) -> super::GenericResult<()> {
        let cluster_spawn = self.cluster.clone();

        tokio::spawn(async move {
            cluster_spawn.up().await;
        });

        let mut events = self.cluster.events();

        while let Some(data) = events.next().await {
            self.cache.update(&data.1);
            self.standby.process(&data.1);

            self.handle(data).await?;
        }

        Ok(())
    }

    async fn handle(&self, data: (u64, Event)) -> super::GenericResult<()> {
        let (shard_id, event) = data;

        match event {
            Event::ShardConnected(_) => crate::log!("Connected with shard {}", shard_id),
            Event::Ready(info) => {
                let user = (*info).user;
                let username = user.name;
                let discriminator = user.discriminator;
                let id = user.id;
                crate::log!(
                    "Ready as user {username}#{discriminator} ({id})",
                    username = username,
                    discriminator = discriminator,
                    id = id
                );
            }
            _ => {}
        }

        Ok(())
    }
}
