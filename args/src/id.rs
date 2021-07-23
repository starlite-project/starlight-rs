use self::private::Sealed;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    ops::Deref,
    str::FromStr,
};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Id<T: Sealed>(T);

impl<T: Sealed + Display> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<T: Sealed> Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Sealed + From<u64>> From<u64> for Id<T> {
    fn from(val: u64) -> Self {
        Self(T::from(val))
    }
}

impl<T: Sealed + From<u64>> FromStr for Id<T> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = s.parse()?;

        Ok(Self(T::from(val)))
    }
}

mod private {
    use twilight_model::id::{
        ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
        GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
    };
    pub trait Sealed {}

    macro_rules! impl_sealed {
        ($($args:ident;)*) => {
            $(
                impl Sealed for $args {}
            )*
        };
    }

    impl_sealed! {
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
}

#[cfg(test)]
mod tests {
    use super::Id;
    use std::num::ParseIntError;
    use twilight_model::id::GenericId;

    #[test]
    fn test_default() {
        let val: Id<GenericId> = Default::default();

        assert_eq!(val, Id(GenericId(0)));
    }

    #[test]
    fn parse() -> Result<(), ParseIntError> {
        let id = "10".parse::<Id<GenericId>>()?;

        assert_eq!(*id, GenericId(10));

        Ok(())
    }
}
