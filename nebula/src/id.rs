use serde::{Deserialize, Serialize};
use std::{
	convert::AsRef,
	fmt::{Display, Formatter, Result as FmtResult},
	num::ParseIntError,
	ops::Deref,
	str::FromStr,
};
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

/// The Id struct for easily converting between different IDs, such as [`ApplicationId`] to [`UserId`].
///
/// [`ApplicationId`]: twilight_model::id::ApplicationId
/// [`UserId`]: twilight_model::id::UserId
#[derive(
	Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Id(pub u64);

impl Id {
	/// Create a new Id from a Snowflake.
	#[must_use]
	pub const fn new(value: u64) -> Self {
		Self(value)
	}

	/// Get the inner snowflake.
	#[must_use]
	pub const fn as_u64(self) -> u64 {
		self.0
	}

	/// Converts from an Id to a correct [`id`].
	///
	/// [`id`]: twilight_model::id
	#[must_use]
	pub fn as_id<T: private::Sealed + From<Self>>(self) -> T {
		T::from(self)
	}
}

impl AsRef<u64> for Id {
	fn as_ref(&self) -> &u64 {
		&self.0
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
