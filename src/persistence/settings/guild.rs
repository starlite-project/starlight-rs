use super::{Settings, SettingsHelper};
use crate::persistence::Database;
use constella::DataTransformer;
use structsy::{Ref, SRes, StructsyIter, StructsyTx};
use structsy_derive::{queries, Persistent};
use twilight_model::id::GuildId;

pub type GuildKey = DataTransformer<GuildId>;

#[derive(Debug, Clone, Copy, PartialEq, Persistent)]
pub struct GuildSettings {
	#[index(mode = "exclusive")]
	raw_id: u64,
	pub id: GuildKey,
}

#[queries(GuildSettings)]
pub trait GuildQuery {
	fn by_id(self, id: GuildKey) -> Self;
}

impl GuildSettings {
	pub fn new(guild_id: GuildId) -> Self {
		let id = GuildKey::from(guild_id);
		let raw_id = id.raw();

		Self { raw_id, id }
	}
}

impl Settings for GuildSettings {}

#[derive(Debug, Clone, Copy)]
pub struct GuildHelper<'db> {
	db: &'db Database,
}

impl<'db> GuildHelper<'db> {
	pub const fn new(db: &'db Database) -> Self {
		Self { db }
	}
}

impl SettingsHelper for GuildHelper<'_> {
	type Target = GuildSettings;

	type Id = GuildKey;

	fn get(&self, id: &Self::Id) -> Option<Self::Target> {
		let query = self.db.query::<Self::Target>();

		let iter: StructsyIter<'static, (Ref<GuildSettings>, GuildSettings)> =
			query.by_id(*id).into_iter();

		for (_, settings) in iter {
			if settings.raw_id != id.raw() {
				continue;
			} else {
				return Some(settings);
			}
		}

		None
	}

	fn create(&self, id: &Self::Id) -> SRes<Self::Target> {
		let mut tx = self.db.begin()?;

		let guild_settings = GuildSettings::new(id.value());
		tx.insert(&guild_settings)?;

		tx.commit()?;

		Ok(guild_settings)
	}
}
