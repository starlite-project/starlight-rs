use crate::lib::state::State;
use async_trait::async_trait;
use twilight_model::application::{
    callback::InteractionResponse, command::Command as SlashCommand,
};

use super::{PartialApplicationCommand, Response};

#[async_trait]
pub trait Command {
    const NAME: &'static str;

    async fn run(&self, ctx: &State) -> Result<InteractionResponse, ()>;

    fn define() -> SlashCommand;
}

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

    async fn run(&self, _: &State) -> Result<InteractionResponse, ()> {
        Ok(Response::message("Pong!"))
    }
}
