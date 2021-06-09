#![allow(dead_code)]
use futures::StreamExt;
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    sync::Arc,
};
use twilight_cache_inmemory::InMemoryCache as Cache;
use twilight_gateway::{Cluster, Event};
use twilight_http::Client as HttpClient;
use twilight_standby::Standby;

mod builder;
mod error;
mod event_handler;

pub use self::builder::StateBuilder;
pub use self::event_handler::EventHandler;

#[derive(Clone)]
pub struct State {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: HttpClient,
    pub standby: Standby,
    pub event_handler: Arc<Box<dyn EventHandler + 'static>>,
}

impl State {
    pub async fn connect(self) -> super::GenericResult<()> {
        let cluster_spawn = self.cluster.clone();

        tokio::spawn(async move {
            cluster_spawn.up().await;
        });

        let mut events = self.cluster.events();

        while let Some(data) = events.next().await {
            self.cache.update(&data.1);
            self.standby.process(&data.1);

            let cloned_state = self.clone();

            tokio::spawn(async move {
                if let Err(source) = cloned_state.handle(data).await {
                    crate::error!("{:?}", source);
                }
            });
        }

        Ok(())
    }

    #[allow(clippy::enum_glob_use)]
    async fn handle(&self, data: (u64, Event)) -> super::GenericResult<()> {
        use twilight_gateway::Event::*;
        let (_, event) = data;

        match event {
            BanAdd(ban) => self.event_handler.ban_add(self, ban).await?,
            BanRemove(ban) => self.event_handler.ban_remove(self, ban).await?,
            ChannelCreate(channel) => self.event_handler.channel_create(self, channel).await?,
            ChannelDelete(channel) => self.event_handler.channel_delete(self, channel).await?,
            ChannelPinsUpdate(pins) => self.event_handler.channel_pins_update(self, pins).await?,
            ChannelUpdate(update) => self.event_handler.channel_update(self, update).await?,
            GatewayHeartbeat(val) => self.event_handler.gateway_heartbeat(self, val).await?,
            GatewayHeartbeatAck => self.event_handler.gateway_heartbeat_ack(self).await?,
            GatewayHello(val) => self.event_handler.gateway_hello(self, val).await?,
            GatewayInvalidateSession(val) => {
                self.event_handler
                    .gateway_invalidate_session(self, val)
                    .await?
            }
            GatewayReconnect => self.event_handler.gateway_reconnect(self).await?,
            GuildCreate(guild) => self.event_handler.guild_create(self, *guild).await?,
            GuildDelete(guild) => self.event_handler.guild_delete(self, *guild).await?,
            GuildEmojisUpdate(guild) => self.event_handler.guild_emojis_update(self, guild).await?,
            GuildIntegrationsUpdate(guild) => {
                self.event_handler
                    .guild_integrations_update(self, guild)
                    .await?
            }
            GuildUpdate(guild) => self.event_handler.guild_update(self, *guild).await?,
            InviteCreate(invite) => self.event_handler.invite_create(self, *invite).await?,
            InviteDelete(invite) => self.event_handler.invite_delete(self, invite).await?,
            MemberAdd(member) => self.event_handler.member_add(self, *member).await?,
            MemberRemove(member) => self.event_handler.member_remove(self, member).await?,
            MemberUpdate(member) => self.event_handler.member_update(self, *member).await?,
            MemberChunk(chunk) => self.event_handler.member_chunk(self, chunk).await?,
            MessageCreate(message) => self.event_handler.message_create(self, *message).await?,
            MessageDelete(message) => self.event_handler.message_delete(self, message).await?,
            MessageDeleteBulk(message) => {
                self.event_handler
                    .message_delete_bulk(self, message)
                    .await?
            }
            MessageUpdate(message) => self.event_handler.message_update(self, *message).await?,
            PresenceUpdate(presence) => self.event_handler.presence_update(self, *presence).await?,
            PresencesReplace => self.event_handler.presences_replace(self).await?,
            ReactionAdd(reaction) => self.event_handler.reaction_add(self, *reaction).await?,
            ReactionRemove(reaction) => self.event_handler.reaction_remove(self, *reaction).await?,
            ReactionRemoveAll(reactions) => {
                self.event_handler
                    .reaction_remove_all(self, reactions)
                    .await?
            }
            ReactionRemoveEmoji(reactions) => {
                self.event_handler
                    .reaction_remove_emoji(self, reactions)
                    .await?
            }
            Ready(ready) => self.event_handler.ready(self, *ready).await?,
            Resumed => self.event_handler.resumed(self).await?,
            RoleCreate(role) => self.event_handler.role_create(self, role).await?,
            RoleDelete(role) => self.event_handler.role_delete(self, role).await?,
            RoleUpdate(role) => self.event_handler.role_update(self, role).await?,
            ShardConnected(connected) => self.event_handler.shard_connected(self, connected).await?,
            ShardConnecting(connecting) => self.event_handler.shard_connecting(self, connecting).await?,
            ShardDisconnected(disconnected) => self.event_handler.shard_disconnected(self, disconnected).await?,
            ShardIdentifying(identifying) =>self.event_handler.shard_identifying(self, identifying).await?,
            ShardReconnecting(reconnecting) => self.event_handler.shard_reconnecting(self, reconnecting).await?,
            ShardPayload(payload) =>self.event_handler.shard_payload(self, payload).await?,
            ShardResuming(resume) => self.event_handler.shard_resuming(self, resume).await?,
            StageInstanceCreate(stage) => self.event_handler.stage_instance_create(self, stage).await?,
            StageInstanceDelete(stage) => self.event_handler.stage_instance_delete(self, stage).await?,
            StageInstanceUpdate(stage) => self.event_handler.stage_instance_update(self, stage).await?,
            TypingStart(typing) => self.event_handler.typing_start(self, *typing).await?,
            UnavailableGuild(guild) => self.event_handler.unavailable_guild(self, guild).await?,
            UserUpdate(user) => self.event_handler.user_update(self, user).await?,
            VoiceServerUpdate(voice) => self.event_handler.voice_server_update(self, voice).await?,
            VoiceStateUpdate(voice) => self.event_handler.voice_state_update(self, *voice).await?,
            WebhooksUpdate(webhooks) => self.event_handler.webhooks_update(self, webhooks).await?,
            _ => {}
        }

        Ok(())
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("State")
            .field("cache", &self.cache)
            .field("cluster", &self.cluster)
            .field("http", &self.http)
            .field("standby", &self.standby)
            .finish()
    }
}
