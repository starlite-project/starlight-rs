use super::message::MessageEntity;
use crate::{
    entity::user::UserEntity, repository::GetEntityFuture, utils, Backend, Entity, Repository,
};
use twilight_model::{
    channel::{ChannelType, PrivateChannel},
    id::{ChannelId, MessageId, UserId},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateChannelEntity {
    pub id: ChannelId,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<String>,
    pub kind: ChannelType,
    pub recipient_id: Option<UserId>,
}

impl From<PrivateChannel> for PrivateChannelEntity {
    fn from(channel: PrivateChannel) -> Self {
        let recipient_id = channel.recipients.first().map(|user| user.id);

        Self {
            id: channel.id,
            last_message_id: channel.last_message_id,
            last_pin_timestamp: channel.last_pin_timestamp,
            kind: channel.kind,
            recipient_id,
        }
    }
}

impl Entity for PrivateChannelEntity {
    type Id = ChannelId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait PrivateChannelRepository<B: Backend>: Repository<PrivateChannelEntity, B> {
    fn last_message(&self, channel_id: ChannelId) -> GetEntityFuture<'_, MessageEntity, B::Error> {
        utils::relation_and_then(
            self.backend().private_channels(),
            self.backend().messages(),
            channel_id,
            |channel| channel.last_message_id,
        )
    }

    fn recipient(&self, channel_id: ChannelId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        utils::relation_and_then(
            self.backend().private_channels(),
            self.backend().users(),
            channel_id,
            |channel| channel.recipient_id,
        )
    }
}
