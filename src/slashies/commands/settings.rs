use crate::{
	persistence::settings::{GuildHelper, SettingsHelper},
	slashies::{Response, SlashCommand},
	state::State,
	utils::{constants::SlashiesErrorMessages, CacheReliant},
};
use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use nebula::ToIdKey;
use twilight_cache_inmemory::ResourceType;
use twilight_model::application::{
	command::{Command, CommandType},
	interaction::ApplicationCommand,
};

#[derive(Debug, Clone)]
pub struct Settings(pub(super) ApplicationCommand);

impl CacheReliant for Settings {
	fn needs() -> ResourceType {
		ResourceType::GUILD
	}
}

#[async_trait]
impl SlashCommand for Settings {
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

		let guild_key = if let Some(key) = interaction.command.guild_id {
			key.to_id_key()
		} else {
			interaction
				.response(Response::error(SlashiesErrorMessages::GuildOnly))
				.await
				.into_diagnostic()?;

			return Ok(());
		};

		let guild_settings = interaction
			.database()
			.helper::<GuildHelper>()
			.acquire(guild_key)
			.into_diagnostic()?;

		let string = dbg!(format!("{:?}", guild_settings));

		interaction
			.response(Response::from(string))
			.await
			.into_diagnostic()?;

		Ok(())
	}
}
