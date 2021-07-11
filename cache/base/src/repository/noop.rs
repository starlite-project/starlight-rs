use super::{
    GetEntityFuture, ListEntitiesFuture, ListEntityIdsFuture, RemoveEntitiesFuture,
    RemoveEntityFuture, UpsertEntitiesFuture, UpsertEntityFuture,
};
use crate::{
    entity::{
        channel::{
            AttachmentEntity, AttachmentRepository, CategoryChannelEntity,
            CategoryChannelRepository, ChannelEntity, GroupRepository, GuildChannelEntity,
            MessageEntity, MessageRepository, PrivateChannelRepository, TextChannelEntity,
            TextChannelRepository, VoiceChannelEntity, VoiceChannelRepository,
        },
        gateway::{PresenceEntity, PresenceRepository},
        guild::{
            EmojiEntity, EmojiRepository, GuildEntity, GuildRepository, MemberEntity,
            MemberRepository, RoleEntity, RoleRepository,
        },
        user::{UserEntity, UserRepository},
        voice::VoiceStateRepository,
    },
    Backend, Entity, Repository,
};
use futures_util::{
    future::{self, FutureExt},
    stream::{self, StreamExt},
};
use twilight_model::id::{AttachmentId, ChannelId, EmojiId, GuildId, MessageId, RoleId, UserId};

macro_rules! empty_future {
    () => {
        future::ok(None).boxed()
    };
}

macro_rules! empty_stream {
    () => {
        future::ok(stream::empty().boxed()).boxed()
    };
}

#[derive(Debug, Clone)]
pub struct NoopRepository<B>(B);

impl<B: Backend + Clone> NoopRepository<B> {
    pub fn new(backend: B) -> Self {
        Self(backend)
    }
}

impl<B: Backend + Clone, E: Entity + 'static> Repository<E, B> for NoopRepository<B> {
    fn backend(&self) -> B {
        self.0.clone()
    }

    fn get(&self, _: E::Id) -> GetEntityFuture<'_, E, B::Error> {
        empty_future!()
    }

    fn list(&self) -> ListEntitiesFuture<'_, E, B::Error> {
        empty_stream!()
    }

    fn remove(&self, _: E::Id) -> RemoveEntityFuture<'_, B::Error> {
        future::ok(()).boxed()
    }

    fn upsert(&self, _: E) -> UpsertEntityFuture<'_, B::Error> {
        future::ok(()).boxed()
    }

    fn remove_bulk<T: Iterator<Item = E::Id>>(&self, _: T) -> RemoveEntitiesFuture<'_, B::Error> {
        future::ok(()).boxed()
    }

    fn upsert_bulk<T: Iterator<Item = E> + Send>(
        &self,
        _: T,
    ) -> UpsertEntitiesFuture<'_, B::Error> {
        future::ok(()).boxed()
    }
}

impl<B: Backend + Clone + Send> AttachmentRepository<B> for NoopRepository<B> {
    fn message(&self, _: AttachmentId) -> GetEntityFuture<'_, MessageEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> CategoryChannelRepository<B> for NoopRepository<B> {
    fn guild(&self, _: ChannelId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> EmojiRepository<B> for NoopRepository<B> {
    fn guild(&self, _: EmojiId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        empty_future!()
    }

    fn roles(&self, _: EmojiId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        empty_stream!()
    }

    fn user(&self, _: EmojiId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> GroupRepository<B> for NoopRepository<B> {
    fn last_message(&self, _: ChannelId) -> GetEntityFuture<'_, MessageEntity, B::Error> {
        empty_future!()
    }

    fn owner(&self, _: ChannelId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        empty_future!()
    }

    fn recipients(&self, _: ChannelId) -> ListEntitiesFuture<'_, UserEntity, B::Error> {
        empty_stream!()
    }
}

impl<B: Backend + Clone + Send> GuildRepository<B> for NoopRepository<B> {
    fn afk_channel(&self, _: GuildId) -> GetEntityFuture<'_, VoiceChannelEntity, B::Error> {
        empty_future!()
    }

    fn channel_ids(&self, _: GuildId) -> ListEntitiesFuture<'_, ChannelId, B::Error> {
        empty_stream!()
    }

