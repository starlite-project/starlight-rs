mod futures;

pub use futures::{
	WaitForEventFuture, WaitForEventStream, WaitForGuildEventFuture, WaitForGuildEventStream,
	WaitForInteractionFuture, WaitForInteractionStream, WaitForMessageFuture, WaitForMessageStream,
	WaitForReactionFuture, WaitForReactionStream,
};

use dashmap::DashMap;
use std::{
	fmt::{Debug, Formatter, Result as FmtResult},
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc,
	},
};
use tokio::sync::{
	mpsc::{self, UnboundedSender as MpscSender},
	oneshot::{self, Sender as OneshotSender},
};
use twilight_model::{
	channel::Channel,
	gateway::{
		event::Event,
		payload::{MessageCreate, ReactionAdd},
	},
	id::{ChannelId, GuildId, MessageId},
};

enum Sender<E> {
	Mpsc(MpscSender<E>),
	Oneshot(OneshotSender<E>),
}

impl<E> Sender<E> {
	fn is_closed(&self) -> bool {
		match self {
			Self::Mpsc(sender) => sender.is_closed(),
			Self::Oneshot(sender) => sender.is_closed(),
		}
	}
}

struct Bystander<E> {
	func: Box<dyn Fn(&E) -> bool + Send + Sync>,
	sender: Option<Sender<E>>,
}

impl<E> Debug for Bystander<E> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Bystander")
			.field("check", &"check func")
			.field("sender", &"mpsc sender")
			.finish()
	}
}

#[derive(Debug, Default)]
struct StandbyInner {
    events: DashMap<u64, Bystander<Event>>,
    event_counter: AtomicU64,
    guilds: DashMap<GuildId, Vec<Bystander<Event>>>,
    messages: DashMap<ChannelId, Vec<Bystander<MessageCreate>>>,
    reactions: DashMap<MessageId, Vec<Bystander<ReactionAdd>>>,
}

#[derive(Debug, Default, Clone)]
pub struct Standby(Arc<StandbyInner>);

impl Standby {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn bystander_process<E: Clone + Debug>(&self, bystander: &mut Bystander<E>, event: &E) -> bool {
        let sender = match bystander.sender.take() {
            Some(sender) => sender,
            None => {
                #[cfg(feature = "tracing")]
                tracing::trace!("bystander has no sender, indicating for removal");

                return true
            }
        };


        if sender.is_closed() {
            #[cfg(feature = "tracing")]
            tracing::trace!("bystander's rx dropped, indicating for removal")
        }
    }
}

const fn event_guild_id(event: &Event) -> Option<GuildId> {
    match event {
        Event::BanAdd(e) => Some(e.guild_id),
        Event::BanRemove(e) => Some(e.guild_id),
        Event::ChannelCreate(e) => channel_guild_id(&e.0),
        Event::ChannelDelete(e) => channel_guild_id(&e.0),
        Event::ChannelPinsUpdate(_) => None,
        Event::ChannelUpdate(e) => channel_guild_id(&e.0),
        Event::GatewayHeartbeatAck => None,
        Event::GatewayHeartbeat(_) => None,
        Event::GatewayHello(_) => None,
        Event::GatewayInvalidateSession(_) => None,
        Event::GatewayReconnect => None,
        Event::GiftCodeUpdate => None,
        Event::GuildCreate(e) => Some(e.0.id),
        Event::GuildDelete(e) => Some(e.id),
        Event::GuildEmojisUpdate(e) => Some(e.guild_id),
        Event::GuildIntegrationsUpdate(e) => Some(e.guild_id),
        Event::GuildUpdate(e) => Some(e.0.id),
        Event::IntegrationCreate(e) => e.0.guild_id,
        Event::IntegrationDelete(e) => Some(e.guild_id),
        Event::IntegrationUpdate(e) => e.0.guild_id,
        Event::InteractionCreate(e) => e.0.guild_id(),
        Event::InviteCreate(e) => Some(e.guild_id),
        Event::InviteDelete(e) => Some(e.guild_id),
        Event::MemberAdd(e) => Some(e.0.guild_id),
        Event::MemberChunk(e) => Some(e.guild_id),
        Event::MemberRemove(e) => Some(e.guild_id),
        Event::MemberUpdate(e) => Some(e.guild_id),
        Event::MessageCreate(e) => e.0.guild_id,
        Event::MessageDelete(_) => None,
        Event::MessageDeleteBulk(_) => None,
        Event::MessageUpdate(_) => None,
        Event::PresenceUpdate(e) => Some(e.guild_id),
        Event::PresencesReplace => None,
        Event::ReactionAdd(e) => e.0.guild_id,
        Event::ReactionRemove(e) => e.0.guild_id,
        Event::ReactionRemoveAll(e) => e.guild_id,
        Event::ReactionRemoveEmoji(e) => Some(e.guild_id),
        Event::Ready(_) => None,
        Event::Resumed => None,
        Event::RoleCreate(e) => Some(e.guild_id),
        Event::RoleDelete(e) => Some(e.guild_id),
        Event::RoleUpdate(e) => Some(e.guild_id),
        Event::ShardConnected(_) => None,
        Event::ShardConnecting(_) => None,
        Event::ShardDisconnected(_) => None,
        Event::ShardIdentifying(_) => None,
        Event::ShardPayload(_) => None,
        Event::ShardReconnecting(_) => None,
        Event::ShardResuming(_) => None,
        Event::StageInstanceCreate(e) => Some(e.0.guild_id),
        Event::StageInstanceDelete(e) => Some(e.0.guild_id),
        Event::StageInstanceUpdate(e) => Some(e.0.guild_id),
        Event::ThreadCreate(e) => channel_guild_id(&e.0),
        Event::ThreadDelete(e) => channel_guild_id(&e.0),
        Event::ThreadListSync(e) => Some(e.guild_id),
        Event::ThreadMemberUpdate(_) => None,
        Event::ThreadMembersUpdate(e) => Some(e.guild_id),
        Event::ThreadUpdate(e) => channel_guild_id(&e.0),
        Event::TypingStart(e) => e.guild_id,
        Event::UnavailableGuild(e) => Some(e.id),
        Event::UserUpdate(_) => None,
        Event::VoiceServerUpdate(e) => e.guild_id,
        Event::VoiceStateUpdate(e) => e.0.guild_id,
        Event::WebhooksUpdate(e) => Some(e.guild_id),
    }
}

const fn channel_guild_id(channel: &Channel) -> Option<GuildId> {
    match channel {
        Channel::Guild(c) => c.guild_id(),
        _ => None,
    }
}