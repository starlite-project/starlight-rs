#![allow(dead_code)]

use self::commands::Commands;
use super::state::State;
use tracing::{event, instrument, Level};
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        interaction::application_command::{ApplicationCommand, CommandData},
    },
    channel::embed::Embed,
    guild::PartialMember,
    id::{ChannelId, GuildId, InteractionId},
    user::User,
};

pub mod commands;

fn log_err<T, E: std::error::Error + 'static>(res: Result<T, E>) {
    if let Err(e) = res {
        event!(Level::ERROR, error = &e as &dyn std::error::Error);
    }
}

#[derive(Debug, Clone)]
pub struct Interaction {
    state: State,
    id: InteractionId,
    token: String,
}

impl Interaction {
    pub async fn ack(&self) {
        log_err(
            self.state
                .http
                .interaction_callback(self.id, self.token.as_str(), &Response::ack())
                .exec()
                .await,
        );
    }

    pub async fn response(&self, response: InteractionResponse) {
        log_err(
            self.state
                .http
                .interaction_callback(self.id, self.token.as_str(), &response)
                .exec()
                .await,
        );
    }
}

#[derive(Debug, Clone)]
pub struct PartialApplicationCommand {
    pub channel_id: ChannelId,
    pub data: CommandData,
    pub guild_id: Option<GuildId>,
    pub member: Option<PartialMember>,
    pub user: Option<User>,
}

impl From<ApplicationCommand> for PartialApplicationCommand {
    fn from(command: ApplicationCommand) -> Self {
        Self {
            channel_id: command.channel_id,
            data: command.data,
            guild_id: command.guild_id,
            member: command.member,
            user: command.user,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Response;

impl Response {
    pub const BASE: CallbackData = CallbackData {
        allowed_mentions: None,
        content: None,
        embeds: vec![],
        flags: None,
        tts: None,
    };

    #[must_use]
    pub const fn ack() -> InteractionResponse {
        InteractionResponse::DeferredChannelMessageWithSource(Self::BASE)
    }

    #[must_use]
    pub fn message(msg: impl Into<String>) -> InteractionResponse {
        Self::_message(msg.into())
    }

    #[must_use]
    pub fn embed(embed: Embed) -> InteractionResponse {
        Self::_embeds(vec![embed])
    }

    #[must_use]
    pub fn embeds(embeds: Vec<Embed>) -> InteractionResponse {
        Self::_embeds(embeds)
    }

    fn _embeds(embeds: Vec<Embed>) -> InteractionResponse {
        if embeds.is_empty() {
            panic!("empty embeds is not allowed");
        }

        let mut data = Self::BASE;

        data.embeds = embeds;

        InteractionResponse::ChannelMessageWithSource(data)
    }

    fn _message(msg: String) -> InteractionResponse {
        if msg.is_empty() {
            panic!("empty message is not allowed");
        }

        let mut data = Self::BASE;

        data.content = Some(msg);

        InteractionResponse::ChannelMessageWithSource(data)
    }
}

#[instrument(skip(state, command), fields(command.name = %command.data.name, command.guild_id))]
pub async fn act(state: State, command: ApplicationCommand) {
    let interaction = Interaction {
        state,
        id: command.id,
        token: command.token.clone(),
    };

    let partial_command = PartialApplicationCommand::from(command);

    if let Some(cmd) = Commands::r#match(partial_command) {
        if cmd.is_long() {
            interaction.ack().await;
        }

        let response = cmd
            .run(state)
            .await
            .unwrap_or_else(|_| Response::message("Error running command"));
        interaction.response(response).await;
    } else {
        event!(Level::WARN, "received unregistered command");
    }
}
