use crate::{
	slashies::{
		interaction::Interaction, ClickCommand, ParseCommand, ParseError, Response, SlashCommand,
	},
	utils::interaction_author,
};
use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use twilight_model::application::{
	command::CommandType,
	interaction::{ApplicationCommand, MessageComponentInteraction},
};
use twilight_util::builder::command::CommandBuilder;

#[derive(Debug, Clone, ClickCommand)]
#[buttons(
	Link("To me!", "https://github.com/pyrotechniac/starlight-rs"),
	Success("A button!"),
	Danger("Another button!"),
	Success("Even more buttons!"),
	Primary("Testing many buttons"),
	Secondary("Last one!")
)]
pub struct Click(pub(super) ApplicationCommand);

#[async_trait]
impl SlashCommand for Click {
	const NAME: &'static str = "click";

	fn define() -> CommandBuilder {
		CommandBuilder::new(
			Self::NAME.to_owned(),
			"Sends a clicyboi".to_owned(),
			CommandType::ChatInput,
		)
	}

	async fn run(&self, interaction: Interaction<'_>) -> Result<()> {
		let response = Response::new()
			.message("Click this")
			.add_components(Self::components().into_diagnostic()?)
			.take();

		interaction.response(response).await.into_diagnostic()?;

		let click_data: MessageComponentInteraction = if let Ok(res) =
			Self::wait_for_click(interaction, interaction_author(interaction.command), 10).await
		{
			res
		} else {
			let response = Response::from("Uh oh! button timed out".to_owned())
				.clear_components()
				.take();

			return interaction.update(response).await;
		};

		let response = Response::from(format!(
			"Success! You clicked {}",
			Self::parse(interaction, &click_data.data.custom_id).into_diagnostic()?
		))
		.clear_components()
		.take();

		interaction.update(response).await?;

		Ok(())
	}
}

impl ParseCommand for Click {
	type Output = String;

	fn parse(_: Interaction, input: &str) -> Result<Self::Output, ParseError> {
		let button = Self::parse_button(input)?;

		button.label.ok_or(ParseError::Custom(
			"an error occurred while getting the button pressed (this shouldn't happen)",
		))
	}
}
