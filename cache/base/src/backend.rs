use crate::entity::{channel::{attachment::AttachmentRepository, category_channel::CategoryChannelRepository, group::GroupRepository, message::MessageRepository}, gateway::presence::PresenceRepository, user::{CurrentUserRepository, UserRepository}};

pub trait Backend: Send + Sync + Sized + 'static {
    type Error: Send + 'static;
    type AttachmentRepository: AttachmentRepository<Self> + Send + Sync;
    type CategoryChannelRepository: CategoryChannelRepository<Self> + Send + Sync;
    type CurrentUserRepository: CurrentUserRepository<Self> + Send + Sync;
    type GroupRepository: GroupRepository<Self> + Send + Sync;
    type MessageRepository: MessageRepository<Self> + Send + Sync;
    type PresenceRepository: PresenceRepository<Self> + Send + Sync;
    type UserRepository: UserRepository<Self> + Send + Sync;

    fn attachments(&self) -> Self::AttachmentRepository;

    fn category_channels(&self) -> Self::CategoryChannelRepository;

    fn current_user(&self) -> Self::CurrentUserRepository;

    fn groups(&self) -> Self::GroupRepository;

    fn messages(&self) -> Self::MessageRepository;

    fn presences(&self) -> Self::PresenceRepository;

    fn users(&self) -> Self::UserRepository;
}
