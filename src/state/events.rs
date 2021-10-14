use super::State;
use miette::Result;
use twilight_gateway::Event;

#[allow(clippy::enum_glob_use, clippy::match_wildcard_for_single_variants)]
pub async fn handle(event: Event, state: State) -> Result<()> {
	use Event::*;
	match event {
		BanAdd(ban) => internal::ban_add(state, ban).await?,
		BanRemove(ban) => internal::ban_remove(state, ban).await?,
		ChannelCreate(channel) => internal::channel_create(state, channel).await?,
		ChannelDelete(channel) => internal::channel_delete(state, channel).await?,
		ChannelPinsUpdate(pins) => internal::channel_pins_update(state, pins).await?,
		ChannelUpdate(update) => internal::channel_update(state, update).await?,
		GatewayHeartbeat(val) => internal::gateway_heartbeat(state, val).await?,
		GatewayHeartbeatAck => internal::gateway_heartbeat_ack(state).await?,
		GatewayHello(val) => internal::gateway_hello(state, val).await?,
		GatewayInvalidateSession(val) => internal::gateway_invalidate_session(state, val).await?,
		GatewayReconnect => internal::gateway_reconnect(state).await?,
		GuildCreate(guild) => internal::guild_create(state, *guild).await?,
		GuildDelete(guild) => internal::guild_delete(state, *guild).await?,
		GuildEmojisUpdate(guild) => internal::guild_emojis_update(state, guild).await?,
		GuildIntegrationsUpdate(guild) => internal::guild_integrations_update(state, guild).await?,
		GuildUpdate(guild) => internal::guild_update(state, *guild).await?,
		InteractionCreate(interaction) => internal::interaction_create(state, *interaction).await?,
		InviteCreate(invite) => internal::invite_create(state, *invite).await?,
		InviteDelete(invite) => internal::invite_delete(state, invite).await?,
		MemberAdd(member) => internal::member_add(state, *member).await?,
		MemberRemove(member) => internal::member_remove(state, member).await?,
		MemberUpdate(member) => internal::member_update(state, *member).await?,
		MemberChunk(chunk) => internal::member_chunk(state, chunk).await?,
		MessageCreate(message) => internal::message_create(state, *message).await?,
		MessageDelete(message) => internal::message_delete(state, message).await?,
		MessageDeleteBulk(message) => internal::message_delete_bulk(state, message).await?,
		MessageUpdate(message) => internal::message_update(state, *message).await?,
		PresenceUpdate(presence) => internal::presence_update(state, *presence).await?,
		PresencesReplace => internal::presences_replace(state).await?,
		ReactionAdd(reaction) => internal::reaction_add(state, *reaction).await?,
		ReactionRemove(reaction) => internal::reaction_remove(state, *reaction).await?,
		ReactionRemoveAll(reactions) => internal::reaction_remove_all(state, reactions).await?,
		ReactionRemoveEmoji(reactions) => internal::reaction_remove_emoji(state, reactions).await?,
		Ready(ready) => internal::ready(state, *ready).await?,
		Resumed => internal::resumed(state).await?,
		RoleCreate(role) => internal::role_create(state, role).await?,
		RoleDelete(role) => internal::role_delete(state, role).await?,
		RoleUpdate(role) => internal::role_update(state, role).await?,
		ShardConnected(connected) => internal::shard_connected(state, connected).await?,
		ShardConnecting(connecting) => internal::shard_connecting(state, connecting).await?,
		ShardDisconnected(disconnected) => {
			internal::shard_disconnected(state, disconnected).await?;
		}
		ShardIdentifying(identifying) => internal::shard_identifying(state, identifying).await?,
		ShardReconnecting(reconnecting) => {
			internal::shard_reconnecting(state, reconnecting).await?;
		}
		ShardPayload(payload) => internal::shard_payload(state, payload).await?,
		ShardResuming(resume) => internal::shard_resuming(state, resume).await?,
		StageInstanceCreate(stage_instance) => {
			internal::stage_instance_create(state, stage_instance).await?;
		}
		StageInstanceDelete(stage_instance) => {
			internal::stage_instance_delete(state, stage_instance).await?;
		}
		StageInstanceUpdate(stage_instance) => {
			internal::stage_instance_update(state, stage_instance).await?;
		}
		TypingStart(typing) => internal::typing_start(state, *typing).await?,
		UnavailableGuild(guild) => internal::unavailable_guild(state, guild).await?,
		UserUpdate(user) => internal::user_update(state, user).await?,
		VoiceServerUpdate(voice) => internal::voice_server_update(state, voice).await?,
		VoiceStateUpdate(voice) => internal::voice_state_update(state, *voice).await?,
		WebhooksUpdate(webhooks) => internal::webhooks_update(state, webhooks).await?,
		_ => {}
	}
	Ok(())
}

mod internal {
	#![allow(
		unused_variables,
		dead_code,
		clippy::wildcard_imports,
		clippy::unused_async
	)]
	use crate::{
		slashies,
		state::State,
	};
	use miette::{ Result};
	use tracing::{event, Level};
	use twilight_model::{
		application::interaction::Interaction,
		gateway::{event::shard::*, payload::*},
	};

	macro_rules! gen_empty_handlers {
        ($($fn_names:ident: [$($fn_args:ty),*];)*) => {
            $(
                pub(super) async fn $fn_names(state: State, $(_: $fn_args),*) -> Result<()> {
                    Ok(())
                }
            )*
        }
    }

	gen_empty_handlers! {
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
		// interaction_create: [InteractionCreate];
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
		// ready: [Ready];
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

	pub(super) async fn ready(state: State, ready: Ready) -> Result<()> {
		event!(Level::INFO, user_name = %ready.user.name);
		event!(Level::INFO, guilds = %ready.guilds.len());
		Ok(())
	}

	pub(super) async fn interaction_create(
		state: State,
		interaction: InteractionCreate,
	) -> Result<()> {
		match interaction.0 {
			Interaction::ApplicationCommand(cmd) => slashies::act(state, *cmd).await,
			// Ignore components, as it's handled by Standby
			Interaction::MessageComponent(_) => {}
			i => event!(Level::WARN, ?i, "unhandled interaction"),
		}
		Ok(())
	}
}
