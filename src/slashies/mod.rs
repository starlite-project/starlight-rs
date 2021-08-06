#![allow(dead_code)]

use self::commands::Commands;
use super::state::State;
use tracing::{event, instrument, Level};
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        interaction::application_command::{ApplicationCommand, CommandData},
    },
    channel::{embed::Embed, message::MessageFlags},
    guild::PartialMember,
    id::{ChannelId, GuildId, InteractionId},
    user::User,
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

        self.0.flags = self.0.flags.map_or(Some(flags), |current_flags| Some(flags | current_flags));

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

#[instrument(skip(state, command), fields(command.name = %command.data.name, command.guild_id))]
pub async fn act(state: State, command: ApplicationCommand) {
    let interaction = Interaction {
        state,
        id: command.id,
        token: command.token.clone(),
    };


    if let Some(cmd) = Commands::r#match(partial_command) {
        if cmd.is_long() {
            interaction.ack().await;
        }

        let response = cmd
            .run(state)
            .await
            .unwrap_or_else(|_| Response::new().message("error executing command").exec());
        interaction.response(response).await;
    } else {
        event!(Level::WARN, "received unregistered command");
    }
}
