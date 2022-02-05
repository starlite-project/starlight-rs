use twilight_model::{
	application::{
		command::CommandType,
		interaction::application_command::{CommandData, CommandDataOption},
	},
	id::{marker::UserMarker, Id},
};
use twilight_util::builder::command::{
	CommandBuilder, StringBuilder, SubCommandBuilder, UserBuilder,
};

use crate::{
	helpers::{
		parsing::{parse_subcommand, CommandParse},
		InteractionsHelper,
	},
	prelude::*,
	slashies::{DefineCommand, SlashCommand, SlashData},
};

#[derive(Debug, Clone)]
pub enum Block {
	Add {
		user: Id<UserMarker>,
		reason: String,
	},
	Remove {
		user: Id<UserMarker>,
	},
	Why {
		user: Id<UserMarker>,
	},
}

impl Block {
	fn parse_user(data: &[CommandDataOption]) -> Option<Id<UserMarker>> {
		data.iter()
			.find(|opt| opt.name == "user")
			.cloned()
			.and_then(|opt| opt.value.parse_option())
	}

	fn parse_add(data: &[CommandDataOption]) -> Result<Self> {
		let id = Self::parse_user(data)
			.ok_or_else(|| error!("couldn't parse user (this shouldn't happen)"))?;

		let reason = data
			.iter()
			.find(|opt| opt.name == "reason")
			.cloned()
			.and_then(|opt| opt.value.parse_option())
			.unwrap_or_default();

		Ok(Self::Add { user: id, reason })
	}

	fn parse_remove(data: &[CommandDataOption]) -> Result<Self> {
		Ok(Self::Remove {
			user: Self::parse_user(data)
				.ok_or_else(|| error!("couldn't parse user (this shouldn't happen)"))?,
		})
	}

	fn parse_why(data: &[CommandDataOption]) -> Result<Self> {
		Ok(Self::Why {
			user: Self::parse_user(data)
				.ok_or_else(|| error!("couldn't parse user (this shouldn't happen)"))?,
		})
	}
}

impl SlashCommand for Block {
	fn run<'a>(
		&'a self,
		helper: InteractionsHelper,
		responder: SlashData,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
		todo!()
	}
}

impl DefineCommand for Block {
	fn define() -> CommandBuilder {
		CommandBuilder::new(
			"block".to_owned(),
			"Blocks a user from using the bot".to_owned(),
			CommandType::ChatInput,
		)
		.default_permission(true)
		.option(
			SubCommandBuilder::new("add".to_owned(), "Adds a user to the block list".to_owned())
				.option(
					UserBuilder::new("user".to_owned(), "User to block".to_owned()).required(true),
				)
				.option(
					StringBuilder::new(
						"reason".to_owned(),
						"Reason for blocking the user".to_owned(),
					)
					.required(true),
				),
		)
		.option(
			SubCommandBuilder::new(
				"remove".to_owned(),
				"Removes a user from the blocklist".to_owned(),
			)
			.option(
				UserBuilder::new("user".to_owned(), "User to remove".to_owned()).required(true),
			),
		)
		.option(
			SubCommandBuilder::new(
				"why".to_owned(),
				"Fetch the reason for blocking the user".to_owned(),
			)
			.option(
				UserBuilder::new("user".to_owned(), "User to see reason of".to_owned())
					.required(true),
			),
		)
	}

	fn parse(mut data: CommandData) -> Result<Self> {
		if data.options.len() != 1 {
			return Err(error!(
				"more than one subcommand was received (this shouldn't happen)"
			));
		}

		let subcommand_value = unsafe { data.options.pop().unwrap_unchecked() };
		let args = parse_subcommand(subcommand_value.value)
			.ok_or_else(|| error!("invalid subcommand value option"))?;

		match subcommand_value.name.as_str() {
			"add" => Self::parse_add(&args),
			"remove" => Self::parse_remove(&args),
			"why" => Self::parse_why(&args),
			_ => Err(error!("invalid subcommand variant")),
		}
	}
}
