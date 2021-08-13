use crate::state::State;
use anyhow::Result;
use async_trait::async_trait;
use click::Click;
use ping::Ping;
use twilight_model::application::{command::Command, interaction::ApplicationCommand};

mod click;
mod ping;

#[must_use]
pub fn get_slashies() -> [Command; 2] {
    [Ping::define(), Click::define()]
}

#[async_trait]
pub trait SlashCommand {
    const NAME: &'static str;

    async fn run(&self, state: State) -> Result<()>;

    fn define() -> Command;
}

#[derive(Debug, Clone)]
pub enum Commands {
    Ping(Ping),
    Click(Click),
}

impl Commands {
    #[must_use]
    pub fn r#match(command: ApplicationCommand) -> Option<Self> {
        match command.data.name.as_str() {
            Ping::NAME => Some(Self::Ping(Ping(command))),
            Click::NAME => Some(Self::Click(Click(command))),
            _ => None,
        }
    }

    pub async fn run(&self, state: State) -> Result<()> {
        match self {
            Self::Ping(c) => c.run(state).await,
            Self::Click(c) => c.run(state).await,
        }
    }
}
