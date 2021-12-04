use std::pin::Pin;

use futures_util::Future;
use twilight_interactions::command::CreateCommand;

use crate::{
	helpers::InteractionsHelper,
	prelude::*,
	slashies::{SlashCommand, SlashData},
};

#[derive(Debug, Clone, Copy, CreateCommand)]
#[command(name = "ping", desc = "Pings the bot")]
pub struct Ping {}

impl SlashCommand for Ping {
	fn run(
		&self,
		helper: InteractionsHelper,
		mut data: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>> + Send>> {
		Box::pin(async move {
			data.ephemeral();
			let context = helper.context();

			if let Some(pong) = context
				.shard()
				.info()
				.into_diagnostic()?
				.latency()
				.average()
			{
				data.message(format!(
					"Pong! Average latency is {} milliseconds",
					pong.as_millis()
				));
			} else {
				data.message("Pong! Couldn't quite get average latency");
			}

			helper.respond(&data).await.into_diagnostic()?;

			Ok(())
		})
	}
}
