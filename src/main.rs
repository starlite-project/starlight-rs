use anyhow::Result;
use starlight::state::{Config, StateBuilder};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::Builder;
use tracing::{event, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use twilight_cache_inmemory::ResourceType;
use twilight_gateway::cluster::ShardScheme;
use twilight_model::gateway::Intents;

// One because the main thread is technically first, so this has to be +1
static ATOMIC_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(windows)]
use tokio::signal::windows::{ctrl_break, ctrl_c};

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

fn main() -> Result<()> {
	Builder::new_multi_thread()
		.enable_all()
		.thread_name_fn(|| {
			let id = {
				ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
				ATOMIC_ID.load(Ordering::SeqCst)
			};
			format!("starlight-pool-{}", id)
		})
		.on_thread_stop(|| {
			ATOMIC_ID.fetch_sub(1, Ordering::SeqCst);
		})
		.build()?
		.block_on(run())?;

	Ok(())
}

async fn run() -> Result<()> {
	let mut log_filter_layer =
		EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
	let log_fmt_layer = fmt::layer()
		.pretty()
		.with_thread_ids(true)
		.with_thread_names(true);

	log_filter_layer = if cfg!(debug_assertions) {
		log_filter_layer
			.add_directive("starlight[act]=debug".parse()?)
			.add_directive("starlight=trace".parse()?)
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
		let mut sig_c = ctrl_c()?;
		let mut sig_break = ctrl_break()?;
		tokio::select! {
			_ = sig_c.recv() => event!(Level::INFO, "received CTRLC"),
			_ = sig_break.recv() => event!(Level::INFO, "received CTRLBREAK"),
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
