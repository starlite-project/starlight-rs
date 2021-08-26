#![allow(dead_code)]

use self::commands::Commands;
use super::{components::ActionRowBuilder, state::State};
use crate::components::ComponentBuilder;
use tracing::{event, instrument, Level};
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
pub mod interaction;

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

	#[must_use]
	pub fn add_component(self, component: Component) -> Self {
		self.add_components(vec![component])
	}

	#[allow(clippy::option_if_let_else)]
	#[must_use]
	pub fn add_components(mut self, components: Vec<Component>) -> Self {
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

	#[must_use]
	pub fn set_components(mut self, components: Vec<Component>) -> Self {
		let components = Self::check_components(components);
		self.0.components = Some(components);
		self
	}

	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn clear_components(mut self) -> Self {
		self.0.components = None;

		self
	}

	pub fn message<T: AsRef<str>>(mut self, content: T) -> Self {
		if content.as_ref().is_empty() {
			panic!("empty message not allowed");
		}

		self.0.content = Some(content.as_ref().to_owned());

		self
	}

	#[must_use]
	pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
		if embeds.is_empty() {
			panic!("empty embeds not allowed");
		}

		self.0.embeds.extend(embeds);

		self
	}

	#[must_use]
	pub fn embed(self, embed: Embed) -> Self {
		self.embeds(vec![embed])
	}

	#[must_use]
	pub fn flags(mut self, flags: MessageFlags) -> Self {
		self.0.flags = self
			.0
			.flags
			.map_or(Some(flags), |current_flags| Some(flags | current_flags));

		self
	}

	#[must_use]
	pub fn ephemeral(self) -> Self {
		self.flags(MessageFlags::EPHEMERAL)
	}

	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn exec(self) -> InteractionResponse {
		InteractionResponse::ChannelMessageWithSource(self.0)
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
		Self::new().message(message)
	}
}

impl From<String> for Response {
	fn from(message: String) -> Self {
		Self::new().message(message.as_str())
	}
}

impl From<Embed> for Response {
	fn from(embed: Embed) -> Self {
		Self::new().embed(embed)
	}
}

impl From<Vec<Embed>> for Response {
	fn from(embeds: Vec<Embed>) -> Self {
		Self::new().embeds(embeds)
	}
}

impl From<Response> for InteractionResponse {
	fn from(value: Response) -> Self {
		value.exec()
	}
}

#[instrument(skip(state, command), fields(command.name = %command.data.name, command.guild_id))]
pub async fn act(state: State, command: ApplicationCommand) {
	if let Some(cmd) = Commands::r#match(command) {
		if let Err(e) = cmd.run(state).await {
			event!(
				Level::ERROR,
				error = &*e as &dyn std::error::Error,
				"error running command"
			);
		}
	} else {
		event!(Level::WARN, "received unregistered command");
	}
}
