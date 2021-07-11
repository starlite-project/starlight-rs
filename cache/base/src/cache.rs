use crate::{
    entity::{
        channel::{
            CategoryChannelEntity, GroupEntity, PrivateChannelEntity, TextChannelEntity,
            VoiceChannelEntity,
        },
        user::UserEntity,
    },
    Backend, Repository,
};
use futures_util::{
    future::{self, FutureExt, TryFutureExt},
    stream::{FuturesUnordered, StreamExt, TryStreamExt},
};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use twilight_model::{
    channel::{Channel, GuildChannel},
    gateway::{
        event::Event,
        payload::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
    },
};

fn noop<B: Backend>() -> Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send>> {
    future::ok(()).boxed()
}

pub trait CacheUpdate<B: Backend> {
    fn process<'a>(
        &'a self,
        cache: &'a Cache<B>,
    ) -> Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send + 'a>>;
}

pub struct ProcessFuture<'a, B: Backend> {
    inner: Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send + 'a>>,
}

impl<B: Backend> Future for ProcessFuture<'_, B> {
    type Output = Result<(), B::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Cache<B: Backend> {
    backend: Arc<B>,
    pub attachments: B::AttachmentRepository,
    pub category_channels: B::CategoryChannelRepository,
    pub current_user: B::CurrentUserRepository,
    pub emojis: B::EmojiRepository,
    pub groups: B::GroupRepository,
    pub guilds: B::GuildRepository,
    pub members: B::MemberRepository,
    pub messages: B::MessageRepository,
    pub presences: B::PresenceRepository,
    pub private_channels: B::PrivateChannelRepository,
    pub roles: B::RoleRepository,
    pub text_channels: B::TextChannelRepository,
    pub users: B::UserRepository,
    pub voice_channels: B::VoiceChannelRepository,
    pub voice_states: B::VoiceStateRepository,
}

impl<B: Backend + Default> Cache<B> {
    pub fn new() -> Self {
        Self::with_backend(B::default())
    }
}

impl<B: Backend> Cache<B> {
    pub fn with_backend(backend: impl Into<Arc<B>>) -> Self {
        let backend: Arc<B> = backend.into();
        let attachments = backend.attachments();
        let category_channels = backend.category_channels();
        let current_user = backend.current_user();
        let emojis = backend.emojis();
        let groups = backend.groups();
        let guilds = backend.guilds();
        let members = backend.members();
        let messages = backend.messages();
        let presences = backend.presences();
        let private_channels = backend.private_channels();
        let roles = backend.roles();
        let text_channels = backend.text_channels();
        let users = backend.users();
        let voice_channels = backend.voice_channels();
        let voice_states = backend.voice_states();

        Self {
            attachments,
            backend,
            category_channels,
            current_user,
            emojis,
            groups,
            guilds,
            members,
            messages,
            presences,
            private_channels,
            roles,
            text_channels,
            users,
            voice_channels,
            voice_states,
        }
    }

    pub fn backend(&self) -> &Arc<B> {
        &self.backend
    }

    pub fn process<'a>(&'a self, event: &'a Event) -> ProcessFuture<'a, B> {
        ProcessFuture {
            inner: event.process(self),
        }
    }
}

impl<B: Backend> CacheUpdate<B> for Event {
    fn process<'a>(
        &'a self,
        cache: &'a Cache<B>,
    ) -> Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send + 'a>> {
        match self {
            Self::BanAdd(_) => noop::<B>(),
            Self::BanRemove(_) => noop::<B>(),
            _ => todo!(),
        }
    }
}

impl<B: Backend> CacheUpdate<B> for ChannelCreate {
    fn process<'a>(
        &'a self,
        cache: &'a Cache<B>,
    ) -> Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send + 'a>> {
        match &self.0 {
            Channel::Group(group) => {
                let futures = FuturesUnordered::new();

                futures.push(
                    cache
                        .users
                        .upsert_bulk(group.recipients.iter().cloned().map(UserEntity::from)),
                );

                let entity = GroupEntity::from(group.clone());
                futures.push(cache.groups.upsert(entity));

                futures.try_collect().boxed()
            }
            Channel::Guild(GuildChannel::Category(c)) => {
                let entity = CategoryChannelEntity::from(c.clone());

                cache.category_channels.upsert(entity)
            }
            Channel::Guild(GuildChannel::Text(c)) => {
                let entity = TextChannelEntity::from(c.clone());

                cache.text_channels.upsert(entity)
            }
            Channel::Guild(GuildChannel::Voice(c)) => {
                let entity = VoiceChannelEntity::from(c.clone());

                cache.voice_channels.upsert(entity)
            }
            Channel::Private(c) => {
                let futures = FuturesUnordered::new();

                futures.push(
                    cache
                        .users
                        .upsert_bulk(c.recipients.iter().cloned().map(UserEntity::from)),
                );

                let entity = PrivateChannelEntity::from(c.clone());
                futures.push(cache.private_channels.upsert(entity));

                futures.try_collect().boxed()
            }
            _ => unimplemented!(),
        }
    }
}

impl<B: Backend> CacheUpdate<B> for ChannelDelete {
    fn process<'a>(
        &'a self,
        cache: &'a Cache<B>,
    ) -> Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send + 'a>> {
        match &self.0 {
            Channel::Group(group) => cache.groups.remove(group.id),
            Channel::Guild(GuildChannel::Category(c)) => cache.category_channels.remove(c.id),
            Channel::Guild(GuildChannel::Text(c)) => cache.text_channels.remove(c.id),
            Channel::Guild(GuildChannel::Voice(c)) => cache.voice_channels.remove(c.id),
            Channel::Private(c) => cache.private_channels.remove(c.id),
            _ => unimplemented!(),
        }
    }
}

impl<B: Backend> CacheUpdate<B> for ChannelPinsUpdate {
    fn process<'a>(
        &'a self,
        cache: &'a Cache<B>,
    ) -> Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(group) = cache.groups.get(self.channel_id).await? {
                return cache
                    .groups
                    .upsert(GroupEntity {
                        last_pin_timestamp: self.last_pin_timestamp.clone(),
                        ..group
                    })
                    .await;
            }

            if let Some(text_channel) = cache.text_channels.get(self.channel_id).await? {
                return cache
                    .text_channels
                    .upsert(TextChannelEntity {
                        last_pin_timestamp: self.last_pin_timestamp.clone(),
                        ..text_channel
                    })
                    .await;
            }

            if let Some(private_channel) = cache.private_channels.get(self.channel_id).await? {
                return cache
                    .private_channels
                    .upsert(PrivateChannelEntity {
                        last_pin_timestamp: self.last_pin_timestamp.clone(),
                        ..private_channel
                    })
                    .await;
            }

            Ok(())
        })
    }
}

impl<B: Backend> CacheUpdate<B> for ChannelUpdate {
    fn process<'a>(
        &'a self,
        cache: &'a Cache<B>,
    ) -> Pin<Box<dyn Future<Output = Result<(), B::Error>> + Send + 'a>> {
        match &self.0 {
            _ => unimplemented!(),
        }
    }
}
