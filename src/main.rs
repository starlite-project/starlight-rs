#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::from_iter_instead_of_collect, clippy::module_name_repetitions)]

#[macro_use]
extern crate lazy_static;

use async_trait::async_trait;
use lib::{
    state::{EventHandler, State, StateBuilder},
    GenericResult,
};
use std::env;
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::ShardScheme;
use twilight_model::gateway::{payload::Ready, Intents};

mod i18n;
mod lib;

#[derive(Debug, Clone, Copy)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: &State, ready: Ready) -> GenericResult<()> {
        let user = ready.user;
        let username = user.name;
        let discrim = user.discriminator;
        let id = user.id;
        crate::log!(
            "Ready as user {username}#{discriminator} ({id})",
            username = username,
            discriminator = discrim,
            id = id
        );

        Ok(())
    }
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    dotenv::dotenv()?;

    let client = StateBuilder::new()
        .token(env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILD_MESSAGES)
        .cluster_builder(|builder| builder.shard_scheme(ShardScheme::Auto))
        .cache_builder(|builder| builder.resource_types(ResourceType::MESSAGE))
        .event_handler(Handler)
        .build()
        .await?;

    client.connect().await?;

    Ok(())
}
