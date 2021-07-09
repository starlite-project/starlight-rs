use crate::{
    repository::{GetEntityFuture, ListEntitiesFuture, Repository},
    utils, Backend, Entity,
};
use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{
        embed::Embed,
        message::{MessageActivity, MessageFlags, MessageReaction, MessageType},
        Message,
    },
    gateway::payload::MessageUpdate,
    id::{ApplicationId, AttachmentId, ChannelId, GuildId, MessageId, RoleId, UserId, WebhookId},
};

use super::attachment::AttachmentEntity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageEntity {
    pub activity: Option<MessageActivity>,
    pub application_id: Option<ApplicationId>,
    pub attachments: Vec<AttachmentId>,
    pub author_id: UserId,
    pub channel_id: ChannelId,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<GuildId>,
    pub id: MessageId,
    pub kind: MessageType,
    pub mention_channels: Vec<ChannelId>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<RoleId>,
    pub mentions: Vec<UserId>,
    pub pinned: bool,
    pub reactions: Vec<MessageReaction>,
    pub timestamp: String,
    pub tts: bool,
    pub webhook_id: Option<WebhookId>,
}

impl From<Message> for MessageEntity {
    fn from(message: Message) -> Self {
        let attachments = message
            .attachments
            .into_iter()
            .map(|attachment| attachment.id)
            .collect();

        let mention_channels = message
            .mention_channels
            .into_iter()
            .map(|channel| channel.id)
            .collect();

        let mentions = message
            .mentions
            .into_iter()
            .map(|mention| mention.id)
            .collect();

        Self {
            activity: message.activity,
            application_id: message.application_id,
            attachments,
            author_id: message.author.id,
            channel_id: message.channel_id,
            content: message.content,
            edited_timestamp: message.edited_timestamp,
            embeds: message.embeds,
            flags: message.flags,
            guild_id: message.guild_id,
            id: message.id,
            kind: message.kind,
            mention_channels,
            mention_everyone: message.mention_everyone,
            mention_roles: message.mention_roles,
            mentions,
            pinned: message.pinned,
            reactions: message.reactions,
            timestamp: message.timestamp,
            tts: message.tts,
            webhook_id: message.webhook_id,
        }
    }
}

impl MessageEntity {
    pub fn update(self, update: MessageUpdate) -> Self {
        let attachments = update
            .attachments
            .map_or(self.attachments, |a| a.into_iter().map(|m| m.id).collect());

        let mentions = update
            .mentions
            .map_or(self.mentions, |m| m.into_iter().map(|m| m.id).collect());

        Self {
            attachments,
            author_id: update.author.map_or(self.author_id, |a| a.id),
            channel_id: update.channel_id,
            content: update.content.map_or(self.content, |m| m),
            edited_timestamp: update.edited_timestamp.or(self.edited_timestamp),
            embeds: update.embeds.map_or(self.embeds, |e| e),
            guild_id: update.guild_id.or(self.guild_id),
            id: update.id,
            kind: update.kind.map_or(self.kind, |k| k),
            mention_everyone: update.mention_everyone.map_or(self.mention_everyone, |m| m),
            mention_roles: update.mention_roles.map_or(self.mention_roles, |m| m),
            mentions,
            pinned: update.pinned.map_or(self.pinned, |p| p),
            timestamp: update.timestamp.map_or(self.timestamp, |t| t),
            tts: update.tts.map_or(self.tts, |t| t),
            ..self
        }
    }
}

impl Entity for MessageEntity {
    type Id = MessageId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait MessageRepository<B: Backend>: Repository<MessageEntity, B> + Send {
    fn attachments(
        &self,
        message_id: MessageId,
    ) -> ListEntitiesFuture<'_, AttachmentEntity, B::Error> {
        todo!()
    }
}
