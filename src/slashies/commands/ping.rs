use std::pin::Pin;

use futures_util::Future;
use twilight_model::application::{
	command::CommandType, interaction::application_command::CommandData,
};
use twilight_util::builder::command::CommandBuilder;

use crate::{
	helpers::InteractionsHelper,
	prelude::*,
	slashies::{DefineCommand, SlashCommand, SlashData},
};

#[derive(Debug, Clone, Copy)]
pub struct Ping;

impl SlashCommand for Ping {
	fn run(
		&self,
		helper: InteractionsHelper,
		mut data: SlashData,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
		async move {
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
				data.message("Pong! Couldn't quite get average latency".to_owned());
			}

			helper.respond(&mut data).await.into_diagnostic()?;

			Ok(())
		}
		.boxed()
	}
}

impl DefineCommand for Ping {
	fn define() -> CommandBuilder {
		CommandBuilder::new(
			"ping".to_owned(),
			"Pings the bot.".to_owned(),
			CommandType::ChatInput,
		)
		.default_permission(true)
	}

	fn parse(_: CommandData) -> Result<Self> {
		Ok(Self)
	}
}
