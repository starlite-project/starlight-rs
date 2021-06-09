#![allow(clippy::wildcard_imports)]
use super::State;
use async_trait::async_trait;
use twilight_model::gateway::{event::shard::*, payload::*};

macro_rules! impl_handler_trait {
    ($($fn:ident: [$($fn_args:ty),*];)*) => {
            #[async_trait]
            #[allow(unused_variables)]
            pub trait EventHandler: Send + Sync {
                $(
                    async fn $fn(&self, state: &State, $(_: $fn_args),*) -> super::super::GenericResult<()> {
                        Ok(())
                    }
                )*
            }
    }
}

impl_handler_trait! {
    ban_add: [BanAdd];
    ban_remove: [BanRemove];
    channel_create: [ChannelCreate];
    channel_delete: [ChannelDelete];
    channel_pins_update: [ChannelPinsUpdate];
    channel_update: [ChannelUpdate];
    gateway_heartbeat: [u64];
    gateway_heartbeat_ack: [];
    gateway_hello: [u64];
    gateway_invalidate_session: [bool];
    gateway_reconnect: [];
    gift_code_update: [];
    guild_create: [GuildCreate];
    guild_delete: [GuildDelete];
    guild_emojis_update: [GuildEmojisUpdate];
    guild_integrations_update: [GuildIntegrationsUpdate];
    guild_update: [GuildUpdate];
    invite_create: [InviteCreate];
    invite_delete: [InviteDelete];
    member_add: [MemberAdd];
    member_remove: [MemberRemove];
    member_update: [MemberUpdate];
    member_chunk: [MemberChunk];
    message_create: [MessageCreate];
    message_delete: [MessageDelete];
    message_delete_bulk: [MessageDeleteBulk];
    message_update: [MessageUpdate];
    presence_update: [PresenceUpdate];
    presences_replace: [];
    reaction_add: [ReactionAdd];
    reaction_remove: [ReactionRemove];
    reaction_remove_all: [ReactionRemoveAll];
    reaction_remove_emoji: [ReactionRemoveEmoji];
    ready: [Ready];
    resumed: [];
    role_create: [RoleCreate];
    role_delete: [RoleDelete];
    role_update: [RoleUpdate];
    shard_connected: [Connected];
    shard_connecting: [Connecting];
    shard_disconnected: [Disconnected];
    shard_identifying: [Identifying];
    shard_reconnecting: [Reconnecting];
    shard_payload: [Payload];
    shard_resuming: [Resuming];
    stage_instance_create: [StageInstanceCreate];
    stage_instance_delete: [StageInstanceDelete];
    stage_instance_update: [StageInstanceUpdate];
    typing_start: [TypingStart];
    unavailable_guild: [UnavailableGuild];
    user_update: [UserUpdate];
    voice_server_update: [VoiceServerUpdate];
    voice_state_update: [VoiceStateUpdate];
    webhooks_update: [WebhooksUpdate];
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultHandler;

impl EventHandler for DefaultHandler {}

impl Default for Box<dyn EventHandler + 'static> {
    fn default() -> Self {
        Box::new(DefaultHandler)
    }
}
