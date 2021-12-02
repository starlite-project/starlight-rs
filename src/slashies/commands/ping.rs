use std::pin::Pin;

use futures_util::Future;
use twilight_interactions::command::CreateCommand;

use crate::{
	helpers::InteractionsHelper,
	prelude::*,
	slashies::{SlashCommand, SlashData},
};

#[derive(Debug, Clone, Copy, CreateCommand)]
#[command(name = "ping", desc = "Pings the bot")]
pub struct Ping {}

impl SlashCommand for Ping {
	fn run(
		&self,
		helper: InteractionsHelper,
		responder: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>>>> {
		Box::pin(async move {
			panic!("todo");
		})
	}
}
