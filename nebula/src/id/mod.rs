use constella::{
	external::{Description, FieldDescription, StructDescription},
	Describer, Transformer,
};
use serde::{Deserialize, Serialize};
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	num::ParseIntError,
	ops::Deref,
	str::FromStr,
};
use twilight_model::{application::interaction::Interaction, channel::{Channel, GuildChannel}, id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
}};

mod key;

pub use self::key::{IdKey, ToIdKey};

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

	#[must_use]
	pub fn as_id<T: private::Sealed + From<Self>>(self) -> T {
		T::from(self)
	}

	#[must_use]
	pub fn as_id_key<T: private::Sealed + Into<Self>>(id: T) -> IdKey {
		IdKey::from(id.into())
	}
}

impl Deref for Id {
	type Target = u64;

	fn deref(&self) -> &Self::Target {
		&self.0
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

impl From<u64> for Id {
	fn from(value: u64) -> Self {
		Self::new(value)
	}
}

impl From<Id> for u64 {
	fn from(value: Id) -> Self {
		value.0
	}
}

impl FromStr for Id {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.parse::<u64>().map(From::from)
	}
}

impl Transformer for Id {
	type DataType = u64;

	fn transform(&self) -> Self::DataType {
		self.0
	}

	fn revert(value: &Self::DataType) -> Self {
		Self(*value)
	}
}

macro_rules! impl_id {
    ($($args:tt;)*) => {
        $(
            impl From<$args> for Id {
                fn from(id: $args) -> Self {
                    Self(id.0)
                }
            }

            impl From<Id> for $args {
                fn from(id: Id) -> Self {
                    Self(id.0)
                }
            }
        )*
    }
}

impl_id! {
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

mod private {
	use twilight_model::id::{
		ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
		GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
	};

	pub trait Sealed {}

	impl Sealed for ApplicationId {}
	impl Sealed for AttachmentId {}
	impl Sealed for AuditLogEntryId {}
	impl Sealed for ChannelId {}
	impl Sealed for CommandId {}
	impl Sealed for EmojiId {}
	impl Sealed for GenericId {}
	impl Sealed for GuildId {}
	impl Sealed for IntegrationId {}
	impl Sealed for InteractionId {}
	impl Sealed for MessageId {}
	impl Sealed for RoleId {}
	impl Sealed for StageId {}
	impl Sealed for UserId {}
	impl Sealed for WebhookId {}
}