    fn channels(&self, _: GuildId) -> ListEntitiesFuture<'_, GuildChannelEntity, B::Error> {
        empty_stream!()
    }

    fn emoji_ids(&self, _: GuildId) -> super::ListEntityIdsFuture<'_, EmojiId, B::Error> {
        empty_stream!()
    }

    fn emojis(&self, _: GuildId) -> ListEntitiesFuture<'_, EmojiEntity, B::Error> {
        empty_stream!()
    }

    fn member_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error> {
        empty_stream!()
    }

    fn members(&self, _: GuildId) -> ListEntitiesFuture<'_, MemberEntity, B::Error> {
        empty_stream!()
    }

    fn owner(&self, _: GuildId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        empty_future!()
    }

    fn presence_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error> {
        empty_stream!()
    }

    fn presences(&self, _: GuildId) -> ListEntitiesFuture<'_, PresenceEntity, B::Error> {
        empty_stream!()
    }

    fn role_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, RoleId, B::Error> {
        empty_stream!()
    }

    fn roles(&self, _: GuildId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        empty_stream!()
    }

    fn rules_channel(&self, _: GuildId) -> GetEntityFuture<'_, TextChannelEntity, B::Error> {
        empty_future!()
    }

    fn system_channel(&self, _: GuildId) -> GetEntityFuture<'_, TextChannelEntity, B::Error> {
        empty_future!()
    }

    fn voice_state_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error> {
        empty_stream!()
    }

    fn voice_states(
        &self,
        _: GuildId,
    ) -> ListEntitiesFuture<'_, crate::entity::voice::VoiceStateEntity, B::Error> {
        empty_stream!()
    }

    fn widget_channel(&self, _: GuildId) -> GetEntityFuture<'_, GuildChannelEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> MemberRepository<B> for NoopRepository<B> {
    fn hoisted_role(&self, _: GuildId, _: UserId) -> GetEntityFuture<'_, RoleEntity, B::Error> {
        empty_future!()
    }

    fn roles(&self, _: GuildId, _: UserId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        empty_stream!()
    }
}

impl<B: Backend + Clone + Send> MessageRepository<B> for NoopRepository<B> {
    fn attachments(&self, _: MessageId) -> ListEntitiesFuture<'_, AttachmentEntity, B::Error> {
        empty_stream!()
    }

    fn author(&self, _: MessageId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        empty_future!()
    }

    fn channel(&self, _: MessageId) -> GetEntityFuture<'_, ChannelEntity, B::Error> {
        empty_future!()
    }

    fn guild(&self, _: MessageId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        empty_future!()
    }

    fn mention_channels(
        &self,
        _: MessageId,
    ) -> ListEntitiesFuture<'_, TextChannelEntity, B::Error> {
        empty_stream!()
    }

    fn mention_roles(&self, _: MessageId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        empty_stream!()
    }

    fn mentions(&self, _: MessageId) -> ListEntitiesFuture<'_, UserEntity, B::Error> {
        empty_stream!()
    }
}

impl<B: Backend + Clone + Send> PresenceRepository<B> for NoopRepository<B> {}

impl<B: Backend + Clone + Send> PrivateChannelRepository<B> for NoopRepository<B> {
    fn last_message(&self, _: ChannelId) -> GetEntityFuture<'_, MessageEntity, B::Error> {
        empty_future!()
    }

    fn recipient(&self, _: ChannelId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> RoleRepository<B> for NoopRepository<B> {
    fn guild(&self, _: RoleId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> TextChannelRepository<B> for NoopRepository<B> {
    fn guild(&self, _: ChannelId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        empty_future!()
    }

    fn last_message(&self, _: ChannelId) -> GetEntityFuture<'_, MessageEntity, B::Error> {
        empty_future!()
    }

    fn parent(&self, _: ChannelId) -> GetEntityFuture<'_, CategoryChannelEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> UserRepository<B> for NoopRepository<B> {
    fn guild_ids(&self, _: UserId) -> ListEntitiesFuture<'_, GuildId, B::Error> {
        empty_stream!()
    }

    fn guilds(&self, _: UserId) -> ListEntitiesFuture<'_, GuildEntity, B::Error> {
        empty_stream!()
    }
}

impl<B: Backend + Clone + Send> VoiceChannelRepository<B> for NoopRepository<B> {
    fn guild(&self, _: ChannelId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        empty_future!()
    }

    fn parent(&self, _: ChannelId) -> GetEntityFuture<'_, CategoryChannelEntity, B::Error> {
        empty_future!()
    }
}

impl<B: Backend + Clone + Send> VoiceStateRepository<B> for NoopRepository<B> {
    fn channel(&self, _: GuildId, _: UserId) -> GetEntityFuture<'_, VoiceChannelEntity, B::Error> {
        empty_future!()
    }
}
