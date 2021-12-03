use twilight_model::application::{callback::InteractionResponse, interaction::ApplicationCommand};

use super::Helpers;
use crate::{
	prelude::*,
	slashies::{commands::Ping, SlashCommand, SlashData},
	state::Context,
};

#[derive(Debug, Clone, Copy)]
#[must_use = "an InteractionsHelper does nothing if not used"]
pub struct InteractionsHelper(Helpers);

impl InteractionsHelper {
	pub(super) const fn new(helpers: Helpers) -> Self {
		Self(helpers)
	}

	#[must_use]
	pub const fn context(self) -> Context {
		self.0.context()
	}

	pub async fn handle(self, command: ApplicationCommand) -> MietteResult<()> {
		Ok(())
	}

	pub async fn ack(self, data: &SlashData) -> Result<(), HttpError> {
		self.context()
			.http()
			.interaction_callback(
				data.command.id,
				&data.command.token,
				&InteractionResponse::DeferredChannelMessageWithSource(SlashData::BASE),
			)
			.exec()
			.await?;

		Ok(())
	}

	pub async fn respond(self, data: &SlashData) -> Result<(), HttpError> {
		self.context()
			.http()
			.interaction_callback(
				data.command.id,
				&data.command.token,
				&InteractionResponse::ChannelMessageWithSource(data.callback.clone()),
			)
			.exec()
			.await?;

		Ok(())
	}

	pub async fn update(self, data: &SlashData) -> MietteResult<()> {
		let callback_data = data.callback.clone();
		let context = self.context();
		let update_interaction = context
			.http()
			.update_interaction_original(&data.command.token)
			.into_diagnostic()?;

		let bytes = serde_json::to_vec(&callback_data).into_diagnostic()?;

		update_interaction
			.payload_json(&bytes[..])
			.exec()
			.await
			.into_diagnostic()?;

		Ok(())
	}
}
