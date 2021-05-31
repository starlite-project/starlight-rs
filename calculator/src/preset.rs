use super::bitops;
use twilight_model::guild::Permissions;

pub const PERMISSIONS_MESSAGING: Permissions = Permissions::from_bits_truncate(
    Permissions::ATTACH_FILES.bits()
        | Permissions::EMBED_LINKS.bits()
        | Permissions::MENTION_EVERYONE.bits()
        | Permissions::SEND_TTS_MESSAGES.bits(),
);

pub const PERMISSIONS_ROOT_ONLY: Permissions = Permissions::from_bits_truncate(
    Permissions::ADMINISTRATOR.bits()
        | Permissions::BAN_MEMBERS.bits()
        | Permissions::CHANGE_NICKNAME.bits()
        | Permissions::KICK_MEMBERS.bits()
        | Permissions::MANAGE_EMOJIS.bits()
        | Permissions::MANAGE_GUILD.bits()
        | Permissions::MANAGE_NICKNAMES.bits()
        | Permissions::VIEW_AUDIT_LOG.bits()
        | Permissions::VIEW_GUILD_INSIGHTS.bits(),
);

pub const PERMISSIONS_STAGE_ONLY: Permissions = bitops::remove(
    bitops::remove(PERMISSIONS_STAGE, PERMISSIONS_TEXT),
    PERMISSIONS_VOICE,
);

pub const PERMISSIONS_TEXT_ONLY: Permissions = bitops::remove(
    bitops::remove(PERMISSIONS_TEXT, PERMISSIONS_STAGE),
    PERMISSIONS_VOICE,
);

pub const PERMISSIONS_VOICE_ONLY: Permissions = bitops::remove(
    bitops::remove(PERMISSIONS_VOICE, PERMISSIONS_STAGE),
    PERMISSIONS_TEXT,
);

const PERMISSIONS_STAGE: Permissions = Permissions::from_bits_truncate(
    Permissions::CONNECT.bits()
        | Permissions::CREATE_INVITE.bits()
        | Permissions::MANAGE_CHANNELS.bits()
        | Permissions::MANAGE_ROLES.bits()
        | Permissions::MOVE_MEMBERS.bits()
        | Permissions::MUTE_MEMBERS.bits()
        | Permissions::REQUEST_TO_SPEAK.bits()
        | Permissions::VIEW_CHANNEL.bits(),
);

const PERMISSIONS_TEXT: Permissions = Permissions::from_bits_truncate(
    Permissions::ADD_REACTIONS.bits()
        | Permissions::ATTACH_FILES.bits()
        | Permissions::CREATE_INVITE.bits()
        | Permissions::EMBED_LINKS.bits()
        | Permissions::MANAGE_CHANNELS.bits()
        | Permissions::MANAGE_MESSAGES.bits()
        | Permissions::MANAGE_ROLES.bits()
        | Permissions::MANAGE_WEBHOOKS.bits()
        | Permissions::MENTION_EVERYONE.bits()
        | Permissions::READ_MESSAGE_HISTORY.bits()
        | Permissions::SEND_MESSAGES.bits()
        | Permissions::SEND_TTS_MESSAGES.bits()
        | Permissions::USE_EXTERNAL_EMOJIS.bits()
        | Permissions::USE_SLASH_COMMANDS.bits()
        | Permissions::VIEW_CHANNEL.bits(),
);

const PERMISSIONS_VOICE: Permissions = Permissions::from_bits_truncate(
    Permissions::CONNECT.bits()
        | Permissions::CREATE_INVITE.bits()
        | Permissions::DEAFEN_MEMBERS.bits()
        | Permissions::MANAGE_CHANNELS.bits()
        | Permissions::MANAGE_ROLES.bits()
        | Permissions::MOVE_MEMBERS.bits()
        | Permissions::MUTE_MEMBERS.bits()
        | Permissions::PRIORITY_SPEAKER.bits()
        | Permissions::SPEAK.bits()
        | Permissions::STREAM.bits()
        | Permissions::USE_VAD.bits()
        | Permissions::VIEW_CHANNEL.bits(),
);

#[cfg(test)]
mod tests {
    use super::{PERMISSIONS_STAGE_ONLY, PERMISSIONS_TEXT_ONLY, PERMISSIONS_VOICE_ONLY};
    use twilight_model::guild::Permissions;

    #[test]
    fn permissions_stage_only() {
        assert_eq!(Permissions::REQUEST_TO_SPEAK, PERMISSIONS_STAGE_ONLY);
    }

    #[test]
    fn permissions_text_only() {
        let expected = Permissions::ADD_REACTIONS
            | Permissions::ATTACH_FILES
            | Permissions::EMBED_LINKS
            | Permissions::MANAGE_MESSAGES
            | Permissions::MANAGE_WEBHOOKS
            | Permissions::MENTION_EVERYONE
            | Permissions::READ_MESSAGE_HISTORY
            | Permissions::SEND_MESSAGES
            | Permissions::SEND_TTS_MESSAGES
            | Permissions::USE_EXTERNAL_EMOJIS
            | Permissions::USE_SLASH_COMMANDS;

        assert_eq!(expected, PERMISSIONS_TEXT_ONLY);
    }

    #[test]
    fn permissions_voice_only() {
        let expected = Permissions::DEAFEN_MEMBERS
            | Permissions::PRIORITY_SPEAKER
            | Permissions::SPEAK
            | Permissions::STREAM
            | Permissions::USE_VAD;

        assert_eq!(expected, PERMISSIONS_VOICE_ONLY);
    }
}
