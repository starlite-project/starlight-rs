use super::Describer;
use structsy::internal::{Description, FieldDescription, StructDescription};
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

macro_rules! impl_describer_primitives {
    ($($args:ty;)*) => {
        $(
            impl Describer for $args {
                fn description() -> Description {
                   Description::Struct(StructDescription::new(stringify!($args), &[]))
                }
            }
        )*
    }
}

macro_rules! impl_describer_id {
    ($($args:tt;)*) => {
        $(
            impl Describer for $args {
                fn description() -> Description {
                    let field = FieldDescription::new::<u32>(0, &"id", None);
                    Description::Struct(StructDescription::new(stringify!($args), &[field]))
                }
            }
        )*
    }
}

impl_describer_primitives! {
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

impl_describer_id! {
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
