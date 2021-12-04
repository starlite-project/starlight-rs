use std::pin::Pin;

use futures_util::Future;
use twilight_model::{
	application::{callback::CallbackData, interaction::ApplicationCommand},
	channel::{
		embed::Embed,
		message::{allowed_mentions::AllowedMentionsBuilder, MessageFlags},
	},
};

use crate::{helpers::InteractionsHelper, prelude::*};

pub trait SlashCommand: Send + Sync {
	fn run<'a>(
		&'a self,
		helper: InteractionsHelper,
		responder: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>> + Send + 'a>>;

	fn autocomplete<'a>(&'a self, helper: InteractionsHelper, responder: SlashData) ->  Pin<Box<dyn Future<Output = MietteResult<()>> + Send + 'a>> {
		Box::pin(async {
			Ok(())
		})
	}
}

#[derive(Debug, Clone)]
#[must_use = "SlashData has no side effects"]
pub struct SlashData {
	pub callback: CallbackData,
	pub command: ApplicationCommand,
}

impl SlashData {
	pub const BASE: CallbackData = CallbackData {
		allowed_mentions: None,
		content: None,
		embeds: vec![],
		flags: None,
		components: None,
		tts: None,
	};

	pub const fn new(command: ApplicationCommand) -> Self {
		Self {
			callback: Self::BASE,
			command,
		}
	}

	pub fn allowed_mentions<F: FnOnce(AllowedMentionsBuilder) -> AllowedMentionsBuilder>(
		mut self,
		builder: F,
	) -> Self {
		self.callback.allowed_mentions = Some(builder(AllowedMentionsBuilder::new()).build());

		self
	}

	pub fn message<T: AsRef<str>>(&mut self, content: T) -> &mut Self {
		assert!(!content.as_ref().is_empty(), "empty message not allowed");

		self.callback.content = Some(content.as_ref().to_owned());

		self
	}

	pub fn embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
		assert!(!embeds.is_empty(), "empty embeds not allowed");

		self.callback.embeds.extend(embeds);

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
		}
	}
}
