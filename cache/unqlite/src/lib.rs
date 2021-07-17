use futures_util::future::{self, FutureExt};
use serde::{de::DeserializeOwned, Serialize};
use star_cache_base::{Backend, Cache, Entity, Repository, entity::{
        channel::{
            AttachmentEntity, AttachmentRepository, CategoryChannelEntity,
            CategoryChannelRepository, GroupEntity, GroupRepository, GuildChannelEntity,
            MessageEntity, MessageRepository, PrivateChannelEntity, PrivateChannelRepository,
            TextChannelEntity, TextChannelRepository, VoiceChannelEntity, VoiceChannelRepository,
        },
        gateway::{PresenceEntity, PresenceRepository},
        guild::{
            EmojiEntity, EmojiRepository, GuildEntity, GuildRepository, MemberEntity,
            MemberRepository, RoleEntity, RoleRepository,
        },
        user::{CurrentUserEntity, CurrentUserRepository, UserEntity, UserRepository},
        voice::{VoiceStateEntity, VoiceStateRepository},
    }, repository::{
        GetEntityFuture, ListEntitiesFuture, ListEntityIdsFuture, RemoveEntityFuture,
        SingleEntityRepository, UpsertEntityFuture,
    }};
use std::{marker::PhantomData, sync::Arc};
use twilight_model::id::{ChannelId, EmojiId, GuildId, RoleId, UserId};
use unqlite::{Error, UnQLite, KV};

pub type UnqliteCache = Cache<UnqliteBackend>;

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

pub struct UnqliteRepository<T>(UnqliteBackend, PhantomData<T>);

impl<T> UnqliteRepository<T> {
    const fn new(backend: UnqliteBackend) -> Self {
        Self(backend, PhantomData)
    }
}

impl<T: DeserializeOwned + Serialize + UnqliteEntity> Repository<T, UnqliteBackend>
    for UnqliteRepository<T>
{
    fn backend(&self) -> UnqliteBackend {
        self.0.clone()
    }

    fn get(&self, entity_id: T::Id) -> GetEntityFuture<'_, T, Error> {
        let bytes: Vec<u8> = (self.0).0.kv_fetch(T::key(entity_id)).unwrap();

        future::ok(Some(bincode::deserialize::<T>(&bytes).unwrap())).boxed()
    }

    fn list(&self) -> ListEntitiesFuture<'_, T, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn remove(&self, entity_id: T::Id) -> RemoveEntityFuture<'_, Error> {
        future::ready((self.0).0.kv_delete(T::key(entity_id))).boxed()
    }

    fn upsert(&self, entity: T) -> UpsertEntityFuture<'_, Error> {
        let bytes = entity.value();

        future::ready((self.0).0.kv_store(T::key(entity.id()), bytes)).boxed()
    }
}

impl<T: DeserializeOwned + Serialize + UnqliteSingleEntity>
    SingleEntityRepository<T, UnqliteBackend> for UnqliteRepository<T>
{
    fn backend(&self) -> UnqliteBackend {
        self.0.clone()
    }

    fn get(&self) -> GetEntityFuture<'_, T, Error> {
        let bytes = (self.0).0.kv_fetch(T::key()).unwrap();

        future::ok(Some(bincode::deserialize::<T>(&bytes).unwrap())).boxed()
    }

    fn remove(&self) -> RemoveEntityFuture<'_, Error> {
        future::ready((self.0).0.kv_delete(T::key())).boxed()
    }

    fn upsert(&self, entity: T) -> UpsertEntityFuture<'_, Error> {
        let bytes = entity.value();

        future::ready((self.0).0.kv_store(T::key(), bytes)).boxed()
    }
}

impl AttachmentRepository<UnqliteBackend> for UnqliteRepository<AttachmentEntity> {}

impl CategoryChannelRepository<UnqliteBackend> for UnqliteRepository<CategoryChannelEntity> {}

impl CurrentUserRepository<UnqliteBackend> for UnqliteRepository<CurrentUserEntity> {
    fn guild_ids(&self) -> ListEntityIdsFuture<'_, GuildId, Error> {
        unimplemented!("not implemented by this backend")
    }
}

impl EmojiRepository<UnqliteBackend> for UnqliteRepository<EmojiEntity> {}

impl GroupRepository<UnqliteBackend> for UnqliteRepository<GroupEntity> {}

impl GuildRepository<UnqliteBackend> for UnqliteRepository<GuildEntity> {
    fn channel_ids(&self, _: GuildId) -> ListEntitiesFuture<'_, ChannelId, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn channels(&self, _: GuildId) -> ListEntitiesFuture<'_, GuildChannelEntity, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn emoji_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, EmojiId, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn member_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn members(&self, _: GuildId) -> ListEntitiesFuture<'_, MemberEntity, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn presence_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn presences(&self, _: GuildId) -> ListEntitiesFuture<'_, PresenceEntity, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn role_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, RoleId, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn voice_state_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, Error> {
        unimplemented!("not implemented by this backend")
    }

    fn voice_states(&self, _: GuildId) -> ListEntitiesFuture<'_, VoiceStateEntity, Error> {
        unimplemented!("not implemented by this backend")
    }
}

impl MemberRepository<UnqliteBackend> for UnqliteRepository<MemberEntity> {}

impl MessageRepository<UnqliteBackend> for UnqliteRepository<MessageEntity> {}

impl PresenceRepository<UnqliteBackend> for UnqliteRepository<PresenceEntity> {}

impl PrivateChannelRepository<UnqliteBackend> for UnqliteRepository<PrivateChannelEntity> {}

impl RoleRepository<UnqliteBackend> for UnqliteRepository<RoleEntity> {}

impl TextChannelRepository<UnqliteBackend> for UnqliteRepository<TextChannelEntity> {}

impl VoiceChannelRepository<UnqliteBackend> for UnqliteRepository<VoiceChannelEntity> {}

impl VoiceStateRepository<UnqliteBackend> for UnqliteRepository<VoiceStateEntity> {}

impl UserRepository<UnqliteBackend> for UnqliteRepository<UserEntity> {
    fn guild_ids(&self, _: UserId) -> ListEntitiesFuture<'_, GuildId, Error> {
        unimplemented!("not implemented by this backend")
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

    fn repo<T>(&self) -> UnqliteRepository<T> {
        UnqliteRepository::new(self.clone())
    }
}

impl Backend for UnqliteBackend {
    type Error = Error;
    type AttachmentRepository = UnqliteRepository<AttachmentEntity>;
    type CategoryChannelRepository = UnqliteRepository<CategoryChannelEntity>;
    type CurrentUserRepository = UnqliteRepository<CurrentUserEntity>;
    type EmojiRepository = UnqliteRepository<EmojiEntity>;
    type GroupRepository = UnqliteRepository<GroupEntity>;
    type GuildRepository = UnqliteRepository<GuildEntity>;
    type MemberRepository = UnqliteRepository<MemberEntity>;
    type MessageRepository = UnqliteRepository<MessageEntity>;
    type PresenceRepository = UnqliteRepository<PresenceEntity>;
    type PrivateChannelRepository = UnqliteRepository<PrivateChannelEntity>;
    type RoleRepository = UnqliteRepository<RoleEntity>;
    type TextChannelRepository = UnqliteRepository<TextChannelEntity>;
    type UserRepository = UnqliteRepository<UserEntity>;
    type VoiceChannelRepository = UnqliteRepository<VoiceChannelEntity>;
    type VoiceStateRepository = UnqliteRepository<VoiceStateEntity>;

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
