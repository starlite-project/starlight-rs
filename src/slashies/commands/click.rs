use super::{ClickCommand, SlashCommand};
use crate::{
    components::{BuildError, ButtonBuilder, ComponentBuilder},
    debug_unreachable,
    slashies::{interaction::Interaction, Response},
    state::State,
    InteractionAuthor,
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
            .add_components(Self::components()?);

        interaction.response(response).await?;

        let click_data =
            Self::wait_for_click(state, interaction, interaction.command.interaction_author())
                .await?;

        interaction
            .update()?
            .content(Some(
                format!(
                    "Success! You clicked {}",
                    Self::parse(interaction, &click_data.data.custom_id)
                )
                .as_str(),
            ))?
            .components(Self::EMPTY_COMPONENTS)?
            .exec()
            .await?;

        Ok(())
    }
}

#[async_trait]
impl ClickCommand<2> for Click {
    type Output = String;

    fn parse(_: Interaction<'_>, value: &str) -> Self::Output {
        let components = Self::define_buttons().unwrap_or_else(|_| debug_unreachable!());

        components
            .iter()
            .find(|button| button.custom_id.as_deref() == Some(value))
            .unwrap_or_else(|| debug_unreachable!())
            .label
            .clone()
            .unwrap()
    }

    fn define_buttons() -> Result<[Button; 2], BuildError> {
        let component_ids = Self::COMPONENT_IDS;

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
