use super::interaction::Interaction;
use crate::{components::BuildError, state::State};
use anyhow::Result;
use async_trait::async_trait;
use click::Click;
use ping::Ping;
use std::any::type_name;
use twilight_model::{
    application::{
        command::Command,
        component::Component,
        interaction::{ApplicationCommand, MessageComponentInteraction},
    },
    gateway::event::Event,
};

mod click;
mod ping;

#[must_use]
pub fn get_slashies() -> [Command; 2] {
    [Ping::define(), Click::define()]
}

#[async_trait]
pub trait SlashCommand<const N: usize> {
    const NAME: &'static str;

    const COMPONENT_IDS: [&'static str; N];

    async fn run(&self, state: State) -> Result<()>;

    fn define() -> Command;
}

const EMPTY: String = String::new();

#[async_trait]
pub trait ClickCommand<const N: usize>: SlashCommand<N> {
    fn define_components(&self) -> Result<Vec<Component>, BuildError>;

    fn component_ids<'a>() -> [String; N] {
        let mut array = [EMPTY; N];

        for i in 0..N {
            array[i] = format!("{}_{}", type_name::<Self>(), i);
        }

        array
    }

    async fn wait_for_click<'a>(
        &self,
        state: State,
        interaction: Interaction<'a>,
    ) -> Result<MessageComponentInteraction> {
        let event = if let Some(guild_id) = interaction.command.guild_id {
            state
                .standby
                .wait_for(guild_id, |event: &Event| {
                    if let Event::InteractionCreate(interaction_create) = event {
                    } else {
                        false
                    }
                })
                .await?
        } else {
            todo!()
        };

        todo!()
    }
}

impl<T: SlashCommand<0>> !ClickCommand<0> for T {}

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
