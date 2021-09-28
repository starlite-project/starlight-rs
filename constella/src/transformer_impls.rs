use super::Transformer;
use structsy::PersistentEmbedded;
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

macro_rules! impl_transformer {
	($($args:ty;)*) => {
		$(
			impl Transformer for $args {
				type DataType = Self;

				fn transform(&self) -> Self::DataType {
					*self
				}

				fn revert(value: &Self::DataType) -> Self {
					*value
				}
			}
		)*
	};
	($($args:tt: $type:ty;)*) => {
		$(
			impl Transformer for $args {
				type DataType = $type;

				fn transform(&self) -> Self::DataType {
					self.0
				}

				fn revert(value: &Self::DataType) -> Self {
					Self(*value)
				}
			}
		)*
	};
}

impl_transformer! {
	u8;
	u16;
	u32;
	u64;
	u128;
	i8;
	i16;
	i32;
	i64;
	i128;
	bool;
	f32;
	f64;
}

impl_transformer! {
	ApplicationId: u64;
	AttachmentId: u64;
	AuditLogEntryId: u64;
	ChannelId: u64;
	CommandId: u64;
	EmojiId: u64;
	GenericId: u64;
	GuildId: u64;
	IntegrationId: u64;
	InteractionId: u64;
	MessageId: u64;
	RoleId: u64;
	StageId: u64;
	UserId: u64;
	WebhookId: u64;
}

impl Transformer for String {
	type DataType = Self;

	fn transform(&self) -> Self::DataType {
		self.clone()
	}

	fn revert(value: &Self::DataType) -> Self {
		value.clone()
	}
}

#[cfg(target_pointer_width = "64")]
#[doc(cfg(target_pointer_width = "64"))]
impl Transformer for usize {
	type DataType = u64;

	fn transform(&self) -> Self::DataType {
		*self as u64
	}

	#[allow(clippy::cast_possible_truncation)]
	fn revert(value: &Self::DataType) -> Self {
		*value as Self
	}
}

#[cfg(target_pointer_width = "64")]
#[doc(cfg(target_pointer_width = "64"))]
impl Transformer for isize {
	type DataType = i64;

	fn transform(&self) -> Self::DataType {
		*self as i64
	}

	#[allow(clippy::cast_possible_truncation)]
	fn revert(value: &Self::DataType) -> Self {
		*value as Self
	}
}

impl<Dt: PersistentEmbedded, T: Transformer<DataType = Dt>> Transformer for Option<T> {
	type DataType = Option<Dt>;

	fn transform(&self) -> Self::DataType {
		self.as_ref().map(Transformer::transform)
	}

	fn revert(value: &Self::DataType) -> Self {
		value.as_ref().map(T::revert)
	}
}

impl<Dt: PersistentEmbedded, T: Transformer<DataType = Dt>> Transformer for Vec<T> {
	type DataType = Vec<Dt>;

	fn transform(&self) -> Self::DataType {
		self.iter().map(Transformer::transform).collect()
	}

	fn revert(value: &Self::DataType) -> Self {
		value.iter().map(T::revert).collect()
	}
}
