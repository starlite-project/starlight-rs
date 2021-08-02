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

    pub const fn new() -> Self {
        Self(Self::BASE)
    }

    pub const fn ack() -> InteractionResponse {
        InteractionResponse::DeferredChannelMessageWithSource(Self::BASE)
    }

    pub fn message(&mut self, content: &str) -> Self {
        if content.is_empty() {
            panic!("empty message not allowed");
        }

        self.0.content = Some(content.to_owned());

        self.to_owned()
    }

    pub fn embeds(&mut self, embeds: Vec<Embed>) -> Self {
        if embeds.is_empty() {
            panic!("empty embeds not allowed");
        }

        self.0.embeds.extend(embeds);

        self.to_owned()
    }

    pub fn embed(&mut self, embed: Embed) -> Self {
        self.embeds(vec![embed])
    }

    pub fn flags(&mut self, flags: MessageFlags) ->Self {
        match self.0.flags {
            None => {
                self.0.flags = Some(flags);
            }
            Some(current_flags) => {
                self.0.flags = Some(flags | current_flags);
            }
        }

        self.to_owned()
    }

    pub fn ephemeral(&mut self) -> Self {
        self.flags(MessageFlags::EPHEMERAL)
    }

    pub fn exec(self) -> InteractionResponse {
        InteractionResponse::ChannelMessageWithSource(self.0)
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
            .unwrap_or_else(|_| Response::new().message("error executing command").exec());
        interaction.response(response).await;
    } else {
        event!(Level::WARN, "received unregistered command");
    }
}
