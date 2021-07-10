use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType, VoiceChannel},
    id::{ChannelId, GuildId},
};

use crate::{
    entity::guild::GuildEntity, repository::GetEntityFuture, utils, Backend, Entity, Repository,
};

use super::category_channel::CategoryChannelEntity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceChannelEntity {
    pub bitrate: u64,
    pub guild_id: Option<GuildId>,
    pub id: ChannelId,
    pub kind: ChannelType,
    pub name: String,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub parent_id: Option<ChannelId>,
    pub position: i64,
    pub user_limit: Option<u64>,
}

impl From<VoiceChannel> for VoiceChannelEntity {
    fn from(channel: VoiceChannel) -> Self {
        Self {
            bitrate: channel.bitrate,
            guild_id: channel.guild_id,
            id: channel.id,
            kind: channel.kind,
            name: channel.name,
            permission_overwrites: channel.permission_overwrites,
            parent_id: channel.parent_id,
            position: channel.position,
            user_limit: channel.user_limit,
        }
    }
}

impl Entity for VoiceChannelEntity {
    type Id = ChannelId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait VoiceChannelRepository<B: Backend>: Repository<VoiceChannelEntity, B> {
    fn guild(&self, channel_id: ChannelId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        utils::relation_and_then(
            self.backend().voice_channels(),
            self.backend().guilds(),
            channel_id,
            |channel| channel.guild_id,
        )
    }

    fn parent(
        &self,
        channel_id: ChannelId,
    ) -> GetEntityFuture<'_, CategoryChannelEntity, B::Error> {
        utils::relation_and_then(
            self.backend().voice_channels(),
            self.backend().category_channels(),
            channel_id,
            |channel| channel.parent_id,
        )
    }
}
