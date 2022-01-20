use std::{hint::unreachable_unchecked, mem, pin::Pin};

use futures_util::Future;
use starchart::{
	action::{CreateEntryAction, ReadEntryAction},
	Action,
};
use twilight_model::application::{
	command::CommandType,
	interaction::application_command::{CommandData, CommandDataOption, CommandOptionValue},
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder, SubCommandBuilder};

use crate::{
	helpers::{parsing::CommandParse, InteractionsHelper},
	prelude::*,
	settings::{GuildSettings, GuildTag, Tables},
	slashies::{DefineCommand, SlashCommand, SlashData},
};

#[derive(Debug, Clone)]
pub enum Tag {
	Add { name: String, content: String },
	Delete { name: String },
}

impl Tag {
	fn parse_add(mut data: Vec<CommandDataOption>) -> Result<Self> {
		if data.len() != 2 {
			return Err(error!("only expected 2 arguments (this shouldn't happen)"));
		}

		// we don't know the order, so we swap them later.
		let mut content = unsafe { data.pop().unwrap_unchecked() };
		let mut name = unsafe { data.pop().unwrap_unchecked() };

		if name.name == "content" {
			mem::swap(&mut name, &mut content);
		}

		let name = name
			.value
			.parse_option()
			.ok_or_else(|| error!("failed to parse string (this shouldn't happen)"))?;
		let content = content
			.value
			.parse_option()
			.ok_or_else(|| error!("failed to parse string (this shouldn't happen)"))?;

		Ok(Self::Add { name, content })
	}

	fn parse_delete(mut data: Vec<CommandDataOption>) -> Result<Self> {
		if data.len() != 1 {
			return Err(error!("only expected 1 argument (this shouldn't happen)"));
		}

		let name = {
			let raw = unsafe { data.pop().unwrap_unchecked() };
			let option = raw.value;

			option
				.parse_option()
				.ok_or_else(|| error!("failed to parse string (this shouldn't happen)"))
		}?;

		Ok(Self::Delete { name })
	}

	async fn run_add(&self, helper: InteractionsHelper, mut responder: SlashData) -> Result<()> {
		if let Self::Add { name, content } = self {
			let guild_settings = {
				let mut action: ReadEntryAction<GuildSettings> = Action::new();

				action
					.set_table(Tables::Guilds.to_string())
					.set_key(unsafe { &responder.guild_id.unwrap_unchecked() });

				action
					.run_read_entry(helper.database())
					.await
					.map_err(|e| error!(e.to_string()))
			}?
			.ok_or_else(|| error!("failed to get GuildSettings"))?;

			if guild_settings.get_tag(name).is_some() {
				responder
					.message("that guild tag already exists, try editing or deleting it first.");
				helper.respond(&mut responder).await.into_diagnostic()?;
				return Ok(());
			}

			let guild_tag = GuildTag::new(name.clone(), content.clone(), responder.user_id());

			let mut create_action: CreateEntryAction<GuildSettings> = Action::new();

			create_action.set_table(Tables::Guilds.to_string()).set_data(&guild_tag);
		} else {
			unsafe { unreachable_unchecked() }
		}

		Ok(())
	}

	async fn run_delete(&self, _helper: InteractionsHelper, _responder: SlashData) -> Result<()> {
		Ok(())
	}
}

impl SlashCommand for Tag {
	fn run<'a>(
		&'a self,
		helper: InteractionsHelper,
		mut responder: SlashData,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
		Box::pin(async move {
			if responder.is_dm() {
				responder.message("this command can only be used in a guild");
				helper.respond(&mut responder).await.into_diagnostic()?;
				return Ok(());
			}
			match self {
				Self::Add { .. } => self.run_add(helper, responder).await,
				Self::Delete { .. } => self.run_delete(helper, responder).await,
			}
		})
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
		.option(
			SubCommandBuilder::new("delete".to_owned(), "Delete a tag".to_owned()).option(
				StringBuilder::new("name".to_owned(), "Name of the tag".to_owned()).required(true),
			),
		)
	}

	fn parse(mut data: CommandData) -> Result<Self> {
		if data.options.len() != 1 {
			return Err(error!(
				"more than one subcommand was received (this shouldn't happen)"
			));
		}
		let subcommand_value = data
			.options
			.pop()
			.ok_or_else(|| error!("failed to get subcommand value (this shouldn't happen)"))?;
		match subcommand_value.value {
			CommandOptionValue::SubCommand(v) => match subcommand_value.name.as_str() {
				"add" => Self::parse_add(v),
				"delete" => Self::parse_delete(v),
				_ => Err(error!("invalid subcommand variant")),
			},
			_ => Err(error!("invalid subcommand value option")),
		}
	}
}
