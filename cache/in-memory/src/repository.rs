use crate::{config::EntityType, InMemoryBackend, InMemoryBackendError};
use dashmap::DashMap;
use futures_util::{
    future::{self, FutureExt},
    stream::{self, StreamExt},
};
use star_cache_base::{
    entity::{
        channel::{
            AttachmentEntity, AttachmentRepository, CategoryChannelEntity,
            CategoryChannelRepository, GroupEntity, GroupRepository, GuildChannelEntity,
            MessageEntity, PrivateChannelEntity, TextChannelEntity, VoiceChannelEntity,
        },
        gateway::PresenceEntity,
        guild::{
            EmojiEntity, EmojiRepository, GuildEntity, GuildRepository, MemberEntity,
            MemberRepository, RoleEntity,
        },
        user::{CurrentUserEntity, CurrentUserRepository, UserEntity},
        voice::VoiceStateEntity,
    },
    repository::{
        GetEntityFuture, ListEntitiesFuture, ListEntityIdsFuture, RemoveEntityFuture,
        SingleEntityRepository, UpsertEntityFuture,
    },
    Entity, Repository,
};
use std::{marker::PhantomData, sync::Mutex};
use twilight_model::id::{AttachmentId, ChannelId, EmojiId, GuildId, RoleId, UserId};

pub type InMemoryAttachmentRepository = InMemoryRepository<AttachmentEntity>;
pub type InMemoryCategoryChannelRepository = InMemoryRepository<CategoryChannelEntity>;
pub type InMemoryCurrentUserRepository = InMemoryRepository<CurrentUserEntity>;
pub type InMemoryEmojiRepository = InMemoryRepository<EmojiEntity>;
pub type InMemoryGroupRepository = InMemoryRepository<GroupEntity>;
pub type InMemoryGuildRepository = InMemoryRepository<GuildEntity>;
pub type InMemoryMemberRepository = InMemoryRepository<MemberEntity>;
pub type InMemoryMessageRepository = InMemoryRepository<MessageEntity>;
pub type InMemoryPresenceRepository = InMemoryRepository<PresenceEntity>;
pub type InMemoryPrivateChannelRepository = InMemoryRepository<PrivateChannelEntity>;
pub type InMemoryRoleRepository = InMemoryRepository<RoleEntity>;
pub type InMemoryTextChannelRepository = InMemoryRepository<TextChannelEntity>;
pub type InMemoryUserRepository = InMemoryRepository<UserEntity>;
pub type InMemoryVoiceChannelRepository = InMemoryRepository<VoiceChannelEntity>;
pub type InMemoryVoiceStateRepository = InMemoryRepository<VoiceStateEntity>;

pub trait EntityExt: Clone + Entity {
    const TYPE: EntityType;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized;
}

impl EntityExt for AttachmentEntity {
    const TYPE: EntityType = EntityType::ATTACHMENT;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.attachments
    }
}

impl EntityExt for CategoryChannelEntity {
    const TYPE: EntityType = EntityType::CHANNEL_CATEGORY;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.channels_category
    }
}

impl EntityExt for EmojiEntity {
    const TYPE: EntityType = EntityType::EMOJI;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.emojis
    }
}

impl EntityExt for GroupEntity {
    const TYPE: EntityType = EntityType::CHANNEL_GROUP;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.groups
    }
}

impl EntityExt for GuildEntity {
    const TYPE: EntityType = EntityType::GUILD;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.guilds
    }
}

impl EntityExt for MemberEntity {
    const TYPE: EntityType = EntityType::MEMBER;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.members
    }
}

impl EntityExt for MessageEntity {
    const TYPE: EntityType = EntityType::MESSAGE;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.messages
    }
}

impl EntityExt for PresenceEntity {
    const TYPE: EntityType = EntityType::PRESENCE;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.presences
    }
}

impl EntityExt for PrivateChannelEntity {
    const TYPE: EntityType = EntityType::CHANNEL_PRIVATE;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.channels_private
    }
}

impl EntityExt for RoleEntity {
    const TYPE: EntityType = EntityType::ROLE;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.roles
    }
}

impl EntityExt for TextChannelEntity {
    const TYPE: EntityType = EntityType::CHANNEL_TEXT;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.channels_text
    }
}

impl EntityExt for UserEntity {
    const TYPE: EntityType = EntityType::USER;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.users
    }
}

