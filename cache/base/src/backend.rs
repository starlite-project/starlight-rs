use crate::entity::{
    channel::{
        AttachmentRepository, CategoryChannelRepository, GroupRepository, MessageRepository,
        PrivateChannelRepository, TextChannelRepository, VoiceChannelRepository,
    },
    gateway::PresenceRepository,
    guild::{EmojiRepository, GuildRepository, MemberRepository, RoleRepository},
    user::{CurrentUserRepository, UserRepository},
    voice::VoiceStateRepository,
};

pub trait Backend: Send + Sync + Sized + 'static {
    type Error: Send + 'static;
    type AttachmentRepository: AttachmentRepository<Self> + Send + Sync;
    type CategoryChannelRepository: CategoryChannelRepository<Self> + Send + Sync;
    type CurrentUserRepository: CurrentUserRepository<Self> + Send + Sync;
    type EmojiRepository: EmojiRepository<Self> + Send + Sync;
    type GroupRepository: GroupRepository<Self> + Send + Sync;
    type GuildRepository: GuildRepository<Self> + Send + Sync;
    type MemberRepository: MemberRepository<Self> + Send + Sync;
    type MessageRepository: MessageRepository<Self> + Send + Sync;
    type PresenceRepository: PresenceRepository<Self> + Send + Sync;
    type PrivateChannelRepository: PrivateChannelRepository<Self> + Send + Sync;
    type RoleRepository: RoleRepository<Self> + Send + Sync;
    type TextChannelRepository: TextChannelRepository<Self> + Send + Sync;
    type UserRepository: UserRepository<Self> + Send + Sync;
    type VoiceChannelRepository: VoiceChannelRepository<Self> + Send + Sync;
    type VoiceStateRepository: VoiceStateRepository<Self> + Send + Sync;

    fn attachments(&self) -> Self::AttachmentRepository;

    fn category_channels(&self) -> Self::CategoryChannelRepository;

    fn current_user(&self) -> Self::CurrentUserRepository;

    fn emojis(&self) -> Self::EmojiRepository;

    fn guilds(&self) -> Self::GuildRepository;

    fn groups(&self) -> Self::GroupRepository;

    fn members(&self) -> Self::MemberRepository;

    fn messages(&self) -> Self::MessageRepository;

    fn presences(&self) -> Self::PresenceRepository;

    fn private_channels(&self) -> Self::PrivateChannelRepository;

    fn roles(&self) -> Self::RoleRepository;

    fn text_channels(&self) -> Self::TextChannelRepository;

    fn users(&self) -> Self::UserRepository;

    fn voice_channels(&self) -> Self::VoiceChannelRepository;

    fn voice_states(&self) -> Self::VoiceStateRepository;
}
