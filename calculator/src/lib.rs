mod bitops;
mod preset;

use self::preset::*;
use twilight_model::{
    channel::{
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
        ChannelType,
    },
    guild::Permissions,
    id::{GuildId, RoleId, UserId},
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "The member calculator isn't useful if you don't calculate permissions"]
pub struct PermissionCalculator<'a> {
    everyone_role: Permissions,
    guild_id: GuildId,
    member_roles: &'a [(RoleId, Permissions)],
    owner_id: UserId,
    user_id: UserId,
}

impl<'a> PermissionCalculator<'a> {
    #[must_use = "Calculators are only useful when used to calculate permissions"]
    pub const fn new(
        guild_id: GuildId,
        user_id: UserId,
        everyone_role: Permissions,
        member_roles: &'a [(RoleId, Permissions)],
    ) -> Self {
        Self {
            everyone_role,
            guild_id,
            owner_id: UserId(0),
            member_roles,
            user_id,
        }
    }

    #[must_use = "Calculators are only useful when used to calculate permissions"]
    pub const fn owner_id(mut self, owner_id: UserId) -> Self {
        self.owner_id = owner_id;

        self
    }

    pub const fn root(&self) -> Permissions {
        if self.owner_id.0 == self.user_id.0 {
            return Permissions::all();
        }

        if self.everyone_role.contains(Permissions::ADMINISTRATOR) {
            return Permissions::all();
        }

        let mut permissions = self.everyone_role;

        let member_role_count = self.member_roles.len();
        let mut idx = 0;

        while idx < member_role_count {
            let (_, role_permissions) = self.member_roles[idx];
            if role_permissions.contains(Permissions::ADMINISTRATOR) {
                return Permissions::all();
            }

            permissions = bitops::insert(permissions, role_permissions);
            idx += 1;
        }

        permissions
    }

    pub const fn in_channel(
        self,
        channel_type: ChannelType,
        channel_overwrites: &[PermissionOverwrite],
    ) -> Permissions {
        let mut permissions = self.root();

        if permissions.contains(Permissions::ADMINISTRATOR) {
            return Permissions::all();
        }

        let mut member_allow = Permissions::empty();
        let mut member_deny = Permissions::empty();
        let mut roles_allow = Permissions::empty();
        let mut roles_deny = Permissions::empty();

        let channel_overwrite_len = channel_overwrites.len();
        let mut idx = 0;

        while idx < channel_overwrite_len {
            let overwrite = &channel_overwrites[idx];
            match overwrite.kind {
                PermissionOverwriteType::Role(role) => {
                    if role.0 == self.guild_id.0 {
                        permissions = bitops::remove(permissions, overwrite.deny);
                        permissions = bitops::insert(permissions, overwrite.allow);

                        idx += 1;
                        continue;
                    }

                    if !member_has_role(self.member_roles, role) {
                        idx += 1;

                        continue;
                    }

                    roles_allow = bitops::insert(roles_allow, overwrite.allow);
                    roles_deny = bitops::insert(roles_deny, overwrite.deny);
                }
                PermissionOverwriteType::Member(user_id) if user_id.0 == self.user_id.0 => {
                    member_allow = bitops::insert(member_allow, overwrite.allow);
                    member_deny = bitops::insert(member_deny, overwrite.deny);
                }
                PermissionOverwriteType::Member(_) => {}
            }

            idx += 1;
        }

        let role_view_channel_denied = roles_deny.contains(Permissions::VIEW_CHANNEL)
            && !roles_allow.contains(Permissions::VIEW_CHANNEL);

        let member_view_channel_denied = member_deny.contains(Permissions::VIEW_CHANNEL)
            && !member_allow.contains(Permissions::VIEW_CHANNEL);

        if member_view_channel_denied || role_view_channel_denied {
            return Permissions::empty();
        }

        let role_send_messages_denied = roles_deny.contains(Permissions::SEND_MESSAGES)
            && !roles_allow.contains(Permissions::SEND_MESSAGES);

        let member_send_messages_denied = member_deny.contains(Permissions::VIEW_CHANNEL)
            && !member_allow.contains(Permissions::SEND_MESSAGES);

        if member_send_messages_denied || role_send_messages_denied {
            member_allow = bitops::remove(member_allow, PERMISSIONS_MESSAGING);
            roles_allow = bitops::remove(roles_allow, PERMISSIONS_MESSAGING);
            permissions = bitops::remove(permissions, PERMISSIONS_MESSAGING);
        }

        permissions = bitops::remove(permissions, roles_deny);
        permissions = bitops::insert(permissions, roles_allow);
        permissions = bitops::remove(permissions, member_deny);
        permissions = bitops::insert(permissions, member_allow);

        permissions = bitops::remove(permissions, PERMISSIONS_ROOT_ONLY);

        if !matches!(channel_type, ChannelType::GuildStageVoice) {
            permissions = bitops::remove(permissions, PERMISSIONS_STAGE_ONLY);
        }

        if !matches!(channel_type, ChannelType::GuildText) {
            permissions = bitops::remove(permissions, PERMISSIONS_TEXT_ONLY);
        }

        if !matches!(channel_type, ChannelType::GuildVoice) {
            permissions = bitops::remove(permissions, PERMISSIONS_VOICE_ONLY);
        }

        permissions
    }
}

