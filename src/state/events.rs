use std::convert::Infallible;

use tracing::{event, Level};
use twilight_gateway::Event;
use twilight_model::{
	application::interaction::Interaction,
	gateway::payload::incoming::{InteractionCreate, Ready},
};

use super::Context;
use crate::prelude::*;

pub(super) async fn handle(context: Context, event: Event) -> MietteResult<()> {
	match event {
		Event::Ready(e) => ready(context, *e).await.into_diagnostic(),
		Event::InteractionCreate(e) => interaction_create(context, *e).await,
		_ => Ok(()),
	}
}

#[allow(clippy::unused_async)]
async fn ready(_: Context, ready: Ready) -> Result<(), Infallible> {
	event!(Level::INFO, user_name = %ready.user.name);
	event!(Level::INFO, guilds = %ready.guilds.len());
	Ok(())
}

async fn interaction_create(context: Context, interaction: InteractionCreate) -> MietteResult<()> {
	match interaction.0 {
		Interaction::ApplicationCommand(cmd) | Interaction::ApplicationCommandAutocomplete(cmd) => {
			context.helpers().interactions().handle(*cmd).await;
		}
		Interaction::MessageComponent(_) => {}
		i => event!(Level::WARN, ?i, "unhandled interaction"),
	}

	Ok(())
}
