mod guild;
use starchart::{
	action::{ActionError, CreateTableAction},
	backend::FsError,
	Action,
};

pub use self::guild::{GuildSettings, GuildTag};
use crate::{prelude::*, state::Context};

// custom function to initialize all tables.
#[instrument(skip(context))]
pub async fn init_tables(context: Context) -> Result<(), ActionError<FsError>> {
	let default = GuildSettings::default();
	event!(Level::INFO, ?default, "creating table guilds");
	let mut action: CreateTableAction<GuildSettings> = Action::new();
	action.set_table("guilds");

	let chart = context.database();

	// chart
	// 	.run(action)
	// 	.await
	// 	.into_diagnostic()?
	// 	.into_diagnostic()?;

	action.run_create_table(chart).await?;

	Ok(())
}
