use std::sync::atomic::{AtomicBool, Ordering};

use tracing::instrument;
use twilight_model::application::{
	callback::InteractionResponse,
	command::Command,
	interaction::{application_command::CommandData, ApplicationCommand, InteractionType},
};
use twilight_util::builder::command::CommandBuilder;

use super::Helpers;
use crate::{
	prelude::*,
	slashies::{
		commands::{Crate, Ping},
		DefineCommand, SlashCommand, SlashData,
	},
	state::Context,
};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

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

	pub async fn init(self) -> MietteResult<()> {
		if INITIALIZED.load(Ordering::SeqCst) {
			return Ok(());
		}
		let context = self.context();

		if let Some(guild_id) = context.config().guild_id {
			context
				.http()
				.set_guild_commands(guild_id, &Self::get_slashies())
				.into_diagnostic()?
				.exec()
				.await
		} else {
			context
				.http()
				.set_global_commands(&Self::get_slashies())
				.into_diagnostic()?
				.exec()
				.await
		}
		.into_diagnostic()?;

		INITIALIZED.store(true, Ordering::SeqCst);
		Ok(())
	}

	#[instrument(skip(self, command), fields(command.name = %command.data.name, command.guild_id))]
	pub async fn handle(self, command: ApplicationCommand) {
		if let Some(slashie) = Self::match_command(command.data.name.as_str(), command.data.clone())
		{
			let data = SlashData::new(command.clone());
			match command.kind {
				InteractionType::ApplicationCommand => {
					if let Err(e) = slashie.run(self, data).await {
						event!(
							Level::ERROR,
							error = &*e.root_cause(),
							"error running command"
						);
					}
				}
				InteractionType::ApplicationCommandAutocomplete => {
					if let Err(e) = slashie.autocomplete(self, data).await {
						event!(
							Level::ERROR,
							error = &*e.root_cause(),
							"error running autocomplete"
						);
					}
				}
				_ => {}
			}
		} else {
			event!(Level::WARN, "received unregistered command");
		}
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

	pub async fn autocomplete(self, data: &SlashData) -> Result<(), HttpError> {
		let autocomplete_data = data.autocomplete.clone();
		let context = self.context();
		context
			.http()
			.interaction_callback(
				data.command.id,
				&data.command.token,
				&InteractionResponse::Autocomplete(autocomplete_data),
			)
			.exec()
			.await?;

		Ok(())
	}

	fn match_command(name: &str, data: CommandData) -> Option<Box<dyn SlashCommand>> {
		match name {
			"ping" => Some(Box::new(Ping {})),
			"crate" => Some(Box::new(Crate::parse(data).unwrap())),
			_ => None,
		}
	}

	fn get_slashies() -> [Command; 2] {
		[Ping::define(), Crate::define()].map(CommandBuilder::build)
	}
}
