use std::{
	path::Path,
	sync::atomic::{AtomicUsize, Ordering},
};

use clap::Parser;
use starlight::{
	prelude::*,
	state::{Config, ContextBuilder},
};
use tokio::runtime::Builder;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use twilight_cache_inmemory::{InMemoryCacheBuilder, ResourceType};
use twilight_gateway::{cluster::ShardScheme, Intents};

static ATOMIC_ID: AtomicUsize = AtomicUsize::new(1);

fn main() -> MietteResult<()> {
	Builder::new_multi_thread()
		.enable_all()
		.thread_name_fn(|| {
			let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst) + 1;
			let output = String::from("starlight-pool-");
			output + &id.to_string()
		})
		.on_thread_stop(|| {
			ATOMIC_ID.fetch_sub(1, Ordering::SeqCst);
		})
		.build()
		.into_diagnostic()?
		.block_on(run())
}

async fn run() -> MietteResult<()> {
	let mut log_filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info"))
		.into_diagnostic()?;
	let log_fmt_layer = fmt::layer()
		.pretty()
		.with_thread_ids(true)
		.with_thread_names(true);

	log_filter_layer = if cfg!(debug_assertions) {
		log_filter_layer
			// .add_directive("starlight[act]=debug".parse().into_diagnostic()?)
			.add_directive("starlight=trace".parse().into_diagnostic()?)
	} else {
		log_filter_layer.add_directive("starlight=info".parse().into_diagnostic()?)
	};

	tracing_subscriber::registry()
		.with(log_filter_layer)
		.with(log_fmt_layer)
		.try_init()
		.into_diagnostic()?;

	let config = Config::parse();
	let (client, events) = get_builder(config)?.build().await?;

	Ok(())
}

#[cfg(feature = "docker")]
fn get_builder(config: Config) -> MietteResult<ContextBuilder> {
	let host = starlight::utils::get_host("twilight_proxy", 3000).into_diagnostic()?;
	shared(config)?
		.http_builder(|builder| builder.proxy(host, true).ratelimiter(None))
		.into_diagnostic()
}

#[cfg(not(feature = "docker"))]
fn get_builder(config: Config) -> MietteResult<ContextBuilder> {
	shared(config)
}

fn shared(config: Config) -> MietteResult<ContextBuilder> {
	Ok(ContextBuilder::new()
		.config(config)
		.intents(Intents::empty())
		.shard_builder(|builder| builder)?
		.cache(InMemoryCacheBuilder::new().resource_types(ResourceType::all())))
}
