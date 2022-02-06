use std::convert::Infallible;

use starchart::{action::CreateEntryAction, Action, Result as ChartResult};
use starlight_macros::model;
use tracing::{event, Level};
use twilight_gateway::Event;
use twilight_model::{
	application::interaction::Interaction,
	gateway::payload::incoming::{InteractionCreate, Ready},
	guild::Guild,
	oauth::current_application_info::CurrentApplicationInfo,
};

use super::Context;
use crate::{
	prelude::*,
	settings::{GlobalSettings, GuildSettings, Tables},
};

// these should all be the same caller context, taking a `Context` as the first parameter, and whatever the event content is in the second.
// however, they should return as strict of an error type as possible, using `Infallible` whevever possible (for more optimizations).
pub(super) async fn handle(context: Context, event: Event) {
	if let Err(e) = match event {
		Event::Ready(e) => ready(context, *e).await,
		Event::GuildCreate(e) => guild_create(context, (*e).0).await.into_diagnostic(),
		Event::InteractionCreate(e) => {
			interaction_create(context, *e).await;
			Ok(())
		}
		_ => Ok(()),
	} {
		event!(Level::ERROR, "error occurred: {:?}", e);
	}
}

#[allow(clippy::unused_async)]
async fn ready(context: Context, ready: Ready) -> Result<()> {
	event!(Level::INFO, user_name = %ready.user.name);
	event!(Level::INFO, guilds = %ready.guilds.len());

	let http = context.http();

	let current_user_app_future = http.current_user_application();

	let current_user_app: CurrentApplicationInfo =
		model!(current_user_app_future).await.into_diagnostic()?;

	let app_id = current_user_app.id;
	// there will always be at least 1
	let mut owners = Vec::with_capacity(1);
	if let Some(team) = current_user_app.team {
		let ids = team.members.iter().map(|mem| mem.user.id);
		owners.extend(ids);
	}

	owners.push(current_user_app.owner.id);

	owners.sort();
	owners.dedup();

	let settings = GlobalSettings::new(app_id, owners);

	Tables::Global.create_entry(context.database(), &settings).await?;

	Ok(())
}

async fn guild_create(context: Context, guild: Guild) -> ChartResult<()> {
	let id = guild.id;
	let database = context.database();

	let mut action: CreateEntryAction<GuildSettings> = Action::new();

	let table = Tables::Guilds.to_string();
	let entry = GuildSettings::new(id);

	action.set_entry(&entry).set_table(&table);

	action.run_create_entry(database).await?;

	Ok(())
}

async fn interaction_create(context: Context, interaction: InteractionCreate) {
	match interaction.0 {
		Interaction::ApplicationCommand(cmd) | Interaction::ApplicationCommandAutocomplete(cmd) => {
			context.helpers().interactions().handle(*cmd).await;
		}
		Interaction::MessageComponent(_) => {}
		i => event!(Level::WARN, ?i, "unhandled interaction"),
	}
}
