use crate::{slashies::{Response, SlashCommand}, state::State};
use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use std::{convert::TryInto, time::Duration};
use twilight_model::application::{
	command::{Command, CommandType},
	interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Ping(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand for Ping {
	const NAME: &'static str = "ping";

	fn define() -> Command {
		Command {
			application_id: None,
			default_permission: None,
			description: String::from("Pings the bot"),
			guild_id: None,
			id: None,
			name: String::from(Self::NAME),
			options: vec![],
			kind: CommandType::ChatInput,
		}
	}

	async fn run(&self, state: State) -> Result<()> {
		let interaction = state.interaction(&self.0);

		let ping = state
			.cluster
			.info()
			.values()
			.filter_map(|info| info.latency().average())
			.sum::<Duration>()
			/ state.cluster.shards().len().try_into().into_diagnostic()?;

		let response = match ping.as_millis() {
			0 => Response::from("Pong! Couldn't quite get average latency"),
			ping => {
				Response::new().message(format!("Pong! Average latency is {} milliseconds", ping))
			}
		};

		interaction
			.response(response.ephemeral())
			.await
			.into_diagnostic()?;

		Ok(())
	}
}
