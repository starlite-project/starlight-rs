use super::{ClickCommand, SlashCommand};
use crate::{
    components::{ActionRowBuilder, BuildError, ButtonBuilder, ComponentBuilder},
    slashies::Response,
    state::State,
};
use anyhow::Result;
use async_trait::async_trait;
use twilight_gateway::Event;
use twilight_model::application::{
    command::Command,
    component::{button::ButtonStyle, Component},
    interaction::{ApplicationCommand, Interaction, MessageComponentInteraction},
};

#[derive(Debug, Clone)]
pub struct Click(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand<2> for Click {
    const NAME: &'static str = "click";

    const COMPONENT_IDS: [&'static str; 2] = ["click_two", "click_one"];

    fn define() -> Command {
        Command {
            application_id: None,
            default_permission: None,
            description: String::from("Sends a clickyboi"),
            guild_id: None,
            id: None,
            name: String::from(Self::NAME),
            options: vec![],
        }
    }

    async fn run(&self, state: State) -> Result<()> {
        let interaction = state.interaction(&self.0);

        let button = ButtonBuilder::new()
            .custom_id(Self::COMPONENT_IDS[0])
            .label("Test")
            .style(ButtonStyle::Primary)
            .build()?;

        let second_button = ButtonBuilder::new()
            .custom_id(Self::COMPONENT_IDS[1])
            .label("Test as well")
            .style(ButtonStyle::Secondary)
            .build()?;

        let row = ActionRowBuilder::from(vec![button, second_button]).build_component()?;

        let response = Response::new()
            .message("Click this")
            .add_component(row.clone());

        interaction.response(response).await?;

        let click = if let Some(guild_id) = interaction.command.guild_id {
            state
                .standby
                .wait_for(guild_id, |event: &Event| {
                    if let Event::InteractionCreate(interaction_create) = event {
                        match &interaction_create.0 {
                            Interaction::MessageComponent(button) => {
                                Self::COMPONENT_IDS.contains(&button.data.custom_id.as_str())
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
                .wait_for_event(|event: &Event| {
                    if let Event::InteractionCreate(interaction) = event {
                        match &interaction.0 {
                            Interaction::MessageComponent(_) => true,
                            _ => false,
                        }
                    } else {
                        false
                    }
                })
                .await?
        };

        let _click = match click {
            Event::InteractionCreate(interaction) => match &interaction.0 {
                Interaction::MessageComponent(comp) => *comp.clone(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        interaction
            .update()?
            .content(Some("success!"))?
            .components(Some(&[]))?
            .exec()
            .await?;

        Ok(())
    }
}

#[async_trait]
impl ClickCommand<2> for Click {
    fn define_components(&self) -> Result<Vec<Component>, BuildError> {
        Ok(vec![ActionRowBuilder::new()
            .create_button(|builder| {
                builder
                    .custom_id(Self::COMPONENT_IDS[0])
                    .label("A button")
                    .style(ButtonStyle::Primary)
            })
            .create_button(|builder| {
                builder
                    .custom_id(Self::COMPONENT_IDS[1])
                    .label("Another button!")
                    .style(ButtonStyle::Danger)
            })
            .build_component()?])
    }
}
