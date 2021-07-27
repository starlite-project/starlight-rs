use bitflags::bitflags;

bitflags! {
    pub struct ResourceType: u64 {
        const CHANNEL = 1;
        const EMOJI = 1 << 1;
        const GUILD = 1 << 2;
        const MEMBER = 1 << 3;
        const MESSAGE = 1 << 4;
        const PRESENCE = 1 << 5;
        const REACTION = 1 << 6;
        const ROLE = 1 << 7;
        const USER_CURRENT = 1 << 8;
        const USER = 1 << 9;
        const VOICE_STATE = 1 << 10;
        const STAGE_INSTANCE = 1 << 11;
        const INTEGRATION = 1 << 12;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub(super) resource_types: ResourceType,
    pub(super) message_cache_size: usize,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            resource_types: ResourceType::all(),
            message_cache_size: 100,
        }
    }

    pub const fn message_cache_size(&self) -> usize {
        self.message_cache_size
    }

    pub fn message_cache_size_mut(&mut self) -> &mut usize {
        &mut self.message_cache_size
    }

    pub const fn resource_types(&self) -> ResourceType {
        self.resource_types
    }

    pub fn resource_types_mut(&mut self) -> &mut ResourceType {
        &mut self.resource_types
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
