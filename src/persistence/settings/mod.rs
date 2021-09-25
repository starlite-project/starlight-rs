mod client;
mod guild;

use super::Database;
use constella::DataTransformer;
use nebula::Id;
use persy::IndexType;
use structsy::{internal::PersistentEmbedded, Persistent, SRes, StructsyQuery};

pub use self::{
	client::{ClientHelper, ClientSettings},
	guild::{GuildHelper, GuildSettings},
};

pub type IdKey = DataTransformer<Id>;

pub trait Settings: Persistent {
	type Id: PersistentEmbedded + Copy;

	type RawId: IndexType + Copy;

	fn id(&self) -> (Self::Id, Self::RawId);
}

pub trait SettingsHelper<'db> {
	type Target: Settings;

	fn new(database: &'db Database) -> Self;

	fn database(&self) -> &Database;

	fn get(&self, id: <Self::Target as Settings>::Id) -> Option<Self::Target>;

	fn create(&self, id: <Self::Target as Settings>::Id) -> SRes<Self::Target>;

	fn acquire(&self, id: <Self::Target as Settings>::Id) -> SRes<Self::Target> {
		self.get(id).map_or_else(|| self.create(id), Ok)
	}

	fn query(&self) -> StructsyQuery<Self::Target> {
		self.database().query()
	}
}
