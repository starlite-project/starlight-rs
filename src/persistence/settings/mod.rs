mod guild;

use structsy::{internal::PersistentEmbedded, Persistent, SRes};

pub use self::guild::{GuildHelper, GuildKey, GuildSettings};

pub trait Settings: Persistent {
	type Id: PersistentEmbedded + Copy;
}

pub trait SettingsHelper {
	type Target: Settings;

	fn get(&self, id: <Self::Target as Settings>::Id) -> Option<Self::Target>;

	fn create(&self, id: <Self::Target as Settings>::Id) -> SRes<Self::Target>;

	fn acquire(&self, id: <Self::Target as Settings>::Id) -> SRes<Self::Target> {
		if let Some(existing) = self.get(id) {
			Ok(existing)
		} else {
			self.create(id)
		}
	}
}
