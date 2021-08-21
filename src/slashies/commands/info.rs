use super::SlashCommand;
use crate::{slashies::Response, state::State};
use anyhow::Result;
use async_trait::async_trait;
use twilight_model::application::{
    command::{BaseCommandOptionData, Command, CommandOption},
    interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Info(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand<0> for Info {
    const NAME: &'static str = "info";

    fn define() -> Command {
        Command {
            application_id: None,
            guild_id: None,
            name: String::from(Self::NAME),
            default_permission: None,
            description: String::from("Get info about a user"),
            id: None,
            options: vec![CommandOption::User(BaseCommandOptionData {
                name: String::from("user"),
                description: String::from(
                    "The user to get information about, defaulting to the author",
                ),
                required: false,
            })],
        }
    }

    async fn run(&self, state: State) -> Result<()> {
        let interaction = state.interaction(&self.0);

        dbg!(interaction);

        interaction.response(Response::from("todo")).await?;

        Ok(())
    }
}
