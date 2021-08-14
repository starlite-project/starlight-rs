use super::SlashCommand;
use crate::{slashies::Response, state::State};
use anyhow::Result;
use async_trait::async_trait;
use std::{convert::TryInto, time::Duration};
use twilight_model::application::{command::Command, interaction::ApplicationCommand};

#[derive(Debug, Clone)]
pub struct Ping(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand<0> for Ping {
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
            / state.cluster.shards().len().try_into()?;

        let mut response = match ping.as_millis() {
            0 => Response::from("Pong! Couldn't quite get average latency"),
            ping => {
                Response::new().message(format!("Pong! Average latency is {} milliseconds", ping))
            }
        };

        interaction.response(response.ephemeral()).await?;

        Ok(())
    }
}
