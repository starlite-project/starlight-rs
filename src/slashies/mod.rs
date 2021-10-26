#![allow(dead_code)]

use self::commands::Commands;
use super::{components::ActionRowBuilder, state::State};
use crate::{
	components::ComponentBuilder, helpers::Color, utils::constants::SlashiesErrorMessages,
};
use tracing::{event, instrument, Level};
use twilight_embed_builder::EmbedBuilder;
use twilight_model::{
	application::{
		callback::{CallbackData, InteractionResponse},
		component::{Component, ComponentType},
		interaction::application_command::ApplicationCommand,
	},
	channel::{
		embed::Embed,
		message::{allowed_mentions::AllowedMentionsBuilder, MessageFlags},
	},
};

pub mod commands;
mod r#impl;
pub mod interaction;

pub use self::r#impl::{ClickCommand, ClickError, ParseCommand, ParseError, SlashCommand};

#[derive(Debug, Clone)]
pub struct Response(CallbackData);

impl Response {
	const BASE: CallbackData = CallbackData {
		allowed_mentions: None,
		content: None,
		embeds: vec![],
		flags: None,
		components: None,
		tts: None,
	};

	#[must_use]
	pub const fn new() -> Self {
		Self(Self::BASE)
	}

	#[must_use]
	pub const fn ack() -> InteractionResponse {
		InteractionResponse::DeferredChannelMessageWithSource(Self::BASE)
	}

	pub fn add_component(&mut self, component: Component) -> &mut Self {
		self.add_components(vec![component])
	}

	#[allow(clippy::option_if_let_else)]
	pub fn add_components(&mut self, components: Vec<Component>) -> &mut Self {
		let components = Self::check_components(components);
		if let Some(ref mut current) = self.0.components {
			current.extend(components);

			self
		} else {
			self.set_components(components)
		}
	}

	pub fn allowed_mentions<F: FnOnce(AllowedMentionsBuilder) -> AllowedMentionsBuilder>(
		mut self,
		builder: F,
	) -> Self {
		self.0.allowed_mentions = Some(builder(AllowedMentionsBuilder::new()).build());

		self
	}

	pub fn set_components(&mut self, components: Vec<Component>) -> &mut Self {
		let components = Self::check_components(components);
		self.0.components = Some(components);
		self
	}

	pub fn clear_components(&mut self) -> &mut Self {
		self.0.components = Some(vec![]);

		self
	}

	pub fn message<T: AsRef<str>>(&mut self, content: T) -> &mut Self {
		assert!(!content.as_ref().is_empty(), "empty message not allowed");

		self.0.content = Some(content.as_ref().to_owned());

		self
	}

	pub fn embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
		assert!(!embeds.is_empty(), "empty embeds not allowed");

		self.0.embeds.extend(embeds);

		self
	}

	pub fn embed(&mut self, embed: Embed) -> &mut Self {
		self.embeds(vec![embed])
	}

	pub fn flags(&mut self, flags: MessageFlags) -> &mut Self {
		self.0.flags = self
			.0
			.flags
			.map_or(Some(flags), |current_flags| Some(flags | current_flags));

		self
	}

	pub fn ephemeral(&mut self) -> &mut Self {
		self.flags(MessageFlags::EPHEMERAL)
	}

	#[must_use]
	pub fn error(message: SlashiesErrorMessages) -> InteractionResponse {
		Self::from(message.to_string()).exec()
	}

	#[allow(clippy::missing_const_for_fn)]
	#[must_use]
	pub fn exec(self) -> InteractionResponse {
		InteractionResponse::ChannelMessageWithSource(self.0)
	}

	pub fn take(&mut self) -> Self {
		Self(CallbackData {
			allowed_mentions: self.0.allowed_mentions.take(),
			components: self.0.components.take(),
			content: self.0.content.take(),
			embeds: self.0.embeds.clone(),
			flags: self.0.flags.take(),
			tts: self.0.tts.take(),
		})
	}

	fn check_component(component: Component) -> Component {
		if component.kind() == ComponentType::ActionRow {
			component
		} else {
			ActionRowBuilder::new()
				.push_component(component)
				.build_component()
				.unwrap()
		}
	}

	fn check_components(components: Vec<Component>) -> Vec<Component> {
		components.into_iter().map(Self::check_component).collect()
	}
}

impl From<&str> for Response {
	fn from(message: &str) -> Self {
		Self::new().message(message).take()
	}
}

impl From<String> for Response {
	fn from(message: String) -> Self {
		Self::new().message(message.as_str()).take()
	}
}

impl From<Embed> for Response {
	fn from(embed: Embed) -> Self {
		Self::new().embed(embed).take()
	}
}

impl From<Vec<Embed>> for Response {
	fn from(embeds: Vec<Embed>) -> Self {
		Self::new().embeds(embeds).take()
	}
}

impl From<Response> for InteractionResponse {
	fn from(response: Response) -> Self {
		response.exec()
	}
}

impl From<&mut Response> for InteractionResponse {
	fn from(response: &mut Response) -> Self {
		response.take().exec()
	}
}

impl From<Response> for CallbackData {
	fn from(response: Response) -> Self {
		response.0
	}
}

impl From<&mut Response> for CallbackData {
	fn from(response: &mut Response) -> Self {
		response.take().0
	}
}

#[instrument(skip(state, command), fields(command.name = %command.data.name, command.guild_id))]
pub async fn act(state: State, command: ApplicationCommand) {
	if let Some(cmd) = Commands::r#match(&command) {
		let interaction = state.interaction(&command);
		if let Err(e) = cmd.run(interaction).await {
			event!(
				Level::ERROR,
				error = &*e.root_cause(),
				"error running command"
			);
			let mut error_response =
				Response::from(SlashiesErrorMessages::InteractionError.to_string());
			let embed_builder = EmbedBuilder::new().color(Color::new(255, 0, 0).to_decimal()).title("Error")
			.description(format!("```\n{}\t\n```", e.root_cause()));
			error_response.embed(unsafe { embed_builder.build().unwrap_unchecked() });
			interaction
				.response(error_response)
				.await
				.expect("unknown failure");
		}
	} else {
		event!(Level::WARN, "received unregistered command");
	}
}
