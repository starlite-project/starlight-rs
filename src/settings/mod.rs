mod guild;
use starchart::{
	action::{ActionError, CreateTableAction},
	Action,
};

pub use self::guild::{GuildSettings, GuildTag};
use crate::{prelude::*, state::Context};

// custom function to initialize all tables.
#[instrument(skip(context))]
pub async fn init_tables(context: Context) -> Result<(), ActionError> {
	let default = GuildSettings::default();
	event!(Level::INFO, ?default, "creating table guilds");
	let mut action: CreateTableAction<GuildSettings> = Action::new();
	action.set_table("guilds".to_owned());

	let chart = context.database();

	action.run_create_table(chart).await?;

	Ok(())
}
