use super::interaction::Interaction;
use crate::{
    components::{BuildError, ComponentBuilder},
    state::State,
};
use anyhow::Result;
use async_trait::async_trait;
use click::Click;
use ping::Ping;
use std::{
    any::type_name,
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    application::{
        command::Command,
        component::{Button, Component},
        interaction::{
            ApplicationCommand, Interaction as DiscordInteraction, MessageComponentInteraction,
        },
    },
    gateway::event::Event,
    id::UserId,
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

    async fn run(&self, state: State) -> Result<()>;

    fn define() -> Command;
}

const EMPTY: String = String::new();

#[async_trait]
pub trait ClickCommand<const N: usize>: SlashCommand<N> {
    fn define_components() -> Result<Vec<Component>, BuildError> {
        let buttons = Self::buttons()?.into_values().collect::<Vec<_>>();

        Ok(vec![buttons.build_component()?])
    }

    fn buttons() -> Result<HashMap<String, Button>, BuildError>;

    #[must_use]
    fn component_ids() -> [String; N] {
        let mut array = [EMPTY; N];

        // Needed cause clippy things this should be enumerated, and we don't actually need the values
        #[allow(clippy::needless_range_loop)]
        for i in 0..N {
            array[i] = format!("{}_{}", type_name::<Self>(), i);
        }

        array
    }

    async fn wait_for_click<'a>(
        state: State,
        interaction: Interaction<'a>,
        user_id: UserId,
    ) -> Result<MessageComponentInteraction> {
        let event = if let Some(guild_id) = interaction.command.guild_id {
            state
                .standby
                .wait_for(guild_id, move |event: &Event| {
                    if let Event::InteractionCreate(interaction_create) = event {
                        match &interaction_create.0 {
                            DiscordInteraction::MessageComponent(button) => {
                                Self::component_ids().contains(&button.data.custom_id)
                                    && button.author_id().unwrap_or_default() == user_id
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                })
                .await?
        } else {
            state
                .standby
                .wait_for_event(move |event: &Event| {
                    if let Event::InteractionCreate(interaction_create) = event {
                        match &interaction_create.0 {
                            DiscordInteraction::MessageComponent(button) => {
                                Self::component_ids().contains(&button.data.custom_id)
                                    && button.author_id().unwrap_or_default() == user_id
                            }
                            _ => false,
                        }
                    } else {
                        false
                    }
                })
                .await?
        };

        if let Event::InteractionCreate(interaction_create) = event {
            if let DiscordInteraction::MessageComponent(comp) = interaction_create.0 {
                Ok(*comp)
            } else {
                Err(ClickError.into())
            }
        } else {
            Err(ClickError.into())
        }
    }
}

#[derive(Debug)]
pub struct ClickError;

impl Display for ClickError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("an error occured getting the click data")
    }
}

impl Error for ClickError {}

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
