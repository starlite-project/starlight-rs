use super::{GetEntityFuture, ListEntitiesFuture, ListEntityIdsFuture, RemoveEntitiesFuture, RemoveEntityFuture, UpsertEntitiesFuture, UpsertEntityFuture};
use crate::{Backend, Entity, Repository, entity::{channel::{AttachmentRepository, CategoryChannelRepository, GroupRepository, GuildChannelEntity, MessageEntity, VoiceChannelEntity}, gateway::PresenceEntity, guild::{EmojiEntity, EmojiRepository, GuildEntity, GuildRepository, MemberEntity, RoleEntity}, user::UserEntity}};
use futures_util::{
    future::{self, FutureExt},
    stream::{self, StreamExt},
};
use twilight_model::id::{AttachmentId, ChannelId, EmojiId, GuildId, RoleId, UserId};

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
        future::ok(None).boxed()
    }

    fn list(&self) -> ListEntitiesFuture<'_, E, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
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
        future::ok(None).boxed()
    }
}

impl<B: Backend + Clone + Send> CategoryChannelRepository<B> for NoopRepository<B> {
    fn guild(&self, _: ChannelId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        future::ok(None).boxed()
    }
}

impl<B: Backend + Clone + Send> EmojiRepository<B> for NoopRepository<B> {
    fn guild(&self, _: EmojiId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        future::ok(None).boxed()
    }

    fn roles(&self, _: EmojiId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn user(&self, _: EmojiId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        future::ok(None).boxed()
    }
}

impl<B: Backend + Clone + Send> GroupRepository<B> for NoopRepository<B> {
    fn last_message(&self, _: ChannelId) -> GetEntityFuture<'_, MessageEntity, B::Error> {
        future::ok(None).boxed()
    }

    fn owner(&self, _: ChannelId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        future::ok(None).boxed()
    }

    fn recipients(&self, _: ChannelId) -> ListEntitiesFuture<'_, UserEntity, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }
}

impl<B: Backend + Clone + Send> GuildRepository<B> for NoopRepository<B> {
    fn afk_channel(&self, guild_id: GuildId) -> GetEntityFuture<'_, VoiceChannelEntity, B::Error> {
        future::ok(None).boxed()
    }

    fn channel_ids(&self, _: GuildId) -> ListEntitiesFuture<'_, ChannelId, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn channels(&self, _: GuildId) -> ListEntitiesFuture<'_, GuildChannelEntity, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn emoji_ids(&self, _: GuildId) -> super::ListEntityIdsFuture<'_, EmojiId, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn emojis(&self, _: GuildId) -> ListEntitiesFuture<'_, EmojiEntity, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn member_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn members(&self, _: GuildId) -> ListEntitiesFuture<'_, MemberEntity, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn owner(&self, _: GuildId) -> GetEntityFuture<'_, UserEntity, B::Error> {
        future::ok(None).boxed()
    }

    fn presence_ids(&self, _: GuildId) -> ListEntityIdsFuture<'_, UserId, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn presences(&self, _: GuildId) -> ListEntitiesFuture<'_, PresenceEntity, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn role_ids(&self, guild_id: GuildId) -> ListEntityIdsFuture<'_, RoleId, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }

    fn roles(&self, guild_id: GuildId) -> ListEntitiesFuture<'_, RoleEntity, B::Error> {
        future::ok(stream::empty().boxed()).boxed()
    }
}
