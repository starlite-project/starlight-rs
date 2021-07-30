use anyhow::Result;
use futures::StreamExt;
use starlight_rs::{state::StateBuilder, Config};
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::ShardScheme;
use twilight_model::gateway::Intents;

#[tokio::main]
async fn main() -> Result<()> {
    let mut fmt_builder = FmtSubscriber::builder().with_span_events(FmtSpan::FULL);

    fmt_builder = if cfg!(debug_assertions) {
        fmt_builder.with_thread_ids(true).with_thread_names(true)
    } else {
        fmt_builder
    };

    fmt_builder
        .try_init()
        .expect("failed to init fmt subscriber");

    dotenv::dotenv()?;

    let config = Config::new()?;

    let (client, mut events) = StateBuilder::new()
        .config(config)
        .intents(Intents::empty())
        .cluster_builder(|builder| builder.shard_scheme(ShardScheme::Auto))
        .cache_builder(|builder| builder.resource_types(ResourceType::MESSAGE))
        .build()
        .await?;

    client.connect().await?;

    while let Some((_, event)) = events.next().await {
        client.handle_event(&event);

        tokio::spawn(async move {
            starlight_rs::state::events::handle(event, client)
                .await
                .unwrap();
        });
    }

    Ok(())
}
