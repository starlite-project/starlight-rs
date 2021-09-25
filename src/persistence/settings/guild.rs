use super::{IdKey, Settings, SettingsHelper};
use crate::{persistence::Database, utils::CacheReliant};
use nebula::Id;
use structsy::{Ref, SRes, StructsyIter, StructsyTx};
use structsy_derive::{queries, Persistent};
use twilight_cache_inmemory::ResourceType;
use twilight_model::id::GuildId;

#[derive(Debug, Clone, Copy, PartialEq, Persistent)]
pub struct GuildSettings {
	#[index(mode = "exclusive")]
	raw_id: u64,
	pub id: IdKey,
}

#[queries(GuildSettings)]
pub trait GuildQuery {
	fn by_id(self, id: IdKey) -> Self;
}

impl GuildSettings {
	#[must_use]
	pub fn new(guild_id: GuildId) -> Self {
		let id = IdKey::from(Id::from(guild_id));
		let raw_id = id.raw();

		Self { raw_id, id }
	}
}

impl Settings for GuildSettings {
	type Id = IdKey;

	type RawId = u64;

	fn id(&self) -> (Self::Id, Self::RawId) {
		(self.id, self.raw_id)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct GuildHelper<'db> {
	database: &'db Database,
}

impl CacheReliant for GuildHelper<'_> {
	fn needs() -> ResourceType {
		ResourceType::GUILD
	}
}

impl<'db> SettingsHelper<'db> for GuildHelper<'db> {
	type Target = GuildSettings;

	fn new(database: &'db Database) -> Self {
		Self { database }
	}

	fn database(&self) -> &Database {
		self.database
	}

	fn get(&self, id: IdKey) -> Option<Self::Target> {
		let query = self.database.query::<Self::Target>();

		let iter: StructsyIter<'static, (Ref<GuildSettings>, GuildSettings)> =
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

		let guild_settings = GuildSettings::new(id.value().into());
		tx.insert(&guild_settings)?;

		tx.commit()?;

		Ok(guild_settings)
	}
}