impl EntityExt for VoiceChannelEntity {
    const TYPE: EntityType = EntityType::CHANNEL_VOICE;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.channels_voice
    }
}

impl EntityExt for VoiceStateEntity {
    const TYPE: EntityType = EntityType::VOICE_STATE;

    fn map(backend: &InMemoryBackend) -> &DashMap<Self::Id, Self>
    where
        Self: Sized,
    {
        &backend.0.voice_states
    }
}

pub trait SingleEntityExt: Clone + Entity {
    const TYPE: EntityType;

    fn lock(backend: &InMemoryBackend) -> &Mutex<Option<Self>>
    where
        Self: Sized;
}

impl SingleEntityExt for CurrentUserEntity {
    const TYPE: EntityType = EntityType::USER_CURRENT;

    fn lock(backend: &InMemoryBackend) -> &Mutex<Option<Self>>
    where
        Self: Sized,
    {
        &backend.0.user_current
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryRepository<T>(pub(crate) InMemoryBackend, pub(crate) PhantomData<T>);

impl<E: EntityExt> Repository<E, InMemoryBackend> for InMemoryRepository<E> {
    fn backend(&self) -> InMemoryBackend {
        self.0.clone()
    }

    fn get(&self, entity_id: E::Id) -> GetEntityFuture<'_, E, InMemoryBackendError> {
        future::ok(E::map(&self.0).get(&entity_id).map(|r| r.value().clone())).boxed()
    }

    fn list(&self) -> ListEntitiesFuture<'_, E, InMemoryBackendError> {
        let values = E::map(&self.0)
            .into_iter()
            .map(|r| *r.key())
            .filter_map(move |key| E::map(&self.0).get(&key).map(|r| r.value().clone()));

        let stream = stream::iter(values).map(Ok).boxed();

        future::ok(stream).boxed()
    }

    fn remove(&self, entity_id: E::Id) -> RemoveEntityFuture<'_, InMemoryBackendError> {
        E::map(&self.0).remove(&entity_id);

        future::ok(()).boxed()
    }

    fn upsert(&self, entity: E) -> UpsertEntityFuture<'_, InMemoryBackendError> {
        if !self.0.config().entity_types().contains(E::TYPE) {
            return future::ok(()).boxed();
        }

        E::map(&self.0).insert(entity.id(), entity);

        future::ok(()).boxed()
    }
}

impl SingleEntityRepository<CurrentUserEntity, InMemoryBackend>
    for InMemoryRepository<CurrentUserEntity>
{
    fn backend(&self) -> InMemoryBackend {
        self.0.clone()
    }

    fn get(&self) -> GetEntityFuture<'_, CurrentUserEntity, InMemoryBackendError> {
        future::ok(
            CurrentUserEntity::lock(&self.0)
                .lock()
                .expect("current user poisoned")
                .clone(),
        )
        .boxed()
    }

    fn remove(&self) -> RemoveEntityFuture<'_, InMemoryBackendError> {
        CurrentUserEntity::lock(&self.0)
            .lock()
            .expect("current user poisoned")
            .take();

        future::ok(()).boxed()
    }

    fn upsert(&self, entity: CurrentUserEntity) -> UpsertEntityFuture<'_, InMemoryBackendError> {
        if !self
            .0
            .config()
            .entity_types()
            .contains(CurrentUserEntity::TYPE)
        {
            return future::ok(()).boxed();
        }

        CurrentUserEntity::lock(&self.0)
            .lock()
            .expect("current user poisoned")
            .replace(entity);

        future::ok(()).boxed()
    }
}

impl AttachmentRepository<InMemoryBackend> for InMemoryAttachmentRepository {
    fn message(
        &self,
        attachment_id: AttachmentId,
    ) -> GetEntityFuture<'_, MessageEntity, InMemoryBackendError> {
        let message = self
            .0
             .0
            .attachments
            .get(&attachment_id)
            .map(|attachment| attachment.message_id)
            .and_then(|id| (self.0).0.messages.get(&id))
            .map(|r| r.value().clone());

        future::ok(message).boxed()
    }
}

