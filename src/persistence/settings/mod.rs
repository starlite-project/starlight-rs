mod guild;

use structsy::{Persistent, SRes, internal::PersistentEmbedded};
use persy::IndexType;

pub use self::guild::{GuildHelper, GuildKey, GuildSettings};

use super::Database;

pub trait Settings: Persistent {
	type Id: PersistentEmbedded + Copy;

	type RawId: IndexType + Copy;

	fn id(&self) -> (Self::Id, Self::RawId);
}

pub trait SettingsHelper<'db> {
	type Target: Settings;

	fn new(database: &'db Database) -> Self;

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
