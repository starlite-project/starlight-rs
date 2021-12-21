use std::{
	env::{self, VarError},
	num::NonZeroU64,
};

use clap::{
	crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches,
	Error as ClapError, FromArgMatches, IntoApp, Parser,
};
use miette::{IntoDiagnostic, Result as MietteResult};
use tracing::instrument;
use twilight_model::id::{ApplicationId, GuildId};

const REMOVE_SLASH_COMMANDS: &str = "remove-slash-commands";
const GUILD_ID: &str = "guild-id";

static mut TOKEN: Option<&str> = None;

#[derive(Debug, Default, Clone, Copy)]
pub struct Config {
	pub guild_id: Option<GuildId>,
	pub remove_slash_commands: bool,
}

impl Config {
	pub fn application_id() -> MietteResult<ApplicationId> {
		let first_part = Self::token()
			.into_diagnostic()?
			.split('.')
			.next()
			.unwrap_or_default();

		let decoded = base64::decode(first_part).into_diagnostic()?;

		let value = unsafe { String::from_utf8_unchecked(decoded) }
			.parse()
			.into_diagnostic()?;

		Ok(ApplicationId(value))
	}

	#[instrument]
	pub fn token() -> Result<&'static str, VarError> {
		if let Some(token) = unsafe { TOKEN } {
			Ok(token)
		} else {
			let token = env::var("DISCORD_TOKEN")?;
			unsafe {
				let leaked = Box::leak(token.into_boxed_str());
				TOKEN = Some(leaked);
				Ok(leaked)
			}
		}
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
			match matches.value_of_t::<NonZeroU64>(GUILD_ID) {
				Ok(g) => Some(GuildId(g)),
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
			match matches.value_of_t::<NonZeroU64>(GUILD_ID) {
				Ok(g) => Some(GuildId(g)),
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
