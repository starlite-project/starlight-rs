use super::SlashCommand;
use crate::{
    components::{ButtonBuilder, ComponentBuilder},
    slashies::Response,
    state::State,
};
use anyhow::Result;
use async_trait::async_trait;
use twilight_model::application::{
    command::Command, component::button::ButtonStyle, interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Click(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand for Click {
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

        let button = ButtonBuilder::new()
            .custom_id("Test")
            .label("Test")
            .style(ButtonStyle::Primary)
            .build_component()?;

        let response = Response::new().message("Click this").add_component(button);

        interaction.response(response).await?;

        Ok(())
    }
}
