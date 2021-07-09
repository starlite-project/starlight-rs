use crate::entity::{channel::{attachment::AttachmentRepository, category_channel::CategoryChannelRepository}, user::CurrentUserRepository};

pub trait Backend: Send + Sync + Sized + 'static {
    type Error: Send + 'static;
    type AttachmentRepository: AttachmentRepository<Self> + Send + Sync;
    type CategoryChannelRepository: CategoryChannelRepository<Self> + Send + Sync;
    type CurrentUserRepository: CurrentUserRepository<Self> + Send + Sync;
}