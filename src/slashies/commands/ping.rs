use crate::slashies::{interaction::Interaction, Response, SlashCommand};
use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use std::{convert::TryInto, time::Duration};
use twilight_model::application::{command::CommandType, interaction::ApplicationCommand};
use twilight_util::builder::command::CommandBuilder;

#[derive(Debug, Clone)]
pub struct Ping(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand for Ping {
	const NAME: &'static str = "ping";

	fn define() -> CommandBuilder {
		CommandBuilder::new(
			Self::NAME.to_owned(),
			"Pings the bot".to_owned(),
			CommandType::ChatInput,
		)
	}

	async fn run(&self, interaction: Interaction<'_>) -> Result<()> {
		let state = interaction.state;

		let ping = state
			.cluster()
			.info()
			.values()
			.filter_map(|info| info.latency().average())
			.sum::<Duration>()
			/ state
				.cluster()
				.shards()
				.len()
				.try_into()
				.into_diagnostic()?;

		let mut response = match ping.as_millis() {
			0 => Response::from("Pong! Couldn't quite get average latency"),
			ping => Response::from(format!("Pong! Average latency is {} milliseconds", ping)),
		};

		interaction
			.response(response.ephemeral().take())
			.await
			.into_diagnostic()?;

		Ok(())
	}
}
