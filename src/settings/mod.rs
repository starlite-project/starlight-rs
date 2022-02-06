mod global;
mod guild;
use starchart::{
	action::{ActionError, CreateTableAction, ReadEntryAction, UpdateEntryAction, CreateEntryAction},
	Action, IndexEntry, Starchart,
};
use futures_util::Future;
pub use self::{
	global::GlobalSettings,
	guild::{BlockedUser, GuildSettings, GuildTag},
};
use crate::{prelude::*, state::Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tables {
	Guilds,
	Global,
}

impl Tables {
	#[instrument(skip(context))]
	pub async fn init(context: Context) -> Result<(), ActionError> {
		Self::init_guilds(context).await?;
		Self::init_global(context).await
	}

	pub async fn get_entry<T: IndexEntry>(
		self,
		chart: &Starchart<YamlBackend>,
		key: &<T as IndexEntry>::Key,
	) -> Result<T>
	where
		<T as IndexEntry>::Key: Sync + Display,
	{
		let mut action: ReadEntryAction<T> = Action::new();
		let table = self.to_string();
		action.set_table(&table).set_key(key);

		action
			.run_read_entry(chart)
			.await
			.into_diagnostic()?
			.ok_or_else(|| error!("could not find entry with key {}", key))
	}

	pub async fn update_entry<T: IndexEntry>(
		self,
		chart: &Starchart<YamlBackend>,
		entry: &T,
	) -> Result<()> {
		let mut action: UpdateEntryAction<T> = Action::new();
		let table = self.to_string();
		action.set_table(&table).set_entry(entry);

		action.run_update_entry(chart).await.into_diagnostic()
	}

	pub async fn create_entry<T: IndexEntry>(self, chart: &Starchart<YamlBackend>, entry: &T) -> Result<()> {
		let mut action: CreateEntryAction<T> = Action::new();
		let table = self.to_string();
		action.set_table(&table).set_entry(entry);

		action.run_create_entry(chart).await.into_diagnostic()
	}

	async fn init_guilds(context: Context) -> Result<(), ActionError> {
		let default = GuildSettings::default();
		event!(Level::INFO, ?default, "creating table guilds");
		let mut action: CreateTableAction<GuildSettings> = Action::new();
		let guilds_table = Self::Guilds.to_string();
		action.set_table(&guilds_table);

		let chart = context.database();

		action.run_create_table(chart).await
	}

	async fn init_global(context: Context) -> Result<(), ActionError> {
		let default = GlobalSettings::default();

		event!(Level::INFO, ?default, "creating table global");
		let mut action: CreateTableAction<GlobalSettings> = Action::new();
		let global_table = Self::Global.to_string();
		action.set_table(&global_table);

		let chart = context.database();

		action.run_create_table(chart).await
	}
}

impl Display for Tables {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Guilds => f.write_str("guilds"),
			Self::Global => f.write_str("global"),
		}
	}
}
