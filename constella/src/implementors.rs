use super::Transformer;
use structsy::PersistentEmbedded;
use twilight_model::id::{
	ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
	GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
};

macro_rules! impl_transformer_primitives {
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
    }
}

macro_rules! impl_transformer_id {
    ($($args:tt;)*) => {
        $(
            impl Transformer for $args {
                type DataType = u64;

                fn transform(&self) -> Self::DataType {
                    self.0
                }

                fn revert(value: &Self::DataType) -> Self {
                    Self(*value)
                }
            }
        )*
    }
}