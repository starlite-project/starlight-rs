use super::{PartialApplicationCommand, Response};
use crate::state::State;
use anyhow::Result;
use async_trait::async_trait;
use std::convert::TryInto;
use twilight_model::application::{
    callback::InteractionResponse, command::Command as SlashCommand,
};

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
        let info = state
            .cluster()
            .info()
            .values()
            .map(|info| info.latency().average())
            .filter(|val| val.is_some())
            .map(|val| val.unwrap())
            .collect::<Vec<_>>();

        let shard_length = state.cluster().shards().len();

        let ping = info
            .iter()
            .cloned()
            .reduce(|acc, val| acc + val)
            .unwrap_or_default()
            / shard_length.try_into()?;

        if ping.as_millis() == 0 {
            Ok(Response::message(format!(
                "Pong! Couldn't quite get average latency"
            )))
        } else {
            Ok(Response::message(format!(
                "Pong! Average latency is {} milliseconds",
                ping.as_millis()
            )))
        }
    }
}

pub fn commands() -> Vec<SlashCommand> {
    vec![Ping::define()]
}
