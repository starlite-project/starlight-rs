use super::state::State;
use tracing::{event, instrument, Level};
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        interaction::application_command::{ApplicationCommand, CommandData},
    },
    guild::PartialMember,
    id::{ChannelId, GuildId, InteractionId},
    user::User,
};

pub mod commands;

#[derive(Debug, Clone)]
pub struct Interaction<'a> {
    state: &'a State,
    id: InteractionId,
    token: String,
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

    pub const fn ack() -> InteractionResponse {
        InteractionResponse::DeferredChannelMessageWithSource(Self::BASE)
    }

    pub fn message(msg: impl Into<String>) -> InteractionResponse {
        Self::_message(msg.into())
    }

    fn _message(msg: String) -> InteractionResponse {
        if msg.is_empty() {
            panic!("empty message is not allowed")
        }

        let mut data = Self::BASE;

        data.content = Some(msg);

        InteractionResponse::ChannelMessageWithSource(data)
    }
}

#[instrument(skip(state, command), fields(command.name = %command.data.name, command.guild_id))]
pub async fn act(state: &State, command: ApplicationCommand) {
    let interaction = Interaction {
        state,
        id: command.id,
        token: command.token.clone(),
    };

    let partial_command = PartialApplicationCommand::from(command);

    todo!()
}
