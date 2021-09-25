use super::Describer;
use structsy::internal::{Description, FieldDescription, StructDescription};
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

macro_rules! impl_describer {
	($($args:ty;)*) => {
		$(
			impl Describer for $args {
				fn description() -> Description {
					Description::Struct(StructDescription::new(stringify!($args), &[]))
				}
			}
		)*
	};
	($($args:tt: $type:ty;)*) => {
		$(
			impl Describer for $args {
				fn description() -> Description {
					let field = FieldDescription::new::<$type>(0, &"id", None);
					Description::Struct(StructDescription::new(stringify!($args), &[field]))
				}
			}
		)*
	};
}

impl_describer! {
	u8;
	u16;
	u32;
	u64;
	u128;
	usize;
	i8;
	i16;
	i32;
	i64;
	i128;
	isize;
	bool;
	f32;
	f64;
}

impl_describer! {
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
