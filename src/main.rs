#[macro_use]
extern crate lazy_static;

use lib::{client::ClientBuilder, GenericResult};
use std::env;
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::{ClusterBuilder, ShardScheme};
use twilight_model::gateway::Intents;
use mimalloc::MiMalloc;

mod i18n;
mod lib;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> GenericResult<()> {
    dotenv::dotenv()?;

    let client = ClientBuilder::new()
        .token(env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILD_MESSAGES)
        .cluster_builder(&|builder: ClusterBuilder| builder.shard_scheme(ShardScheme::Auto))
        .cache_builder(&|builder| builder.resource_types(ResourceType::MESSAGE))
        .build()
        .await?;


    client.connect().await?;

    Ok(())
}
