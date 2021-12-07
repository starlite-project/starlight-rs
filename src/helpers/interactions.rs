use std::{
	mem,
	sync::atomic::{AtomicBool, Ordering},
};

use starlight_macros::model;
use tracing::instrument;
use twilight_http::request::application::{
	interaction::{CreateFollowupMessage, UpdateFollowupMessage},
	InteractionError,
};
use twilight_model::{
	application::{
		callback::{Autocomplete, InteractionResponse},
		command::Command,
		interaction::{application_command::CommandData, ApplicationCommand, InteractionType},
	},
	channel::Message,
};
use twilight_util::builder::command::CommandBuilder;

use super::Helpers;
use crate::{
	prelude::*,
	slashies::{
		commands::{Crate, Ping, Play},
		DefineCommand, SlashCommand, SlashData,
	},
	state::{Context, QuickAccess},
};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
#[must_use = "an InteractionsHelper does nothing if not used"]
pub struct InteractionsHelper(Helpers);

impl InteractionsHelper {
	pub(super) const fn new(helpers: Helpers) -> Self {
		Self(helpers)
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

						let mut err_data = SlashData::new(command);

						err_data
							.message("an error occurred running the interaction")
							.ephemeral();

						if self.raw_get(&err_data).await.is_err() {
							self.respond(&mut err_data).await.unwrap();
						} else {
							self.update(&mut err_data).await.unwrap();
						}
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

	pub async fn respond(self, data: &mut SlashData) -> Result<(), HttpError> {
		self.context()
			.http()
			.interaction_callback(
				data.command.id,
				&data.command.token,
				&InteractionResponse::ChannelMessageWithSource(mem::replace(
					&mut data.callback,
					SlashData::BASE,
				)),
			)
			.exec()
			.await?;

		Ok(())
	}

	pub async fn update(self, data: &mut SlashData) -> MietteResult<()> {
		let callback_data = mem::replace(&mut data.callback, SlashData::BASE);
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

	pub async fn autocomplete(self, data: &mut SlashData) -> Result<(), HttpError> {
		let autocomplete_data =
			mem::replace(&mut data.autocomplete, Autocomplete { choices: vec![] });
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

	pub fn raw_create<'a>(
		&'a self,
		data: &'a SlashData,
	) -> Result<CreateFollowupMessage<'a>, InteractionError> {
		self.http().create_followup_message(&data.command.token)
	}

	pub async fn raw_update<'a>(
		&'a self,
		data: &'a SlashData,
	) -> MietteResult<UpdateFollowupMessage<'a>> {
		let http = self.http();
		let original_message_id = self.raw_get(data).await?.id;

		http.update_followup_message(&data.command.token, original_message_id)
			.into_diagnostic()
	}

	pub async fn raw_get(self, data: &SlashData) -> MietteResult<Message> {
		let http = self.http();
		let get_original = http
			.get_interaction_original(&data.command.token)
			.into_diagnostic()?;

		model!(get_original).await.into_diagnostic()
	}

	fn match_command(name: &str, data: CommandData) -> Option<Box<dyn SlashCommand>> {
		match name {
			"ping" => Some(Box::new(Ping {})),
			"crate" => Some(Box::new(Crate::parse(data).unwrap())),
			"play" => Some(Box::new(Play::parse(data).unwrap())),
			_ => None,
		}
	}

	fn get_slashies() -> [Command; 3] {
		[Ping::define(), Crate::define(), Play::define()].map(CommandBuilder::build)
	}
}

impl QuickAccess for InteractionsHelper {
	fn context(&self) -> Context {
		self.0.context()
	}
}
