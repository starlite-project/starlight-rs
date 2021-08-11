#![allow(dead_code)]

use self::commands::Commands;
use super::state::State;
use tracing::{event, instrument, Level};
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        interaction::application_command::ApplicationCommand,
    },
    channel::{embed::Embed, message::MessageFlags},
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

    pub fn message<T: AsRef<str>>(&mut self, content: T) -> Self {
        if content.as_ref().is_empty() {
            panic!("empty message not allowed");
        }

        self.0.content = Some(content.as_ref().to_owned());

        self.clone()
    }

    pub fn embeds(&mut self, embeds: Vec<Embed>) -> Self {
        if embeds.is_empty() {
            panic!("empty embeds not allowed");
        }

        self.0.embeds.extend(embeds);

        self.clone()
    }

    pub fn embed(&mut self, embed: Embed) -> Self {
        self.embeds(vec![embed])
    }

    pub fn flags(&mut self, flags: MessageFlags) -> Self {
        self.0.flags = self
            .0
            .flags
            .map_or(Some(flags), |current_flags| Some(flags | current_flags));

        self.clone()
    }

    pub fn ephemeral(&mut self) -> Self {
        self.flags(MessageFlags::EPHEMERAL)
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn exec(self) -> InteractionResponse {
        InteractionResponse::ChannelMessageWithSource(self.0)
    }
}

impl From<&str> for Response {
    fn from(message: &str) -> Self {
        Self::new().message(message)
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
