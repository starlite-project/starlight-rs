#[macro_use]
extern crate lazy_static;

use lib::client::Client;
use std::{env, error::Error};
use twilight_cache_inmemory::ResourceType;
use twilight_model::gateway::Intents;
use twilight_gateway::cluster::ShardScheme;

mod i18n;
mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv::dotenv()?;

    let client = Client::builder(env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILD_MESSAGES)
        .resource_type(ResourceType::MESSAGE)
        .shard_scheme(ShardScheme::Auto)
        .build()
        .await?;

    client.connect().await?;

    Ok(())
}
