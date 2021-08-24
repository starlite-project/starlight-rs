use anyhow::Result;
use starlight_rs::state::{Config, StateBuilder};
use tracing::{event, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::ShardScheme;
use twilight_model::gateway::Intents;

#[cfg(windows)]
use tokio::signal::windows::ctrl_c;

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<()> {

	dbg!(starlight_rs::get_binary_metadata()?.len());

	let mut log_filter_layer =
		EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
	let log_fmt_layer = fmt::layer()
		.pretty()
		.with_thread_ids(true)
		.with_thread_names(true);

	log_filter_layer = if cfg!(debug_assertions) {
		log_filter_layer
			.add_directive("starlight_rs[act]=debug".parse()?)
			.add_directive("starlight_rs=trace".parse()?)
	} else {
		log_filter_layer.add_directive("starlight_rs=info".parse()?)
	};

	tracing_subscriber::registry()
		.with(log_filter_layer)
		.with(log_fmt_layer)
		.try_init()?;

	dotenv::dotenv()?;

	let config = Config::new()?;

	let (client, events) = StateBuilder::new()
		.config(config)
		.intents(Intents::empty())
		.cluster_builder(|builder| builder.shard_scheme(ShardScheme::Auto))
		.cache_builder(|builder| builder.resource_types(ResourceType::all()))
		.build()
		.await?;

	client.connect().await?;

	#[cfg(windows)]
	{
		let mut signal = ctrl_c()?;
		tokio::select! {
			_ = signal.recv() => event!(Level::INFO, "received SIGINT"),
			_ = client.process(events) => (),
		};
	}

	#[cfg(unix)]
	{
		let mut sigint = signal(SignalKind::interrupt())?;
		let mut sigterm = signal(SignalKind::terminate())?;

		tokio::select! {
			_ = sigint.recv() => event!(Level::INFO, "received SIGINT"),
			_ = sigterm.recv() => event!(Level::INFO, "received SIGTERM"),
			_ = client.process(events) => (),
		};
	}

	event!(Level::INFO, "shutting down");

	client.shutdown();

	Ok(())
}
