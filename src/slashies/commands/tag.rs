use std::pin::Pin;

use futures_util::Future;
use twilight_model::application::{
	command::CommandType, interaction::application_command::CommandData,
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder, SubCommandBuilder};

use crate::{
	helpers::InteractionsHelper,
	prelude::*,
	slashies::{DefineCommand, SlashCommand, SlashData},
};

#[derive(Debug, Clone)]
pub enum Tag {
	Add { name: String, content: String },
	Delete { name: String },
}

impl SlashCommand for Tag {
	fn run<'a>(
		&'a self,
		helper: InteractionsHelper,
		responder: SlashData,
	) -> Pin<Box<dyn Future<Output = MietteResult<()>> + Send + 'a>> {
		Box::pin(async { Ok(()) })
	}
}

impl DefineCommand for Tag {
	fn define() -> CommandBuilder {
		CommandBuilder::new(
			"tag".to_owned(),
			"Show, create, and edit tags!".to_owned(),
			CommandType::ChatInput,
		)
		.default_permission(true)
		.option(
			SubCommandBuilder::new("add".to_owned(), "Add a tag".to_owned())
				.option(
					StringBuilder::new("name".to_owned(), "Name of the new tag".to_owned())
						.required(true),
				)
				.option(
					StringBuilder::new("content".to_owned(), "Content of the tag".to_owned())
						.required(true),
				),
		)
		.option(SubCommandBuilder::new(
			"delete".to_owned(),
			"Delete a tag".to_owned(),
		)
	.option(StringBuilder::new("name".to_owned(), "Name of the tag".to_owned())))
	}

	fn parse(_: CommandData) -> MietteResult<Self> {
		Err(error!("todo"))
	}
}
