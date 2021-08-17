use super::{ClickCommand, SlashCommand};
use crate::{
    components::{BuildError, ButtonBuilder, ComponentBuilder},
    slashies::{interaction::Interaction, Response},
    state::State,
    GetUserId,
};
use anyhow::Result;
use async_trait::async_trait;
use twilight_model::application::{
    command::{BaseCommandOptionData, Command, CommandOption},
    component::{button::ButtonStyle, Button},
    interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Info(pub(crate) ApplicationCommand);

impl<'a> Info {
    async fn run_buttons(&self, interaction: Interaction<'a>) -> Result<()> {
        let response = Response::new()
            .message("Select an option")
            .set_components(Self::components()?);

        interaction.response(response).await?;

        let click_data = Self::wait_for_click(
            interaction.state,
            interaction,
            interaction.command.user_id(),
        )
        .await?;

        dbg!(click_data);

        Ok(())
    }

    async fn run_user(&self, interaction: Interaction<'a>) -> Result<()> {
        let response = Response::from("User info: todo");

        interaction.response(response).await?;

        Ok(())
    }
}

#[async_trait]
impl SlashCommand<3> for Info {
    const NAME: &'static str = "info";

    fn define() -> Command {
        Command {
            application_id: None,
            guild_id: None,
            name: String::from(Self::NAME),
            default_permission: None,
            description: String::from("Gets info about a user, the guild, or myself!"),
            id: None,
            options: vec![CommandOption::User(BaseCommandOptionData {
                description: String::from("The user to get info for"),
                name: String::from("user"),
                required: false,
            })],
        }
    }

    async fn run(&self, state: State) -> Result<()> {
        let interaction = state.interaction(&self.0);

        if interaction.command.data.options.is_empty() {
            self.run_buttons(interaction).await
        } else {
            self.run_user(interaction).await
        }
    }
}

impl ClickCommand<3> for Info {
    type Output = ();

    fn define_buttons() -> Result<[Button; 3], BuildError> {
        let component_ids: [&'static str; 3] = Self::component_ids();
        let buttons = [
            ButtonBuilder::new()
                .custom_id(component_ids[0])
                .label("Author")
                .style(ButtonStyle::Primary)
                .build()?,
            ButtonBuilder::new()
                .custom_id(component_ids[1])
                .label("Me")
                .style(ButtonStyle::Success)
                .build()?,
            ButtonBuilder::new()
                .custom_id(component_ids[2])
                .label("Guild")
                .style(ButtonStyle::Danger)
                .build()?,
        ];

        Ok(buttons)
    }

    fn parse(_: State, _: &str) -> Self::Output {}
}
