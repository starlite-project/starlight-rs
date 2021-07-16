pub extern crate star_cache_base as cache;

pub mod config;
pub mod prelude;
pub mod repository;

use self::config::{Config, EntityType};
use cache::{
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
    Backend,
};
use dashmap::DashMap;
use repository::{
    InMemoryAttachmentRepository, InMemoryCategoryChannelRepository, InMemoryCurrentUserRepository,
    InMemoryEmojiRepository, InMemoryGroupRepository, InMemoryGuildRepository,
    InMemoryMemberRepository, InMemoryMessageRepository, InMemoryPresenceRepository,
    InMemoryPrivateChannelRepository, InMemoryRepository, InMemoryRoleRepository,
    InMemoryTextChannelRepository, InMemoryUserRepository, InMemoryVoiceChannelRepository,
    InMemoryVoiceStateRepository,
};
use std::{
    collections::{BTreeSet, HashSet},
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    sync::{Arc, Mutex},
};
use twilight_model::id::{AttachmentId, ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct InMemoryBackendError;

impl Display for InMemoryBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("this can't be constructed")
    }
}

impl Error for InMemoryBackendError {}

#[derive(Debug, Default)]
struct InMemoryBackendRef {
    attachments: DashMap<AttachmentId, AttachmentEntity>,
    channels_category: DashMap<ChannelId, CategoryChannelEntity>,
    channels_private: DashMap<ChannelId, PrivateChannelEntity>,
    channels_text: DashMap<ChannelId, TextChannelEntity>,
    channels_stage: DashMap<ChannelId, VoiceChannelEntity>,
    channels_voice: DashMap<ChannelId, VoiceChannelEntity>,
    channel_messages: DashMap<ChannelId, BTreeSet<MessageId>>,
    config: Config,
    emojis: DashMap<EmojiId, EmojiEntity>,
    groups: DashMap<ChannelId, GroupEntity>,
    guilds: DashMap<GuildId, GuildEntity>,
    guild_channels: DashMap<GuildId, HashSet<ChannelId>>,
    guild_emojis: DashMap<GuildId, HashSet<EmojiId>>,
    guild_members: DashMap<GuildId, HashSet<UserId>>,
    guild_presences: DashMap<GuildId, HashSet<UserId>>,
    guild_roles: DashMap<GuildId, HashSet<RoleId>>,
    guild_voice_states: DashMap<GuildId, HashSet<UserId>>,
    members: DashMap<(GuildId, UserId), MemberEntity>,
    messages: DashMap<MessageId, MessageEntity>,
    presences: DashMap<(GuildId, UserId), PresenceEntity>,
    roles: DashMap<RoleId, RoleEntity>,
    users: DashMap<UserId, UserEntity>,
    user_current: Mutex<Option<CurrentUserEntity>>,
    user_guilds: DashMap<UserId, Vec<GuildId>>,
    voice_states: DashMap<(GuildId, UserId), VoiceStateEntity>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct InMemoryBackendBuilder(Config);

impl InMemoryBackendBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> InMemoryBackend {
        InMemoryBackend(Arc::new(InMemoryBackendRef {
            config: self.0,
            ..InMemoryBackendRef::default()
        }))
    }

    pub fn entity_types(&mut self, entity_types: EntityType) -> &mut Self {
        *self.0.entity_types_mut() = entity_types;

        self
    }

    pub fn message_cache_size(&mut self, message_cache_size: usize) -> &mut Self {
        *self.0.message_cache_size_mut() = message_cache_size;

        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryBackend(Arc<InMemoryBackendRef>);

impl InMemoryBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> InMemoryBackendBuilder {
        InMemoryBackendBuilder::new()
    }

    pub fn config(&self) -> Config {
        self.0.config
    }

    fn repo<T>(&self) -> InMemoryRepository<T> {
        InMemoryRepository(self.clone(), PhantomData)
    }
}

impl Backend for InMemoryBackend {
    type Error = InMemoryBackendError;
    type AttachmentRepository = InMemoryAttachmentRepository;
    type CategoryChannelRepository = InMemoryCategoryChannelRepository;
    type CurrentUserRepository = InMemoryCurrentUserRepository;
    type EmojiRepository = InMemoryEmojiRepository;
    type GroupRepository = InMemoryGroupRepository;
    type GuildRepository = InMemoryGuildRepository;
    type MemberRepository = InMemoryMemberRepository;
    type MessageRepository = InMemoryMessageRepository;
    type PresenceRepository = InMemoryPresenceRepository;
    type PrivateChannelRepository = InMemoryPrivateChannelRepository;
    type RoleRepository = InMemoryRoleRepository;
    type TextChannelRepository = InMemoryTextChannelRepository;
    type UserRepository = InMemoryUserRepository;
    type VoiceChannelRepository = InMemoryVoiceChannelRepository;
    type VoiceStateRepository = InMemoryVoiceStateRepository;

    fn attachments(&self) -> Self::AttachmentRepository {
        self.repo()
    }

    fn category_channels(&self) -> Self::CategoryChannelRepository {
        self.repo()
    }

    fn current_user(&self) -> Self::CurrentUserRepository {
        self.repo()
    }

    fn emojis(&self) -> Self::EmojiRepository {
        self.repo()
    }

    fn groups(&self) -> Self::GroupRepository {
        self.repo()
    }

    fn guilds(&self) -> Self::GuildRepository {
        self.repo()
    }

    fn members(&self) -> Self::MemberRepository {
        self.repo()
    }

    fn messages(&self) -> Self::MessageRepository {
        self.repo()
    }

    fn presences(&self) -> Self::PresenceRepository {
        self.repo()
    }

    fn private_channels(&self) -> Self::PrivateChannelRepository {
        self.repo()
    }

    fn roles(&self) -> Self::RoleRepository {
        self.repo()
    }

    fn text_channels(&self) -> Self::TextChannelRepository {
        self.repo()
    }

    fn users(&self) -> Self::UserRepository {
        self.repo()
    }

    fn voice_channels(&self) -> Self::VoiceChannelRepository {
        self.repo()
    }

    fn voice_states(&self) -> Self::VoiceStateRepository {
        self.repo()
    }
}
