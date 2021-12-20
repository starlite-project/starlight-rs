use std::pin::Pin;

use futures_util::Future;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_util::builder::command::CommandBuilder;

use super::SlashData;
use crate::{helpers::InteractionsHelper, prelude::*};

pub trait SlashCommand: Send + Sync {
	fn run<'a>(
		&'a self,
		helper: InteractionsHelper,
		responder: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>> + Send + 'a>>;

	#[allow(unused_variables)]
	fn autocomplete<'a>(
		&'a self,
		helper: InteractionsHelper,
		responder: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>> + Send + 'a>> {
		Box::pin(async { Ok(()) })
	}
}

pub trait DefineCommand: SlashCommand + Sized {
	fn define() -> CommandBuilder;

	fn parse(data: CommandData) -> MietteResult<Self>;
}
