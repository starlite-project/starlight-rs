use serde::{Deserialize, Serialize};
use twilight_model::{
    id::{ChannelId, GuildId, UserId},
    voice::VoiceState,
};

use crate::{
    entity::channel::VoiceChannelEntity, repository::GetEntityFuture, utils, Backend, Entity,
    Repository,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceStateEntity {
    pub channel_id: Option<ChannelId>,
    pub deaf: bool,
    pub guild_id: GuildId,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: bool,
    pub session_id: String,
    pub suppress: bool,
    pub token: Option<String>,
    pub user_id: UserId,
}

impl From<(VoiceState, GuildId)> for VoiceStateEntity {
    fn from((voice_state, guild_id): (VoiceState, GuildId)) -> Self {
        Self {
            channel_id: voice_state.channel_id,
            deaf: voice_state.deaf,
            guild_id,
            mute: voice_state.mute,
            self_deaf: voice_state.self_deaf,
            self_mute: voice_state.self_mute,
            self_stream: voice_state.self_stream,
            session_id: voice_state.session_id,
            suppress: voice_state.suppress,
            token: voice_state.token,
            user_id: voice_state.user_id,
        }
    }
}

impl Entity for VoiceStateEntity {
    type Id = (GuildId, UserId);

    fn id(&self) -> Self::Id {
        (self.guild_id, self.user_id)
    }
}

pub trait VoiceStateRepository<B: Backend>: Repository<VoiceStateEntity, B> {
    fn channel(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> GetEntityFuture<'_, VoiceChannelEntity, B::Error> {
        utils::relation_and_then(
            self.backend().voice_states(),
            self.backend().voice_channels(),
            (guild_id, user_id),
            |state| state.channel_id,
        )
    }
}
