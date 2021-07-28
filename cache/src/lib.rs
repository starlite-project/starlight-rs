pub mod model;

mod config;

pub use self::config::{Config, ResourceType};

use self::model::*;
use dashmap::{
    mapref::{entry::Entry, one::Ref},
    DashMap, DashSet,
};
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    hash::Hash,
    ops::Deref,
    sync::{Arc, Mutex},
};
use twilight_model::{
    channel::{Group, GuildChannel, PrivateChannel, StageInstance},
    gateway::event::Event,
    guild::{GuildIntegration, Role},
    id::{ChannelId, EmojiId, GuildId, IntegrationId, MessageId, RoleId, StageId, UserId},
    user::{CurrentUser, User},
    voice::VoiceState,
};

#[derive(Debug)]
struct GuildItem<T> {
    data: T,
    guild_id: GuildId,
}

fn upsert_guild_item<K: Eq + Hash, V: PartialEq>(
    map: &DashMap<K, GuildItem<V>>,
    guild_id: GuildId,
    key: K,
    value: V,
) {
    match map.entry(key) {
        Entry::Occupied(entry) if entry.get().data == value => {}
        Entry::Occupied(mut entry) => {
            entry.insert(GuildItem {
                data: value,
                guild_id,
            });
        }
        Entry::Vacant(entry) => {
            entry.insert(GuildItem {
                data: value,
                guild_id,
            });
        }
    }
}

fn upsert_item<K: Eq + Hash, V: PartialEq>(map: &DashMap<K, V>, k: K, v: V) {
    map.insert(k, v);
}

type TupleMap<K, V> = DashMap<(GuildId, K), V>;

type GuildSet<V> = DashMap<GuildId, HashSet<V>>;

#[derive(Debug, Default)]
struct CacheRef {
    config: Config,
    channels_guild: DashMap<ChannelId, GuildItem<GuildChannel>>,
    channels_private: DashMap<ChannelId, PrivateChannel>,
    current_user: Mutex<Option<CurrentUser>>,
    emojis: DashMap<EmojiId, GuildItem<CachedEmoji>>,
    groups: DashMap<ChannelId, Group>,
    guilds: DashMap<GuildId, CachedGuild>,
    guild_channels: GuildSet<ChannelId>,
    guild_emojis: GuildSet<EmojiId>,
    guild_integrations: GuildSet<IntegrationId>,
    guild_members: GuildSet<UserId>,
    guild_presences: GuildSet<UserId>,
    guild_roles: GuildSet<RoleId>,
    guild_stage_instances: GuildSet<StageId>,
    integrations: TupleMap<IntegrationId, GuildItem<GuildIntegration>>,
    members: TupleMap<UserId, CachedMember>,
    messages: DashMap<ChannelId, VecDeque<CachedMessage>>,
    presences: TupleMap<UserId, CachedPresence>,
    roles: DashMap<RoleId, GuildItem<Role>>,
    stage_instances: DashMap<StageId, GuildItem<StageInstance>>,
    unavailable_guilds: DashSet<GuildId>,
    users: DashMap<UserId, (User, BTreeSet<GuildId>)>,
    voice_state_channels: GuildSet<(GuildId, UserId)>,
    voice_state_guilds: GuildSet<UserId>,
    voice_states: TupleMap<UserId, VoiceState>,
}

#[derive(Debug, Default, Clone)]
pub struct Cache(Arc<CacheRef>);

impl Cache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(&self) -> Config {
        self.0.config
    }

    fn wants(&self, resource_type: ResourceType) -> bool {
        self.0.config.resource_types().contains(resource_type)
    }
}
