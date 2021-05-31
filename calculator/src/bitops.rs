use twilight_model::guild::Permissions;

pub const fn insert(permissions: Permissions, other: Permissions) -> Permissions {
    Permissions::from_bits_truncate(permissions.bits() | other.bits())
}

pub const fn remove(permissions: Permissions, other: Permissions) -> Permissions {
    Permissions::from_bits_truncate(permissions.bits() & !other.bits())
}

#[cfg(test)]
mod tests {
    use twilight_model::guild::Permissions;

    #[test]
    fn insert() {
        let actual = super::insert(
            Permissions::KICK_MEMBERS,
            Permissions::BAN_MEMBERS | Permissions::CONNECT,
        );

        let expected = Permissions::BAN_MEMBERS | Permissions::CONNECT | Permissions::KICK_MEMBERS;

        assert_eq!(actual, expected);
    }

    #[test]
    fn remove() {
        let actual = super::remove(
            Permissions::BAN_MEMBERS | Permissions::KICK_MEMBERS,
            Permissions::BAN_MEMBERS,
        );

        assert_eq!(actual, Permissions::KICK_MEMBERS);
    }

    #[test]
    fn remove_nonexistent() {
        let actual = super::remove(Permissions::KICK_MEMBERS, Permissions::BAN_MEMBERS);

        assert_eq!(actual, Permissions::KICK_MEMBERS);
    }
}
