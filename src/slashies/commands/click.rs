use crate::{
	slashies::{
		interaction::Interaction, ClickCommand, ParseCommand, ParseError, Response, SlashCommand,
	},
	state::State,
	utils::interaction_author,
};
use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use twilight_model::application::{
	command::{Command, CommandType},
	interaction::ApplicationCommand,
};

#[derive(Debug, Clone, ClickCommand)]
#[buttons(Link("To me!", "https://github.com/pyrotechniac/starlight-rs"), Success("A button!"), Danger("Another button!"))]
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
			kind: CommandType::ChatInput,
		}
	}

	async fn run(&self, state: State) -> Result<()> {
		let interaction = state.interaction(&self.0);

		let response = Response::new()
			.message("Click this")
			.add_components(Self::components().into_diagnostic()?);

		interaction.response(response).await.into_diagnostic()?;

		let click_data =
			Self::wait_for_click(interaction, interaction_author(interaction.command)).await?;

		interaction
			.update()
			.into_diagnostic()?
			.content(Some(
				format!(
					"Success! You clicked {}",
					Self::parse(interaction, &click_data.data.custom_id).into_diagnostic()?
				)
				.as_str(),
			))
			.into_diagnostic()?
			.components(Self::EMPTY_COMPONENTS)
			.into_diagnostic()?
			.exec()
			.await
			.into_diagnostic()?;

		Ok(())
	}
}

impl ParseCommand<3> for Click {
	type Output = String;

	fn parse(_: Interaction, input: &str) -> Result<Self::Output, ParseError> {
		let components = Self::define_buttons().map_err(|_| ParseError)?;

		components
			.iter()
			.cloned()
			.find(|button| button.custom_id.as_deref() == Some(input))
			.and_then(|button| button.label)
			.ok_or(ParseError)
	}
}
