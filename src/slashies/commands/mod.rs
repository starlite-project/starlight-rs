use crate::state::State;
use anyhow::Result;
use async_trait::async_trait;
use ping::Ping;
use twilight_model::application::{callback::InteractionResponse, command::Command, interaction::ApplicationCommand};

mod ping;

#[must_use]
pub fn get_slashies() -> [Command; 1] {
    [Ping::define()]
}

#[async_trait]
pub trait SlashCommand {
    const NAME: &'static str;

    async fn run(&self, state: State) -> Result<InteractionResponse>;

    fn define() -> Command;
}

#[derive(Debug, Clone)]
pub enum Commands {
    Ping(Ping),
}

impl Commands {
    #[must_use]
    pub fn r#match(command: ApplicationCommand) -> Option<Self> {
        match command.data.name.as_str() {
            Ping::NAME => Some(Self::Ping(Ping(command))),
            _ => None,
        }
    }

    pub async fn run(&self, state: State) -> Result<InteractionResponse> {
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
