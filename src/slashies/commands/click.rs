use super::{ClickCommand, SlashCommand};
use crate::{
    components::{BuildError, ButtonBuilder, ComponentBuilder},
    debug_unreachable,
    slashies::Response,
    state::State,
    GetUserId,
};
use anyhow::Result;
use async_trait::async_trait;
use twilight_model::application::{
    command::Command,
    component::{button::ButtonStyle, Button},
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
            .add_components(Self::components()?.into_iter().collect());

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
        let components = Self::define_buttons().unwrap_or_else(|_| debug_unreachable!());

        components
            .iter()
            .find(|button| {
                *button
                    .custom_id
                    .as_ref()
                    .unwrap_or_else(|| debug_unreachable!())
                    == value
            })
            .unwrap_or_else(|| debug_unreachable!())
            .label
            .clone()
            .unwrap()
    }

    fn define_buttons() -> Result<[Button; 2], BuildError> {
        let component_ids = Self::component_ids();

        Ok([
            ButtonBuilder::new()
                .custom_id(component_ids[0])
                .label("A button")
                .style(ButtonStyle::Success)
                .build()?,
            ButtonBuilder::new()
                .custom_id(component_ids[1])
                .label("Another button!")
                .style(ButtonStyle::Danger)
                .build()?,
        ])
    }
}
