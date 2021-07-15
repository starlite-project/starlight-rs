use bitflags::bitflags;

bitflags! {
    pub struct EntityType: u64 {
        const ATTACHMENT = 1 << 0;
        const CHANNEL_CATEGORY = 1 << 1;
        const CHANNEL_GROUP = 1 << 2;
        const CHANNEL_PRIVATE = 1 << 3;
        const CHANNEL_TEXT = 1 << 4;
        const CHANNEL_STAGE = 1 << 5;
        const CHANNEL_VOICE = 1 << 6;
        const EMOJI = 1 << 7;
        const GUILD = 1 << 8;
        const MEMBER = 1 << 9;
        const MESSAGE = 1 << 10;
        const PRESENCE = 1 << 11;
        const ROLE = 1 << 12;
        const USER = 1 << 13;
        const USER_CURRENT = 1 << 14;
        const VOICE_STATE = 1 << 15;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    entity_types: EntityType,
    message_cache_size: usize,
}

impl Config {
    pub const fn entity_types(&self) -> EntityType {
        self.entity_types
    }

    pub fn entity_types_mut(&mut self) -> &mut EntityType {
        &mut self.entity_types
    }

    pub const fn message_cache_size(&self) -> usize {
        self.message_cache_size
    }

    pub fn message_cache_size_mut(&mut self) -> &mut usize {
        &mut self.message_cache_size
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            entity_types: EntityType::all(),
            message_cache_size: 100,
        }
    }
}
