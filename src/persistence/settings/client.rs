use super::{Settings, SettingsHelper};
use crate::{persistence::Database, utils::CacheReliant};
use constella::DataTransformer;
use structsy::{Ref, SRes, StructsyIter, StructsyTx};
use structsy_derive::{queries, Persistent};
use twilight_cache_inmemory::ResourceType;
use twilight_model::id::UserId;

pub type ClientKey = DataTransformer<UserId>;

#[derive(Debug, Clone, Copy, PartialEq, Persistent)]
pub struct ClientSettings {
	#[index(mode = "exclusive")]
	raw_id: u64,
	pub id: ClientKey,
}

#[queries(ClientSettings)]
pub trait ClientQuery {
	fn by_id(self, id: ClientKey) -> Self;
}

impl ClientSettings {
	pub fn new(client_id: UserId) -> Self {
		let id = ClientKey::from(client_id);
		let raw_id = id.raw();

		Self { raw_id, id }
	}
}

impl Settings for ClientSettings {
	type Id = ClientKey;

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

	fn get(&self, id: ClientKey) -> Option<Self::Target> {
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

	fn create(&self, id: ClientKey) -> SRes<Self::Target> {
		let mut tx = self.database.begin()?;

		let client_settings = ClientSettings::new(id.value());
		tx.insert(&client_settings)?;

		tx.commit()?;

		Ok(client_settings)
	}
}
