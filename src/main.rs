#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::from_iter_instead_of_collect, clippy::module_name_repetitions)]

#[macro_use]
extern crate lazy_static;

use lib::{
    state::{State, StateBuilder},
    GenericResult,
};
use std::env;
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::ShardScheme;
use twilight_model::gateway::Intents;

mod i18n;
mod lib;

#[tokio::main]
async fn main() -> GenericResult<()> {
    dotenv::dotenv()?;

    let client = StateBuilder::new()
        .token(env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILD_MESSAGES)
        .cluster_builder(|builder| builder.shard_scheme(ShardScheme::Auto))
        .cache_builder(|builder| builder.resource_types(ResourceType::MESSAGE))
        .build()
        .await?;

    client.connect().await;

    State::start(client).await?;

    Ok(())
}
