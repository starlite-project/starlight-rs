use super::{ClickCommand, SlashCommand};
use crate::{
    components::{ActionRowBuilder, BuildError, ComponentBuilder},
    debug_unreachable,
    slashies::Response,
    state::State,
    GetUserId,
};
use anyhow::Result;
use async_trait::async_trait;
use twilight_model::application::{
    command::Command,
    component::{button::ButtonStyle, ActionRow, Component},
    interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Click(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand<2> for Click {
    const NAME: &'static str = "click";

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

        let response = Response::new()
            .message("Click this")
            .add_components(Self::components()?);

        interaction.response(response).await?;

        let click_data =
            Self::wait_for_click(state, interaction, interaction.command.user_id()).await?;

        interaction
            .update()?
            .content(Some(
                format!(
                    "Success! You clicked {}",
                    Self::parse(&click_data.data.custom_id)
                )
                .as_str(),
            ))?
            .components(Some(&[]))?
            .exec()
            .await?;

        Ok(())
    }
}

#[async_trait]
impl ClickCommand<2> for Click {
    type Output = String;

    fn parse(value: &str) -> Self::Output {
        let components = Self::components().unwrap_or_else(|_| debug_unreachable!());

        components
            .iter()
            .map(|comp| match comp {
                Component::Button(button) => button.clone(),
                _ => debug_unreachable!(),
            })
            .find(|button| {
                *button
                    .custom_id
                    .as_ref()
                    .unwrap_or_else(|| debug_unreachable!())
                    == value
            })
            .unwrap_or_else(|| debug_unreachable!())
            .label
            .unwrap()
    }

    fn define_buttons() -> Result<ActionRow, BuildError> {
        let component_ids = Self::component_ids();
    
        let action_row = ActionRowBuilder::new()
            .create_button(|mut builder| {
                builder
                    .custom_id(component_ids[0].clone())
                    .label("A button")
                    .style(ButtonStyle::Success)
            })
            .create_button(|mut builder| {
                builder
                    .custom_id(component_ids[1].clone())
                    .label("Another button!")
                    .style(ButtonStyle::Danger)
            })
            .build()?;

        Ok(action_row)
    }
}
