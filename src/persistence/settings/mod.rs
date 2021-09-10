mod guild;

use structsy::{Persistent, SRes};

pub use self::guild::{GuildKey, GuildSettings, GuildHelper};

pub trait Settings: Persistent {}

pub trait SettingsHelper {
    type Target: Settings;
    type Id;

    fn get(&self, id: &Self::Id) -> Option<Self::Target>;

    fn create(&self, id: &Self::Id) -> SRes<Self::Target>;

    fn acquire(&self, id: &Self::Id) -> SRes<Self::Target> {
        // self.get(id).unwrap_or_else(|| self.create(id))
        if let Some(existing) = self.get(id) {
            Ok(existing)
        } else {
            self.create(id)
        }
    }
}