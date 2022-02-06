use std::{hint::unreachable_unchecked, pin::Pin};

use twilight_model::{
	application::{
		command::{CommandOptionChoice, CommandType},
		interaction::application_command::{CommandData, CommandDataOption},
	},
	guild::Permissions,
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder, SubCommandBuilder};

use crate::{
	helpers::{
		parsing::{parse_subcommand, CommandParse},
		InteractionsHelper,
	},
	prelude::*,
	settings::{GuildSettings, GuildTag, Tables},
	slashies::{DefineCommand, SlashCommand, SlashData},
	utils::{levenshtein, DefaultMessages},
};

#[derive(Debug, Clone)]
pub enum Tag {
	Add { name: String, content: String },
	Delete { name: String },
	Edit { name: String, content: String },
	Show { name: String },
}

impl Tag {
	fn parse_full(data: &[CommandDataOption]) -> (String, String) {
		let name = data
			.iter()
			.find(|opt| opt.name == "name")
			.cloned()
			.and_then(|opt| opt.value.parse_option())
			.unwrap_or_default();
		let content = data
			.iter()
			.find(|opt| opt.name == "content")
			.cloned()
			.and_then(|opt| opt.value.parse_option())
			.unwrap_or_default();

		(name, content)
	}

	fn parse_add(data: &[CommandDataOption]) -> Self {
		let (name, content) = Self::parse_full(data);
		Self::Add { name, content }
	}

	fn parse_edit(data: &[CommandDataOption]) -> Self {
		let (name, content) = Self::parse_full(data);
		Self::Edit { name, content }
	}

	fn parse_name(data: &[CommandDataOption]) -> String {
		let name = data
			.iter()
			.find(|opt| opt.name == "name")
			.cloned()
			.and_then(|opt| opt.value.parse_option())
			.unwrap_or_default();

		name
	}

	fn parse_delete(data: &[CommandDataOption]) -> Self {
		Self::Delete {
			name: Self::parse_name(data),
		}
	}

	fn parse_show(data: &[CommandDataOption]) -> Self {
		Self::Show {
			name: Self::parse_name(data),
		}
	}

	async fn run_show(self, helper: InteractionsHelper, mut responder: SlashData) -> Result<()> {
		if let Self::Show { name } = self {
			let guild_settings = Tables::Guilds
				.get_entry::<GuildSettings>(helper.database(), unsafe {
					&responder.guild_id.unwrap_unchecked()
				})
				.await?;

			if let Some(tag) = guild_settings.tags().iter().find(|tag| tag.name() == name) {
				responder.message(tag.description().to_owned());
				helper.respond(&mut responder).await.into_diagnostic()?;
			} else {
				responder.message(format!("couldn't find tag `{}`", name));
				helper.respond(&mut responder).await.into_diagnostic()?;
			}
		} else {
			unsafe { unreachable_unchecked() }
		}

		Ok(())
	}

	async fn run_add(self, helper: InteractionsHelper, mut responder: SlashData) -> Result<()> {
		if let Self::Add { name, content } = self {
			let mut guild_settings = Tables::Guilds
				.get_entry::<GuildSettings>(helper.database(), unsafe {
					&responder.guild_id.unwrap_unchecked()
				})
				.await?;

			if guild_settings.tags().iter().any(|tag| tag.name() == name) {
				responder.message(format!(
					"the guild tag `{}` already exists, try editing or deleting it first.",
					&name
				));
				helper.respond(&mut responder).await.into_diagnostic()?;
				return Ok(());
			}
			responder.message(format!("successfully created tag `{}`.", &name));

			let guild_tag = GuildTag::new(name, content, responder.user_id());

			guild_settings.extend(Some(guild_tag));

			Tables::Guilds
				.update_entry(helper.database(), &guild_settings)
				.await?;

			helper.respond(&mut responder).await.into_diagnostic()?;
		} else {
			unsafe { unreachable_unchecked() }
		}

		Ok(())
	}

	async fn run_edit(self, helper: InteractionsHelper, mut responder: SlashData) -> Result<()> {
		if let Self::Edit { name, content } = self {
			let mut guild_settings = Tables::Guilds
				.get_entry::<GuildSettings>(helper.database(), unsafe {
					&responder.guild_id.unwrap_unchecked()
				})
				.await?;

			let can_manage_messages = {
				let user_perms = responder.author_permissions(&helper)?;

				user_perms.contains(Permissions::MANAGE_MESSAGES)
					|| user_perms.contains(Permissions::ADMINISTRATOR)
			};

			let user_id = responder.user_id();

			if let Some(tag) = guild_settings
				.tags_mut()
				.iter_mut()
				.find(|tag| tag.name() == name)
			{
				if !can_manage_messages && tag.author() != user_id {
					responder.message(DefaultMessages::PermissionDenied.to_string());
					helper.respond(&mut responder).await.into_diagnostic()?;
					return Ok(());
				}
				tag.set_description(content);

				Tables::Guilds
					.update_entry(helper.database(), &guild_settings)
					.await?;

				responder.message(format!("successfully edited `{}`", name));
				helper.respond(&mut responder).await.into_diagnostic()?;
			} else {
				responder.message(format!(
					"the guild tag `{}` doesn't exist, try creating it first.",
					name
				));

				helper.respond(&mut responder).await.into_diagnostic()?;
			}
		} else {
			unsafe { unreachable_unchecked() }
		}

		Ok(())
	}

