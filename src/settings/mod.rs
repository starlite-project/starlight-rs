mod guild;
use starchart::{
	action::{ActionError, CreateTableAction},
	Action,
};

pub use self::guild::{GuildSettings, GuildTag};
use crate::{prelude::*, state::Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tables {
	Guilds,
}

impl Display for Tables {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Guilds => f.write_str("guilds"),
		}
	}
}

// custom function to initialize all tables.
#[instrument(skip(context))]
pub async fn init_tables(context: Context) -> Result<(), ActionError> {
	let default = GuildSettings::default();
	event!(Level::INFO, ?default, "creating table guilds");
	let mut action: CreateTableAction<GuildSettings> = Action::new();
	action.set_table(Tables::Guilds.to_string());

	let chart = context.database();

	action.run_create_table(chart).await?;

	Ok(())
}
