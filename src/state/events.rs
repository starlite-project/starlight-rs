use std::convert::Infallible;

use miette::{IntoDiagnostic, Result as MietteResult};
use tracing::{event, Level};
use twilight_gateway::Event;
use twilight_model::gateway::payload::incoming::Ready;

use super::Context;

pub(super) async fn handle(context: Context, event: Event) -> MietteResult<()> {
	match event {
		Event::Ready(r) => ready(context, *r).await.into_diagnostic(),
		_ => Ok(()),
	}
}

#[allow(clippy::unused_async)]
async fn ready(context: Context, ready: Ready) -> Result<(), Infallible> {
	event!(Level::INFO, user_name = %ready.user.name);
	event!(Level::INFO, guilds = %ready.guilds.len());
	Ok(())
}
