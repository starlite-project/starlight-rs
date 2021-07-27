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

struct CacheRef {}