impl CategoryChannelRepository<InMemoryBackend> for InMemoryCategoryChannelRepository {
    fn guild(
        &self,
        channel_id: ChannelId,
    ) -> GetEntityFuture<'_, GuildEntity, InMemoryBackendError> {
        let guild = self
            .0
             .0
            .channels_category
            .get(&channel_id)
            .and_then(|channel| channel.guild_id)
            .and_then(|id| (self.0).0.guilds.get(&id))
            .map(|r| r.value().clone());

        future::ok(guild).boxed()
    }
}

impl CurrentUserRepository<InMemoryBackend> for InMemoryCurrentUserRepository {
    fn guild_ids(&self) -> ListEntityIdsFuture<'_, GuildId, InMemoryBackendError> {
        let current_user_fut = self.get();

        Box::pin(async move {
            let user = if let Some(user) = current_user_fut.await? {
                user
            } else {
                return Ok(stream::empty().boxed());
            };

            let stream = (self.0).0.user_guilds.get(&user.id).map_or_else(
                || stream::empty().boxed(),
                |r| stream::iter(r.value().iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed(),
            );

            Ok(stream)
        })
    }

    fn guilds(&self) -> ListEntitiesFuture<'_, GuildEntity, InMemoryBackendError> {
        Box::pin(async move {
            let user = if let Some(user) = self.get().await? {
                user
            } else {
                return Ok(stream::empty().boxed());
            };

            let guild_ids = match (self.0).0.user_guilds.get(&user.id) {
                Some(user_guilds) => user_guilds.clone(),
                None => return Ok(stream::empty().boxed()),
            };

            let iter = guild_ids
                .into_iter()
                .filter_map(move |id| (self.0).0.guilds.get(&id).map(|r| Ok(r.value().clone())));

            let stream = stream::iter(iter).boxed();

            Ok(stream)
        })
    }
}

impl EmojiRepository<InMemoryBackend> for InMemoryEmojiRepository {
    fn guild(&self, emoji_id: EmojiId) -> GetEntityFuture<'_, GuildEntity, InMemoryBackendError> {
        let guild = self
            .0
             .0
            .emojis
            .get(&emoji_id)
            .map(|emoji| emoji.guild_id)
            .and_then(|id| (self.0).0.guilds.get(&id))
            .map(|r| r.value().clone());

        future::ok(guild).boxed()
    }

    fn roles(&self, emoji_id: EmojiId) -> ListEntitiesFuture<'_, RoleEntity, InMemoryBackendError> {
        let role_ids = match (self.0).0.emojis.get(&emoji_id) {
            Some(emoji) => emoji.role_ids.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = role_ids
            .into_iter()
            .filter_map(move |id| (self.0).0.roles.get(&id).map(|r| Ok(r.value().clone())));

        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn user(&self, emoji_id: EmojiId) -> GetEntityFuture<'_, UserEntity, InMemoryBackendError> {
        let user = self
            .0
             .0
            .emojis
            .get(&emoji_id)
            .and_then(|emoji| emoji.user_id)
            .and_then(|id| (self.0).0.users.get(&id))
            .map(|r| r.value().clone());

        future::ok(user).boxed()
    }
}

impl GroupRepository<InMemoryBackend> for InMemoryGroupRepository {
    fn last_message(
        &self,
        group_id: ChannelId,
    ) -> GetEntityFuture<'_, MessageEntity, InMemoryBackendError> {
        let message = self
            .0
             .0
            .groups
            .get(&group_id)
            .and_then(|group| group.last_message_id)
            .and_then(|id| (self.0).0.messages.get(&id))
            .map(|r| r.value().clone());

        future::ok(message).boxed()
    }

    fn owner(&self, group_id: ChannelId) -> GetEntityFuture<'_, UserEntity, InMemoryBackendError> {
        let guild = self
            .0
             .0
            .groups
            .get(&group_id)
            .map(|message| message.owner_id)
            .and_then(|id| (self.0).0.users.get(&id))
            .map(|r| r.value().clone());

        future::ok(guild).boxed()
    }

    fn recipients(
        &self,
        group_id: ChannelId,
    ) -> ListEntitiesFuture<'_, UserEntity, InMemoryBackendError> {
        let recipient_ids = match (self.0).0.groups.get(&group_id) {
            Some(group) => group.recipient_ids.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = recipient_ids
            .into_iter()
            .filter_map(move |id| (self.0).0.users.get(&id).map(|r| Ok(r.value().clone())));

        let stream = stream::iter(iter).boxed();
        future::ok(stream).boxed()
    }
}

