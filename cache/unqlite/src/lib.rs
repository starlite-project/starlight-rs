use std::sync::Arc;

use serde::Serialize;
use star_cache_base::{
    entity::{
        channel::{
            AttachmentEntity, CategoryChannelEntity, GroupEntity, MessageEntity,
            PrivateChannelEntity, TextChannelEntity, VoiceChannelEntity,
        },
        gateway::PresenceEntity,
        guild::{EmojiEntity, GuildEntity, MemberEntity, RoleEntity},
        user::{CurrentUserEntity, UserEntity},
        voice::VoiceStateEntity,
    },
    Entity,
};
use unqlite::UnQLite;

pub trait UnqliteEntity: Entity + Serialize {
    fn key(id: Self::Id) -> Vec<u8>;

    fn value(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

pub trait UnqliteSingleEntity: Entity + Serialize {
    fn key() -> &'static [u8];

    fn value(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

impl UnqliteEntity for AttachmentEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("at:{}", id).into_bytes()
    }
}

impl UnqliteEntity for CategoryChannelEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("cc:{}", id).into_bytes()
    }
}

impl UnqliteSingleEntity for CurrentUserEntity {
    fn key() -> &'static [u8] {
        &[b'u', b'c']
    }
}

impl UnqliteEntity for EmojiEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("em:{}", id).into_bytes()
    }
}

impl UnqliteEntity for GroupEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("gr:{}", id).into_bytes()
    }
}

impl UnqliteEntity for GuildEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("g:{}", id).into_bytes()
    }
}

impl UnqliteEntity for MemberEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("m:{}:{}", id.0, id.1).into_bytes()
    }
}

impl UnqliteEntity for MessageEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("ms:{}", id).into_bytes()
    }
}

impl UnqliteEntity for PresenceEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("pr:{}:{}", id.0, id.1).into_bytes()
    }
}

impl UnqliteEntity for PrivateChannelEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("cp:{}", id).into_bytes()
    }
}

impl UnqliteEntity for RoleEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("r:{}", id).into_bytes()
    }
}

impl UnqliteEntity for TextChannelEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("ct:{}", id).into_bytes()
    }
}

impl UnqliteEntity for UserEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("u:{}", id).into_bytes()
    }
}

impl UnqliteEntity for VoiceChannelEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("cv:{}", id).into_bytes()
    }
}

impl UnqliteEntity for VoiceStateEntity {
    fn key(id: Self::Id) -> Vec<u8> {
        format!("v:{}:{}", id.0, id.1).into_bytes()
    }
}

#[derive(Clone)]
pub struct UnqliteBackend(Arc<UnQLite>);

impl UnqliteBackend {
    pub fn new(unqlite: UnQLite) -> Self {
        Self(Arc::new(unqlite))
    }

    pub fn create<F: AsRef<str>>(filename: F) -> Self {
        Self::new(UnQLite::create(filename))
    }

    pub fn create_in_memory() -> Self {
        Self::new(UnQLite::create_in_memory())
    }

    pub fn create_temp() -> Self {
        Self::new(UnQLite::create_temp())
    }

    pub fn open_mmap<F: AsRef<str>>(filename: F) -> Self {
        Self::new(UnQLite::open_mmap(filename))
    }

    pub fn open_readonly<F: AsRef<str>>(filename: F) -> Self {
        Self::new(UnQLite::open_readonly(filename))
    }
}
