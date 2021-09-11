use super::SlashCommand;
use crate::{persistence::settings::SettingsHelper, slashies::Response, state::State};
use anyhow::Result;
use async_trait::async_trait;
use twilight_model::application::{
	command::{Command, CommandType},
	interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Settings(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand<0> for Settings {
	const NAME: &'static str = "settings";

	fn define() -> Command {
		Command {
			application_id: None,
			guild_id: None,
			name: String::from(Self::NAME),
			default_permission: None,
			description: String::from("Sets the settings for the guild"),
			id: None,
			kind: CommandType::ChatInput,
			options: vec![],
		}
	}

	async fn run(&self, state: State) -> Result<()> {
		let interaction = state.interaction(&self.0);

        let settings = interaction.database().guilds().acquire(interaction.command.guild_id.unwrap_or_else(|| {
            panic!()
        }))?;

		interaction.response(Response::from("todo")).await?;

        

		Ok(())
	}
}
