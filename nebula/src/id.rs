use serde::{Deserialize, Serialize};
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};
use std::{fmt::{Display, Formatter, Result as FmtResult}, ops::{Deref, DerefMut}};
use constella::{Describer, Transformer};
use structsy::internal::{Description, FieldDescription, StructDescription};

#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Id(pub u64);

impl Id {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl Deref for Id {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl Describer for Id {
    fn description() -> Description {
        let field = FieldDescription::new::<u64>(0, "id", None);
        Description::Struct(StructDescription::new("Id", &[field]))
    }
}

macro_rules! impl_from_id {
    ($($args:tt;)*) => {
        $(
            impl From<$args> for Id {
                fn from(id: $args) -> Self {
                    Self(id.0)
                }
            }
        )*
    }
}

macro_rules! impl_id_from {
    ($($args:tt;)*) => {
        $(
            impl From<Id> for $args {
                fn from(id: Id) -> Self {
                    Self(id.0)
                }
            }
        )*
    }
}

impl_from_id! {
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

impl_id_from! {
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