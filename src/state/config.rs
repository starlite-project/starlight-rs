use std::env::{self, VarError};

use clap::{
	crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches,
	Error as ClapError, FromArgMatches, IntoApp, Parser,
};
use miette::{IntoDiagnostic, Result};
use tracing::instrument;
use twilight_model::id::{
	marker::{ApplicationMarker, GuildMarker},
	Id,
};

const REMOVE_SLASH_COMMANDS: &str = "remove-slash-commands";
const GUILD_ID: &str = "guild-id";

// static mut TOKEN: Option<&str> = None;
const TOKEN: Option<&'static str> = option_env!("DISCORD_TOKEN");

static mut APPLICATION_ID: Option<Id<ApplicationMarker>> = None;

#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
	pub guild_id: Option<Id<GuildMarker>>,
	pub remove_slash_commands: bool,
}

impl Config {
	pub fn application_id() -> Result<Id<ApplicationMarker>> {
		unsafe {
			if let Some(id) = APPLICATION_ID {
				return Ok(id);
			}
		}

		let token = Self::token().into_diagnostic()?;

		let first_part = token.split('.').next().unwrap_or_default();

		let decoded = base64::decode(first_part).into_diagnostic()?;

		let value = unsafe { String::from_utf8_unchecked(decoded) }
			.parse()
			.into_diagnostic()?;

		unsafe { APPLICATION_ID = Id::new_checked(value) };

		Ok(unsafe { Id::new_unchecked(value) })
	}

	#[instrument]
	pub fn token() -> Result<String, VarError> {
		TOKEN.map_or_else(|| env::var("DISCORD_TOKEN"), |token| Ok(token.to_owned()))
	}
}

impl IntoApp for Config {
	fn into_app<'help>() -> App<'help> {
		App::new(crate_name!())
			.about(crate_description!())
			.version(crate_version!())
			.author(crate_authors!())
			.args(&[
				Arg::new(GUILD_ID)
					.help("Guild ID to use for testing")
					.env("GUILD_ID")
					.long("guild-id")
					.short('g')
					.takes_value(true),
				Arg::new(REMOVE_SLASH_COMMANDS)
					.help("Removes the global slash commands and exits")
					.env("DELETE_SLASH_COMMANDS")
					.long("delete-slash-commands"),
			])
	}

	fn into_app_for_update<'help>() -> App<'help> {
		Self::into_app()
	}
}

impl FromArgMatches for Config {
	fn from_arg_matches(matches: &ArgMatches) -> Result<Self, ClapError> {
		let guild_id = if cfg!(debug_assertions) {
			match matches.value_of_t::<u64>(GUILD_ID) {
				Ok(g) => Id::new_checked(g),
				Err(e) if e.kind == clap::ErrorKind::ArgumentNotFound => None,
				Err(e) => return Err(e),
			}
		} else {
			None
		};

		Ok(Self {
			guild_id,
			remove_slash_commands: matches.is_present(REMOVE_SLASH_COMMANDS),
		})
	}

	fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), ClapError> {
		let guild_id = if cfg!(debug_assertions) {
			match matches.value_of_t::<u64>(GUILD_ID) {
				Ok(g) => Id::new_checked(g),
				Err(e) if e.kind == clap::ErrorKind::ArgumentNotFound => None,
				Err(e) => return Err(e),
			}
		} else {
			None
		};

		self.guild_id = guild_id;

		self.remove_slash_commands = matches.is_present(REMOVE_SLASH_COMMANDS);

		Ok(())
	}
}

impl Parser for Config {}
