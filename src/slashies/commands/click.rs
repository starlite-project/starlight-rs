use std::collections::HashMap;

use super::{ClickCommand, SlashCommand};
use crate::{
    components::{BuildError, ButtonBuilder, ComponentBuilder},
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
            .add_components(Self::define_components()?);

        interaction.response(response).await?;

        let click_data =
            Self::wait_for_click(state, interaction, interaction.command.user_id()).await?;

        let buttons = Self::buttons()?;

        interaction
            .update()?
            .content(Some(
                format!(
                    "Success! You clicked {}",
                    buttons[&click_data.data.custom_id].label.as_ref().unwrap()
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
    fn buttons() -> Result<HashMap<String, Button>, BuildError> {
        let component_ids = Self::component_ids();
        let mut map = HashMap::new();

        map.insert(
            component_ids[0].clone(),
            ButtonBuilder::new()
                .custom_id(component_ids[0].clone())
                .label("A button")
                .style(ButtonStyle::Success)
                .build()?,
        );

        map.insert(
            component_ids[1].clone(),
            ButtonBuilder::new()
                .custom_id(component_ids[1].clone())
                .label("Another button!")
                .style(ButtonStyle::Danger)
                .build()?,
        );

        Ok(map)
    }
}
