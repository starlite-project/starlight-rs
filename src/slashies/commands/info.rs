use super::SlashCommand;
use crate::{helpers::CacheHelper, slashies::Response, state::State, utils::{CacheReliant, constants::SlashiesErrorMessages}};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use twilight_cache_inmemory::ResourceType;
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};
use twilight_mention::Mention;
use twilight_model::{
	application::{
		command::{BaseCommandOptionData, Command, CommandOption, CommandType},
		interaction::ApplicationCommand,
	},
	id::UserId,
	user::{CurrentUser, User},
};
use twilight_util::snowflake::Snowflake;

#[derive(Debug, Clone)]
pub struct Info(pub(super) ApplicationCommand);

impl Info {
	const BASE: EmbedBuilder = EmbedBuilder::new();

	const FORMAT_TYPE: &'static str = "%D, %r";
}

impl CacheReliant for Info {
	fn needs() -> ResourceType {
		ResourceType::USER | ResourceType::MEMBER | ResourceType::USER_CURRENT | ResourceType::ROLE
	}
}

#[async_trait]
impl SlashCommand<0> for Info {
	const NAME: &'static str = "info";

	fn define() -> Command {
		Command {
			application_id: None,
			guild_id: None,
			name: String::from(Self::NAME),
			default_permission: None,
			description: String::from("Get info about a user"),
			id: None,
			options: vec![CommandOption::User(BaseCommandOptionData {
				name: String::from("user"),
				description: String::from(
					"The user to get information about, defaulting to the author",
				),
				required: false,
			})],
			kind: CommandType::ChatInput,
		}
	}

	async fn run(&self, state: State) -> Result<()> {
		let interaction = state.interaction(&self.0);

		let guild_id = if let Some(id) = interaction.command.guild_id {
			id
		} else {
			interaction
				.response(Response::error(SlashiesErrorMessages::GuildOnly))
				.await?;

			return Ok(());
		};

		let user = if let Some(resolved) = &interaction.command.data.resolved {
			resolved
				.users
				.first()
				.unwrap_or_else(|| supernova::debug_unreachable!())
		} else if let Some(member) = &interaction.command.member {
			if let Some(user) = member.user.as_ref() {
				user
			} else {
				interaction
					.response(Response::error(SlashiesErrorMessages::CantGetUser))
					.await?;
				return Ok(());
			}
		} else {
			interaction
				.response(Response::error(SlashiesErrorMessages::CantGetUser))
				.await?;
			return Ok(());
		};

		let helper = CacheHelper::new(&interaction.state);

		let member = helper.member(guild_id, user.id).await?;

		let created_at_timestamp = user.id.timestamp();

		let current_user = helper.current_user().await?;

		let current_user_enum = UserOrCurrentUser::from(&current_user);

		let user_enum = UserOrCurrentUser::from(user);

		let created_at_formatted = Utc
			.timestamp_millis(created_at_timestamp)
			.format(Self::FORMAT_TYPE)
			.to_string();

		let joined_at_timestamp: Option<String> = member.joined_at.map(|timestamp| {
			let parsed: DateTime<Utc> = timestamp.parse().expect("failed to parse into datetime");

			parsed.format(Self::FORMAT_TYPE).to_string()
		});

		let mut roles = helper.member_roles(guild_id, user.id).await?;

		roles.reverse();

		let mut embed_builder = Self::BASE
			.thumbnail(ImageSource::url(user_avatar(&user_enum))?)
			.description(format!(
				"**{name}#{discriminator}** - {mention} - [Avatar]({avatar})",
				name = user.name,
				discriminator = user.discriminator,
				mention = user.mention(),
				avatar = user_avatar(&user_enum)
			))
			.footer(
				EmbedFooterBuilder::new(format!("ID: {}", user.id))
					.icon_url(ImageSource::url(user_avatar(&current_user_enum))?),
			)
			.field(EmbedFieldBuilder::new("Created At", created_at_formatted))
			.timestamp(format!("{:?}", Utc::now()));

		let user_color = roles
			.iter()
			.map(|role| role.color)
			.find(|color| color != &0);

		embed_builder = match user_color {
			Some(color) if color != 0 => embed_builder.color(dbg!(color)),
			_ => embed_builder,
		};

		embed_builder = joined_at_timestamp.map_or(embed_builder.clone(), |joined_timestamp| {
			embed_builder.field(EmbedFieldBuilder::new("Joined At", joined_timestamp))
		});

		embed_builder = if roles.len() == 1
			&& roles
				.get(0)
				.unwrap_or_else(|| supernova::debug_unreachable!())
				.id == guild_id.0.into()
		{
			embed_builder
		} else {
			embed_builder.field(EmbedFieldBuilder::new(
				format!("Roles ({})", roles.len()),
				roles
					.iter()
					.map(|role| role.mention().to_string())
					.collect::<Vec<_>>()
					.join(" "),
			))
		};

		interaction
			.response(
				Response::from(embed_builder.build()?)
					.allowed_mentions(|builder| builder.replied_user().user_ids([user.id])),
			)
			.await?;

		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum UserOrCurrentUser<'a> {
	CurrentUser(&'a CurrentUser),
	User(&'a User),
}

impl<'a> UserOrCurrentUser<'a> {
	const fn avatar(&self) -> &'a Option<String> {
		match *self {
			Self::CurrentUser(user) => &user.avatar,
			Self::User(user) => &user.avatar,
		}
	}

	const fn id(&self) -> UserId {
		match *self {
			Self::CurrentUser(user) => user.id,
			Self::User(user) => user.id,
		}
	}

	const fn discriminator(&self) -> &'a String {
		match *self {
			Self::CurrentUser(user) => &user.discriminator,
			Self::User(user) => &user.discriminator,
		}
	}
}

impl<'a> From<&'a CurrentUser> for UserOrCurrentUser<'a> {
	fn from(current_user: &'a CurrentUser) -> Self {
		Self::CurrentUser(current_user)
	}
}

impl<'a> From<&'a User> for UserOrCurrentUser<'a> {
	fn from(user: &'a User) -> Self {
		Self::User(user)
	}
}

fn user_avatar(user: &UserOrCurrentUser) -> String {
	user.avatar().as_ref().map_or_else(
		|| {
			format!(
				"https://cdn.discordapp.com/embed/avatars/{}.png",
				user.discriminator()
					.chars()
					.last()
					.unwrap_or_else(|| supernova::debug_unreachable!())
					.to_digit(10)
					.unwrap_or_else(|| supernova::debug_unreachable!())
			)
		},
		|hash| {
			format!(
				"https://cdn.discordapp.com/avatars/{}/{}.{}",
				user.id(),
				hash,
				if hash.starts_with("a_") { "gif" } else { "png" }
			)
		},
	)
}
