use crate::{
	helpers::{cache::MemberHelper, CacheHelper},
	slashies::{interaction::Interaction, Response, SlashCommand},
	utils::{constants::SlashiesErrorMessages, CacheReliant},
};
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use miette::{IntoDiagnostic, Result};
use twilight_cache_inmemory::ResourceType;
use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};
use twilight_mention::Mention;
use twilight_model::{
	application::{
		command::{BaseCommandOptionData, Command, CommandOption, CommandType},
		interaction::ApplicationCommand,
	},
	datetime::Timestamp,
	guild::Role,
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
impl SlashCommand for Info {
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

	#[allow(clippy::too_many_lines, clippy::cast_sign_loss)]
	async fn run(&self, interaction: Interaction<'_>) -> Result<()> {
		let guild_id = if let Some(id) = interaction.command.guild_id {
			id
		} else {
			interaction
				.response(Response::error(SlashiesErrorMessages::GuildOnly))
				.await
				.into_diagnostic()?;

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
					.await
					.into_diagnostic()?;
				return Ok(());
			}
		} else {
			interaction
				.response(Response::error(SlashiesErrorMessages::CantGetUser))
				.await
				.into_diagnostic()?;
			return Ok(());
		};

		let helper = CacheHelper::new(&interaction.state);

		let member: MemberHelper = helper.member(guild_id, user.id).await.into_diagnostic()?;
		let created_at_timestamp = user.id.timestamp();
		let current_user = helper.current_user().await.into_diagnostic()?;
		let current_user_enum = UserOrCurrentUser::from(&current_user);

		let user_enum = UserOrCurrentUser::from(user);
		let created_at_formatted = Utc
			.timestamp_millis(created_at_timestamp)
			.format(Self::FORMAT_TYPE)
			.to_string();

		let joined_at_timestamp: Option<String> = member.joined_at.map(|timestamp: Timestamp| {
			let parsed: DateTime<Utc> = timestamp
				.iso_8601()
				.to_string()
				.parse()
				.expect("failed to parse into datetime");

			parsed.format(Self::FORMAT_TYPE).to_string()
		});

		let mut roles: Vec<Role> = helper
			.member_roles(guild_id, user.id)
			.await
			.into_diagnostic()?;

		roles.reverse();

		let mut embed_builder = Self::BASE
			.thumbnail(ImageSource::url(user_avatar(&user_enum)).into_diagnostic()?)
			.description(format!(
				"**{name}#{discriminator}** - {mention} - [Avatar]({avatar})",
				name = user.name,
				discriminator = user.discriminator,
				mention = user.mention(),
				avatar = user_avatar(&user_enum)
			))
			.footer(
				EmbedFooterBuilder::new(format!("ID: {}", user.id))
					.icon_url(ImageSource::url(user_avatar(&current_user_enum)).into_diagnostic()?),
			)
			.field(EmbedFieldBuilder::new("Created At", created_at_formatted))
			.timestamp(
				Timestamp::from_micros(Utc::now().timestamp_millis() as u64)
					.expect("failed to create timestamp (this shouldn't happen)"),
			);

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
				Response::from(embed_builder.build().into_diagnostic()?)
					.allowed_mentions(|builder| builder.replied_user().user_ids([user.id])),
			)
			.await
			.into_diagnostic()?;

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

	const fn discriminator(&self) -> u16 {
		match *self {
			Self::CurrentUser(user) => user.discriminator,
			Self::User(user) => user.discriminator,
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
				user.discriminator() % 5
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
