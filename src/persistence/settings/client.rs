use super::{IdKey, Settings, SettingsHelper};
use crate::{persistence::Database, utils::CacheReliant};
use nebula::Id;
use structsy::{Ref, SRes, StructsyIter, StructsyTx};
use structsy_derive::{queries, Persistent};
use twilight_cache_inmemory::ResourceType;
use twilight_model::id::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Persistent)]
pub struct ClientSettings {
	#[index(mode = "exclusive")]
	raw_id: u64,
	pub id: IdKey,
}

#[queries(ClientSettings)]
pub trait ClientQuery {
	fn by_id(self, id: IdKey) -> Self;
}

impl ClientSettings {
	#[must_use]
	pub fn new(client_id: UserId) -> Self {
		let id = IdKey::from(Id::from(client_id));
		let raw_id = id.raw();

		Self { raw_id, id }
	}
}

impl Settings for ClientSettings {
	type Id = IdKey;

	type RawId = u64;

	fn id(&self) -> (Self::Id, Self::RawId) {
		(self.id, self.raw_id)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ClientHelper<'db> {
	database: &'db Database,
}

impl CacheReliant for ClientHelper<'_> {
	fn needs() -> ResourceType {
		ResourceType::USER_CURRENT
	}
}

impl<'db> SettingsHelper<'db> for ClientHelper<'db> {
	type Target = ClientSettings;

	fn new(database: &'db Database) -> Self {
		Self { database }
	}

	fn database(&self) -> &Database {
		self.database
	}

	fn get(&self, id: IdKey) -> Option<Self::Target> {
		let query = self.database.query::<Self::Target>();

		let iter: StructsyIter<'static, (Ref<ClientSettings>, ClientSettings)> =
			query.by_id(id).into_iter();

		for (_, settings) in iter {
			if settings.raw_id == id.raw() {
				return Some(settings);
			}
		}

		None
	}

	fn create(&self, id: IdKey) -> SRes<Self::Target> {
		let mut tx = self.database.begin()?;

		let client_settings = ClientSettings::new(id.value().into());
		tx.insert(&client_settings)?;

		tx.commit()?;

		Ok(client_settings)
	}
}
