use crate::state::State;

use super::SlashCommand;
use anyhow::Result;
use async_trait::async_trait;
use twilight_model::application::{
    command::{BaseCommandOptionData, Command, CommandOption},
    interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Info(pub(crate) ApplicationCommand);

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

        dbg!(interaction);

        Ok(())
    }
}