impl GuildRepository<InMemoryBackend> for InMemoryGuildRepository {
    fn afk_channel(
        &self,
        guild_id: GuildId,
    ) -> GetEntityFuture<'_, VoiceChannelEntity, InMemoryBackendError> {
        let guild = self
            .0
             .0
            .guilds
            .get(&guild_id)
            .and_then(|guild| guild.afk_channel_id)
            .and_then(|id| (self.0).0.channels_voice.get(&id))
            .map(|r| r.value().clone());

        future::ok(guild).boxed()
    }

    fn channel_ids(
        &self,
        guild_id: GuildId,
    ) -> ListEntitiesFuture<'_, ChannelId, InMemoryBackendError> {
        let stream = (self.0).0.guild_channels.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed(),
        );

        future::ok(stream).boxed()
    }

    fn channels(
        &self,
        guild_id: GuildId,
    ) -> ListEntitiesFuture<'_, GuildChannelEntity, InMemoryBackendError> {
        let channel_ids = match (self.0).0.guild_channels.get(&guild_id) {
            Some(guild_channels) => guild_channels.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = channel_ids.into_iter().filter_map(move |id| {
            if let Some(r) = (self.0).0.channels_text.get(&id) {
                return Some(Ok(GuildChannelEntity::Text(r.value().clone())));
            }

            if let Some(r) = (self.0).0.channels_voice.get(&id) {
                return Some(Ok(GuildChannelEntity::Voice(r.value().clone())));
            }

            if let Some(r) = (self.0).0.channels_category.get(&id) {
                return Some(Ok(GuildChannelEntity::Category(r.value().clone())));
            }

            None
        });

        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn emoji_ids(
        &self,
        guild_id: GuildId,
    ) -> ListEntityIdsFuture<'_, EmojiId, InMemoryBackendError> {
        let stream = (self.0).0.guild_emojis.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed(),
        );

        future::ok(stream).boxed()
    }

    fn emojis(
        &self,
        guild_id: GuildId,
    ) -> ListEntitiesFuture<'_, EmojiEntity, InMemoryBackendError> {
        let emoji_ids = match (self.0).0.guild_emojis.get(&guild_id) {
            Some(guild_emojis) => guild_emojis.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = emoji_ids
            .into_iter()
            .filter_map(move |id| (self.0).0.emojis.get(&id).map(|r| Ok(r.value().clone())));
        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn member_ids(
        &self,
        guild_id: GuildId,
    ) -> ListEntityIdsFuture<'_, UserId, InMemoryBackendError> {
        let stream = (self.0).0.guild_members.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed(),
        );

        future::ok(stream).boxed()
    }

    fn members(
        &self,
        guild_id: GuildId,
    ) -> ListEntitiesFuture<'_, MemberEntity, InMemoryBackendError> {
        let user_ids = match (self.0).0.guild_members.get(&guild_id) {
            Some(guild_members) => guild_members.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = user_ids.into_iter().filter_map(move |id| {
            self.0
                 .0
                .members
                .get(&(guild_id, id))
                .map(|r| Ok(r.value().clone()))
        });

        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn owner(&self, guild_id: GuildId) -> GetEntityFuture<'_, UserEntity, InMemoryBackendError> {
        let guild = self
            .0
             .0
            .guilds
            .get(&guild_id)
            .map(|guild| guild.owner_id)
            .and_then(|id| (self.0).0.users.get(&id))
            .map(|r| r.value().clone());

        future::ok(guild).boxed()
    }

    fn presence_ids(
        &self,
        guild_id: GuildId,
    ) -> ListEntityIdsFuture<'_, UserId, InMemoryBackendError> {
        let stream = (self.0).0.guild_presences.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed(),
        );

        future::ok(stream).boxed()
    }

    fn presences(
        &self,
        guild_id: GuildId,
    ) -> ListEntitiesFuture<'_, PresenceEntity, InMemoryBackendError> {
        let user_ids = match (self.0).0.guild_presences.get(&guild_id) {
            Some(guild_presences) => guild_presences.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = user_ids.into_iter().filter_map(move |id| {
            self.0
                 .0
                .presences
                .get(&(guild_id, id))
                .map(|r| Ok(r.value().clone()))
        });

        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn role_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, RoleId, InMemoryBackendError> {
        let stream = (self.0).0.guild_roles.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed(),
        );

        future::ok(stream).boxed()
    }

    fn roles(&self, guild_id: GuildId) -> ListEntitiesFuture<'_, RoleEntity, InMemoryBackendError> {
        let role_ids = match (self.0).0.guild_roles.get(&guild_id) {
            Some(guild_roles) => guild_roles.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = role_ids
            .into_iter()
            .filter_map(move |id| (self.0).0.roles.get(&id).map(|r| Ok(r.value().clone())));

        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn rules_channel(
        &self,
        guild_id: GuildId,
    ) -> GetEntityFuture<'_, TextChannelEntity, InMemoryBackendError> {
        let guild = self
            .0
             .0
            .guilds
            .get(&guild_id)
            .and_then(|guild| guild.rules_channel_id)
            .and_then(|id| (self.0).0.channels_text.get(&id))
            .map(|r| r.value().clone());

        future::ok(guild).boxed()
    }

    fn system_channel(
        &self,
        guild_id: GuildId,
    ) -> GetEntityFuture<'_, TextChannelEntity, InMemoryBackendError> {
        let guild = self
            .0
             .0
            .guilds
            .get(&guild_id)
            .and_then(|guild| guild.system_channel_id)
            .and_then(|id| (self.0).0.channels_text.get(&id))
            .map(|r| r.value().clone());

        future::ok(guild).boxed()
    }

    fn voice_state_ids(
        &self,
        guild_id: GuildId,
    ) -> ListEntityIdsFuture<'_, UserId, InMemoryBackendError> {
        let stream = (self.0).0.guild_voice_states.get(&guild_id).map_or_else(
            || stream::empty().boxed(),
            |set| stream::iter(set.iter().map(|x| Ok(*x)).collect::<Vec<_>>()).boxed(),
        );

        future::ok(stream).boxed()
    }

    fn voice_states(
        &self,
        guild_id: GuildId,
    ) -> ListEntitiesFuture<'_, VoiceStateEntity, InMemoryBackendError> {
        let user_ids = match (self.0).0.guild_voice_states.get(&guild_id) {
            Some(guild_voice_states) => guild_voice_states.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };

        let iter = user_ids.into_iter().filter_map(move |id| {
            self.0
                 .0
                .voice_states
                .get(&(guild_id, id))
                .map(|r| Ok(r.value().clone()))
        });

        let stream = stream::iter(iter).boxed();

        future::ok(stream).boxed()
    }

    fn widget_channel(
        &self,
        guild_id: GuildId,
    ) -> GetEntityFuture<'_, GuildChannelEntity, InMemoryBackendError> {
        let id = match (self.0).0.guilds.get(&guild_id) {
            Some(guild) if guild.widget_channel_id.is_some() => guild.widget_channel_id.unwrap(),
            _ => return future::ok(None).boxed(),
        };

        if let Some(r) = (self.0).0.channels_text.get(&id) {
            let entity = GuildChannelEntity::Text(r.value().clone());

            return future::ok(Some(entity)).boxed();
        }

        if let Some(r) = (self.0).0.channels_voice.get(&id) {
            let entity = GuildChannelEntity::Voice(r.value().clone());

            return future::ok(Some(entity)).boxed();
        }

        if let Some(r) = (self.0).0.channels_category.get(&id) {
            let entity = GuildChannelEntity::Category(r.value().clone());

            return future::ok(Some(entity)).boxed();
        }

        future::ok(None).boxed()
    }
}

impl MemberRepository<InMemoryBackend> for InMemoryMemberRepository {
    fn hoisted_role(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> GetEntityFuture<'_, RoleEntity, InMemoryBackendError> {
        let role = self
            .0
             .0
            .members
            .get(&(guild_id, user_id))
            .and_then(|member| member.hoisted_role_id)
            .and_then(|id| (self.0).0.roles.get(&id))
            .map(|r| r.value().clone());

        future::ok(role).boxed()
    }

    fn roles(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> ListEntitiesFuture<'_, RoleEntity, InMemoryBackendError> {
        let role_ids = match (self.0).0.members.get(&(guild_id, user_id)) {
            Some(member) => member.role_ids.clone(),
            None => return future::ok(stream::empty().boxed()).boxed(),
        };
    }
}