	async fn run_delete(self, helper: InteractionsHelper, mut responder: SlashData) -> Result<()> {
		if let Self::Delete { name } = self {
			let mut guild_settings = Tables::Guilds
				.get_entry::<GuildSettings>(helper.database(), unsafe {
					&responder.guild_id.unwrap_unchecked()
				})
				.await?;

			let can_manage_messages = {
				let user_perms = responder.author_permissions(&helper)?;

				user_perms.contains(Permissions::MANAGE_MESSAGES)
					|| user_perms.contains(Permissions::ADMINISTRATOR)
			};

			let user_id = responder.user_id();

			let mut manageable_tags = guild_settings
				.tags()
				.iter()
				.filter(|tag| tag.author() == user_id)
				.map(GuildTag::name);

			if !can_manage_messages && !manageable_tags.any(|tag| tag == name) {
				responder.message(DefaultMessages::PermissionDenied.to_string());
				helper.respond(&mut responder).await.into_diagnostic()?;
				return Ok(());
			}

			if guild_settings.remove_tag(&name).is_none() {
				responder.message(format!("tag `{}` was not found.", &name));
				helper.respond(&mut responder).await.into_diagnostic()?;
				return Ok(());
			}

			Tables::Guilds
				.update_entry(helper.database(), &guild_settings)
				.await?;

			responder.message(format!("tag `{}` was successfully deleted.", &name));
			helper.respond(&mut responder).await.into_diagnostic()?;
		} else {
			unsafe { unreachable_unchecked() }
		}

		Ok(())
	}
}

impl SlashCommand for Tag {
	fn run(
		&self,
		helper: InteractionsHelper,
		mut responder: SlashData,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
		async move {
			if responder.is_dm() {
				responder.message("this command can only be used in a guild".to_owned());
				helper.respond(&mut responder).await.into_diagnostic()?;
				return Ok(());
			}
			match self {
				Self::Add { .. } => self.clone().run_add(helper, responder).await,
				Self::Delete { .. } => self.clone().run_delete(helper, responder).await,
				Self::Edit { .. } => self.clone().run_edit(helper, responder).await,
				Self::Show { .. } => self.clone().run_show(helper, responder).await,
			}
		}
		.boxed()
	}

	fn autocomplete<'a>(
		&'a self,
		helper: InteractionsHelper,
		mut responder: SlashData,
	) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
		async move {
			if responder.is_dm() {
				return Ok(());
			}
			let name = match self {
				Self::Add { .. } => unsafe { unreachable_unchecked() },
				Self::Edit { name, .. } | Self::Delete { name, .. } | Self::Show { name } => {
					name.as_str()
				}
			};

			if name.len() < 3 {
				return Ok(());
			}

			let mut guild_settings = Tables::Guilds
				.get_entry::<GuildSettings>(helper.database(), unsafe {
					&responder.guild_id.unwrap_unchecked()
				})
				.await?;

			let tags = guild_settings.tags_mut();

			let can_manage_messages = {
				let user_permissions = responder.author_permissions(&helper)?;

				user_permissions.contains(Permissions::ADMINISTRATOR)
					|| user_permissions.contains(Permissions::MANAGE_MESSAGES)
			};
			let user_id = responder.user_id();

			tags.sort_by(|first, second| {
				levenshtein(first.name(), name).cmp(&levenshtein(second.name(), name))
			});

			let results = tags
				.iter()
				.filter_map(|tag| {
					if (can_manage_messages || tag.author() == user_id)
						&& levenshtein(tag.name(), name) < 3
					{
						Some(CommandOptionChoice::String {
							name: tag.name().to_owned(),
							value: tag.name().to_owned(),
						})
					} else {
						None
					}
				})
				.collect();

			responder.autocomplete(results);
			helper
				.autocomplete(&mut responder)
				.await
				.into_diagnostic()?;

			Ok(())
		}
		.boxed()
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
				StringBuilder::new("name".to_owned(), "Name of the tag".to_owned())
					.required(true)
					.autocomplete(true),
			),
		)
		.option(
			SubCommandBuilder::new("edit".to_owned(), "Edits a tags description".to_owned())
				.option(
					StringBuilder::new("name".to_owned(), "Name of the new tag".to_owned())
						.required(true)
						.autocomplete(true),
				)
				.option(
					StringBuilder::new("content".to_owned(), "Content of the tag".to_owned())
						.required(true)
						.autocomplete(true),
				),
		)
		.option(
			SubCommandBuilder::new("show".to_owned(), "View a specific tag".to_owned()).option(
				StringBuilder::new("name".to_owned(), "Name of the tag".to_owned())
					.required(true)
					.autocomplete(true),
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
			"add" => Ok(Self::parse_add(&args)),
			"delete" => Ok(Self::parse_delete(&args)),
			"edit" => Ok(Self::parse_edit(&args)),
			"show" => Ok(Self::parse_show(&args)),
			_ => Err(error!("invalid subcommand variant")),
		}
	}
}
