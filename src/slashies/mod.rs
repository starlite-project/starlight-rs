pub mod commands;
mod r#impl;

use std::ops::Deref;

use twilight_model::{
	application::{
		callback::{Autocomplete, CallbackData},
		command::CommandOptionChoice,
		interaction::ApplicationCommand,
	},
	channel::{
		embed::Embed,
		message::{allowed_mentions::AllowedMentionsBuilder, MessageFlags},
	},
	id::{marker::UserMarker, Id},
};

pub use self::r#impl::{DefineCommand, SlashCommand};

#[derive(Debug, Clone)]
#[must_use = "SlashData has no side effects"]
pub struct SlashData {
	pub callback: CallbackData,
	pub command: ApplicationCommand,
	pub autocomplete: Autocomplete,
}

impl SlashData {
	pub const BASE: CallbackData = CallbackData {
		allowed_mentions: None,
		content: None,
		embeds: None,
		flags: None,
		components: None,
		tts: None,
	};

	pub const fn new(command: ApplicationCommand) -> Self {
		Self {
			callback: Self::BASE,
			command,
			autocomplete: Autocomplete { choices: vec![] },
		}
	}

	#[must_use]
	#[allow(clippy::option_if_let_else)]
	pub fn user_id(&self) -> Id<UserMarker> {
		if let Some(member) = &self.command.member {
			if let Some(user) = &member.user {
				user.id
			} else {
				panic!("failed to get user_id")
			}
		} else if let Some(user) = &self.command.user {
			user.id
		} else {
			panic!("failed to get user_id")
		}
	}

	#[must_use]
	pub const fn is_guild(&self) -> bool {
		self.command.guild_id.is_some()
	}

	#[must_use]
	pub const fn is_dm(&self) -> bool {
		!self.is_guild()
	}

	pub fn allowed_mentions<F: FnOnce(AllowedMentionsBuilder) -> AllowedMentionsBuilder>(
		&mut self,
		builder: F,
	) -> &mut Self {
		self.callback.allowed_mentions = Some(builder(AllowedMentionsBuilder::new()).build());

		self
	}

	pub fn message<T: AsRef<str>>(&mut self, content: T) -> &mut Self {
		assert!(!content.as_ref().is_empty(), "empty message not allowed");

		self.callback.content = Some(content.as_ref().to_owned());

		self
	}

	pub fn autocomplete(&mut self, choices: Vec<CommandOptionChoice>) -> &mut Self {
		self.autocomplete = Autocomplete { choices };

		self
	}

	pub fn embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
		assert!(!embeds.is_empty(), "empty embeds not allowed");

		if let Some(current_embeds) = &mut self.callback.embeds {
			current_embeds.extend(embeds);
		} else {
			self.callback.embeds = Some(embeds);
		}

		self
	}

	pub fn embed(&mut self, embed: Embed) -> &mut Self {
		self.embeds(vec![embed])
	}

	pub fn flags(&mut self, flags: MessageFlags) -> &mut Self {
		self.callback.flags = self
			.callback
			.flags
			.map_or(Some(flags), |current_flags| Some(flags | current_flags));

		self
	}

	pub fn ephemeral(&mut self) -> &mut Self {
		self.flags(MessageFlags::EPHEMERAL)
	}

	pub fn take(&mut self) -> Self {
		Self {
			callback: CallbackData {
				allowed_mentions: self.callback.allowed_mentions.take(),
				components: self.callback.components.take(),
				content: self.callback.content.take(),
				embeds: self.callback.embeds.clone(),
				flags: self.callback.flags.take(),
				tts: self.callback.tts.take(),
			},
			command: self.command.clone(),
			autocomplete: self.autocomplete.clone(),
		}
	}
}

impl Deref for SlashData {
	type Target = ApplicationCommand;

	fn deref(&self) -> &Self::Target {
		&self.command
	}
}
