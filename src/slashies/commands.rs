use super::{PartialApplicationCommand, Response};
use crate::state::State;
use anyhow::Result;
use async_trait::async_trait;
use std::{convert::TryInto, time::Duration};
use twilight_model::application::{
    callback::InteractionResponse, command::Command as SlashCommand,
};

#[must_use]
pub fn get_slashies() -> Vec<SlashCommand> {
    vec![Ping::define()]
}

#[async_trait]
pub trait Command {
    const NAME: &'static str;

    async fn run(&self, ctx: &State) -> Result<InteractionResponse>;

    fn define() -> SlashCommand;
}

#[derive(Debug, Clone)]
pub enum Commands {
    Ping(Ping),
}

impl Commands {
    #[must_use]
    pub fn r#match(command: PartialApplicationCommand) -> Option<Self> {
        match command.data.name.as_str() {
            Ping::NAME => Some(Self::Ping(Ping(command))),
            _ => None,
        }
    }

    pub async fn run(&self, state: &State) -> Result<InteractionResponse> {
        match self {
            Self::Ping(c) => c.run(state).await,
        }
    }

    #[must_use]
    pub const fn is_long(&self) -> bool {
        match self {
            Self::Ping(_) => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ping(PartialApplicationCommand);

#[async_trait]
impl Command for Ping {
    const NAME: &'static str = "ping";

    fn define() -> SlashCommand {
        SlashCommand {
            application_id: None,
            default_permission: None,
            description: String::from("Pings the bot"),
            guild_id: None,
            id: None,
            name: String::from(Self::NAME),
            options: vec![],
        }
    }

    async fn run(&self, state: &State) -> Result<InteractionResponse> {
        let ping = state
            .cluster()
            .info()
            .values()
            .filter_map(|info| info.latency().average())
            .sum::<Duration>()
            / state.cluster().shards().len().try_into()?;

        match ping.as_millis() {
            0 => Ok(Response::message(
                "Pong! Couldn't quite get average latency",
            )),
            ping => Ok(Response::message(format!(
                "Pong! Average latency is {} milliseconds",
                ping
            ))),
        }
    }
}
