#[macro_use]
extern crate lazy_static;

use lib::{client::ClientBuilder, GenericResult};
use std::env;
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::{ClusterBuilder, ShardScheme};
use twilight_model::gateway::Intents;

mod i18n;
mod lib;

#[tokio::main]
async fn main() -> GenericResult<()> {
    dotenv::dotenv()?;

    crate::log!("Got past dotenv");

    let client = ClientBuilder::new()
        .token(env::var("DISCORD_TOKEN")?)
        .intents(Intents::all())
        .cluster_builder(&|builder: ClusterBuilder| builder.shard_scheme(ShardScheme::Auto))
        .cache_builder(&|builder| builder.resource_types(ResourceType::MESSAGE))
        .build()
        .await?;

        crate::log!("Got past builder");

    client.connect().await?;

    Ok(())
}
