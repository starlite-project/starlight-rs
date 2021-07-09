use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{ChannelType, PrivateChannel},
    id::{ChannelId, MessageId, UserId},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
