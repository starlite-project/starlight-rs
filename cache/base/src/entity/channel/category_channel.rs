use crate::{
    repository::{GetEntityFuture, Repository},
    utils, Backend, Entity,
};
use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, CategoryChannel, ChannelType},
    id::{ChannelId, GuildId},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryChannelEntity {
    pub guild_id: Option<GuildId>,
    pub id: ChannelId,
    pub kind: ChannelType,
    pub name: String,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
}

impl From<CategoryChannel> for CategoryChannelEntity {
    fn from(channel: CategoryChannel) -> Self {
        Self {
            guild_id: channel.guild_id,
            id: channel.id,
            kind: channel.kind,
            name: channel.name,
            permission_overwrites: channel.permission_overwrites,
            position: channel.position,
        }
    }
}

impl Entity for CategoryChannelEntity {
    type Id = ChannelId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

pub trait CategoryChannelRepository<B: Backend>: Repository<CategoryChannelEntity, B> {}
