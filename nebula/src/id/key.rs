use constella::DataTransformer;
use super::Id;
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

pub type IdKey = DataTransformer<Id>;

pub trait ToIdKey {
    fn to_id_key(self) -> IdKey;
}

macro_rules! impl_to_id_key {
    ($($args:ty;)*) => {
        $(
            impl ToIdKey for $args {
                fn to_id_key(self) -> IdKey {
                    IdKey::from(Id::from(self))
                }
            }
        )*
    }
}

impl_to_id_key! {
	ApplicationId;
	AttachmentId;
	AuditLogEntryId;
	ChannelId;
	CommandId;
	EmojiId;
	GenericId;
	GuildId;
	IntegrationId;
	InteractionId;
	MessageId;
	RoleId;
	StageId;
	UserId;
	WebhookId;
}