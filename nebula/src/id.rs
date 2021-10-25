use serde::{Deserialize, Serialize};
use starchart::Key;
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	num::{NonZeroU64, ParseIntError},
	str::FromStr,
};
use thiserror::Error;
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

/// Error when converting from a [`u64`] to an [`Id`].
///
/// [`u64`]: prim@u64
#[derive(Debug, Default, Error, Clone, Copy)]
#[error("could not convert the u64 to an Id")]
pub struct ConvertError;

/// The Id struct for easily converting between different IDs, such as [`ApplicationId`] to [`UserId`].
///
/// [`ApplicationId`]: twilight_model::id::ApplicationId
/// [`UserId`]: twilight_model::id::UserId
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(clippy::unsafe_derive_deserialize)]
pub struct Id(pub NonZeroU64);

impl Id {
	/// Create a new [`Id`] from a Snowflake.
	#[must_use]
	pub const fn new(n: u64) -> Option<Self> {
		if let Some(n) = NonZeroU64::new(n) {
			Some(Self(n))
		} else {
			None
		}
	}

	/// Create a new [`Id`] from a Snowflake, without checking if the value is non-zero.
	///
	/// # Safety
	///
	/// The value must not be zero.
	#[must_use]
	pub const unsafe fn new_unchecked(n: u64) -> Self {
		Self(NonZeroU64::new_unchecked(n))
	}

	/// Get the underlying value of the ID.
	#[must_use]
	pub const fn get(self) -> u64 {
		self.0.get()
	}

	/// Turns the ID into an appropriate [`id`].
	///
	/// [`id`]: twilight_model::id
	#[must_use]
	pub fn as_id<T: private::Sealed + From<Self>>(self) -> T {
		T::from(self)
	}
}

impl Display for Id {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self.0, f)
	}
}

impl Key for Id {}

impl TryFrom<u64> for Id {
	type Error = ConvertError;

	fn try_from(value: u64) -> Result<Self, Self::Error> {
		Self::new(value).ok_or(ConvertError)
	}
}

impl From<NonZeroU64> for Id {
	fn from(value: NonZeroU64) -> Self {
		Self(value)
	}
}

impl From<Id> for NonZeroU64 {
	fn from(value: Id) -> Self {
		value.0
	}
}

impl FromStr for Id {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.parse::<NonZeroU64>().map(From::from)
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