const fn member_has_role(member_roles: &[(RoleId, Permissions)], role_id: RoleId) -> bool {
    let len = member_roles.len();
    let mut idx = 0;

    while idx < len {
        let (iter_role_id, _) = member_roles[idx];
        if iter_role_id.0 == role_id.0 {
            return true;
        }

        idx += 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{GuildId, PermissionCalculator, RoleId, UserId};
    use twilight_model::{
        channel::{
            permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
            ChannelType,
        },
        guild::Permissions,
    };

    const GUILD_ID: GuildId = GuildId(1);
    const USER_ID: UserId = UserId(2);

    #[test]
    fn owner_is_admin() {
        let member_roles = &[];

        let calculator =
            PermissionCalculator::new(GUILD_ID, USER_ID, Permissions::SEND_MESSAGES, member_roles)
                .owner_id(USER_ID);

        assert_eq!(Permissions::all(), calculator.root());
    }

    #[test]
    fn view_channel_deny_implicit() {
        let everyone_role = Permissions::MENTION_EVERYONE | Permissions::SEND_MESSAGES;
        let member_roles = &[(RoleId(3), Permissions::empty())];
        {
            let overwrites = &[PermissionOverwrite {
                allow: Permissions::SEND_TTS_MESSAGES,
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(RoleId(3)),
            }];

            let calculated =
                PermissionCalculator::new(GUILD_ID, USER_ID, everyone_role, member_roles)
                    .in_channel(ChannelType::GuildText, overwrites);

            assert_eq!(calculated, Permissions::empty());
        }

        {
            let overwrites = &[PermissionOverwrite {
                allow: Permissions::SEND_TTS_MESSAGES,
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Member(UserId(2)),
            }];

            let calculated =
                PermissionCalculator::new(GUILD_ID, USER_ID, everyone_role, member_roles)
                    .in_channel(ChannelType::GuildText, overwrites);

            assert_eq!(calculated, Permissions::empty());
        }
    }

    #[test]
    fn remove_text_and_stage_perms_when_voice() {
        let everyone_role = Permissions::CONNECT;
        let member_roles = &[(RoleId(3), Permissions::SEND_MESSAGES)];
        let calculated = PermissionCalculator::new(GUILD_ID, USER_ID, everyone_role, member_roles)
            .in_channel(ChannelType::GuildVoice, &[]);

        assert_eq!(calculated, Permissions::CONNECT);
    }

    #[test]
    fn remove_audio_perms_when_text() {
        let everyone_role = Permissions::CONNECT;
        let member_roles = &[(RoleId(3), Permissions::SEND_MESSAGES)];

        let calculated = PermissionCalculator::new(GUILD_ID, USER_ID, everyone_role, member_roles)
            .in_channel(ChannelType::GuildText, &[]);

        assert_eq!(
            calculated,
            Permissions::CONNECT | Permissions::SEND_MESSAGES
        );
    }

    #[test]
    fn deny_send_messages_removes_related() {
        let everyone_role =
            Permissions::MANAGE_MESSAGES | Permissions::EMBED_LINKS | Permissions::MENTION_EVERYONE;
        let member_roles = &[(RoleId(3), Permissions::empty())];

        let overwrites = &[PermissionOverwrite {
            allow: Permissions::ATTACH_FILES,
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Role(RoleId(3)),
        }];

        let calculated = PermissionCalculator::new(GUILD_ID, USER_ID, everyone_role, member_roles)
            .in_channel(ChannelType::GuildText, overwrites);

        assert_eq!(calculated, Permissions::MANAGE_MESSAGES);
    }

    #[test]
    fn admin() {
        let calc = PermissionCalculator::new(
            GUILD_ID,
            USER_ID,
            Permissions::empty(),
            &[(RoleId(3), Permissions::ADMINISTRATOR)],
        );

        assert!(calc.root().is_all());

        let perms = calc.in_channel(ChannelType::GuildText, Default::default());
        assert!(perms.is_all());
    }
}
