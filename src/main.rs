use std::sync::atomic::{AtomicUsize, Ordering};

use clap::Parser;
use dotenv::dotenv;
use starlight::{
	prelude::*,
	state::{Config, ContextBuilder, State},
};
use tokio::runtime::Builder;
#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(windows)]
use tokio::signal::windows::{ctrl_break, ctrl_c};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use twilight_cache_inmemory::{InMemoryCacheBuilder, ResourceType};
use twilight_gateway::Intents;

static THREAD_ID: AtomicUsize = AtomicUsize::new(1);

fn main() -> Result<()> {
	dotenv().ok();
	Builder::new_multi_thread()
		.enable_all()
		.thread_name_fn(|| {
			let id = THREAD_ID.fetch_add(1, Ordering::SeqCst) + 1;
			let output = String::from("starlight-pool-");
			output + &id.to_string()
		})
		.on_thread_stop(|| {
			THREAD_ID.fetch_sub(1, Ordering::SeqCst);
		})
		.build()
		.into_diagnostic()?
		.block_on(run())
}

async fn run() -> Result<()> {
	let mut log_filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info"))
		.into_diagnostic()?;
	let log_fmt_layer = fmt::layer()
		.pretty()
		.with_thread_ids(true)
		.with_thread_names(true);

	log_filter_layer = if cfg!(debug_assertions) {
		log_filter_layer.add_directive("starlight=debug".parse().into_diagnostic()?)
	} else {
		log_filter_layer.add_directive("starlight=info".parse().into_diagnostic()?)
	};

	tracing_subscriber::registry()
		.with(log_filter_layer)
		.with(log_fmt_layer)
		.try_init()
		.into_diagnostic()?;

	let config = Config::parse();
	let (client, events) = ContextBuilder::new()
		.config(config)
		.intents(Intents::from_bits(3).unwrap_or_else(Intents::all))
		.shard_builder(|b| b)?
		.cache(InMemoryCacheBuilder::new().resource_types(ResourceType::all()))
		.database_path("./target/db")
		.build()
		.await?;

	client.connect().await?;

	#[cfg(windows)]
	{
		let mut sig_c = ctrl_c().into_diagnostic()?;
		let mut sig_break = ctrl_break().into_diagnostic()?;
		tokio::select! {
			_ = sig_c.recv() => event!(Level::INFO, "received CTRLC"),
			_ = sig_break.recv() => event!(Level::INFO, "received CTRLBREAK"),
			_ = client.process(events) => (),
		};
	}

	#[cfg(unix)]
	{
		let mut sigint = signal(SignalKind::interrupt()).into_diagnostic()?;
		let mut sigterm = signal(SignalKind::terminate()).into_diagnostic()?;

		tokio::select! {
			_ = sigint.recv() => event!(Level::INFO, "received SIGINT"),
			_ = sigterm.recv() => event!(Level::INFO, "received SIGTERM"),
			_ = client.process(events) => (),
		};
	}

	event!(Level::INFO, "shutting down");

	client.shutdown();

	let client_ptr = unsafe { Box::from_raw(client.0 as *const State as *mut State) };

	drop(client_ptr);

	Ok(())
}
