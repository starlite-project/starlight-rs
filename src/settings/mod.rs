mod guild;
use starchart::{
	action::{ActionError, CreateTableAction, ReadEntryAction, UpdateEntryAction},
	Action, IndexEntry, Starchart,
};

pub use self::guild::{GuildSettings, GuildTag};
use crate::{prelude::*, state::Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tables {
	Guilds,
}

impl Tables {
	#[instrument(skip(context))]
	pub async fn init(context: Context) -> Result<(), ActionError> {
		Self::init_guilds(context).await?;
		Ok(())
	}

	pub async fn get_entry<T: IndexEntry>(
		self,
		chart: &Starchart<TomlBackend>,
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
		chart: &Starchart<TomlBackend>,
		entry: &T,
	) -> Result<()> {
		let mut action: UpdateEntryAction<T> = Action::new();
		let table = self.to_string();
		action.set_table(&table).set_entry(entry);

		action.run_update_entry(chart).await.into_diagnostic()
	}

	async fn init_guilds(context: Context) -> Result<(), ActionError> {
		let default = GuildSettings::default();
		event!(Level::INFO, ?default, "creating table guilds");
		let mut action: CreateTableAction<GuildSettings> = Action::new();
		let guilds_table = Self::Guilds.to_string();
		action.set_table(&guilds_table);

		let chart = context.database();

		action.run_create_table(chart).await?;

		Ok(())
	}
}

impl Display for Tables {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Guilds => f.write_str("guilds"),
		}
	}
}
