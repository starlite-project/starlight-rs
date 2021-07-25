#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::from_iter_instead_of_collect, clippy::module_name_repetitions)]

use futures::StreamExt;
use lib::{state::StateBuilder, GenericResult};
use std::{env, sync::Arc};
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::ShardScheme;
use twilight_model::gateway::Intents;

mod lib;

#[tokio::main]
async fn main() -> GenericResult<()> {
    dbg!(clap::crate_authors!());

    let mut fmt_builder = FmtSubscriber::builder().with_span_events(FmtSpan::FULL);

    fmt_builder = if cfg!(debug_assertions) {
        fmt_builder.with_thread_ids(true).with_thread_names(true)
    } else {
        fmt_builder
    };

    fmt_builder.try_init()?;

    dotenv::dotenv()?;

    let (client, mut events) = StateBuilder::new()
        .token(lib::token()?)
        .intents(Intents::all())
        .cluster_builder(|builder| builder.shard_scheme(ShardScheme::Auto))
        .cache_builder(|builder| builder.resource_types(ResourceType::MESSAGE))
        .build()
        .await?;

    client.connect().await?;

    let state = Arc::new(client);

    while let Some((_, event)) = events.next().await {
        state.handle_event(&event);
        let state_clone = Arc::clone(&state);

        tokio::spawn(async move {
            self::lib::state::events::handle(event, state_clone)
                .await
                .unwrap();
        });
    }

    Ok(())
}
