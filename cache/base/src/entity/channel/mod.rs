use serde::{Deserialize, Serialize};

pub mod attachment;
pub mod category_channel;
pub mod group;
pub mod message;
pub mod private_channel;
pub mod text_channel;
pub mod voice_channel;

pub use self::{
    attachment::{AttachmentEntity, AttachmentRepository},
    category_channel::{CategoryChannelEntity, CategoryChannelRepository},
    group::{GroupEntity, GroupRepository},
    message::{MessageEntity, MessageRepository},
    private_channel::{PrivateChannelEntity, PrivateChannelRepository},
    text_channel::{TextChannelEntity, TextChannelRepository},
    voice_channel::{VoiceChannelEntity, VoiceChannelRepository},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChannelEntity {
    Group(GroupEntity),
    Guild(GuildChannelEntity),
    Private(PrivateChannelEntity),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GuildChannelEntity {
    Category(CategoryChannelEntity),
    Text(TextChannelEntity),
    Voice(VoiceChannelEntity),
}
