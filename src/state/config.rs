use clap::Parser;
use clap::{crate_authors, crate_description, crate_license, crate_name, crate_version, App, Arg};
use miette::{IntoDiagnostic, Result};
use nebula::Id;
use nebula::Leak;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::instrument;
use twilight_model::id::GuildId;

const REMOVE_SLASH_COMMANDS_KEY: &str = "remove-slash-commands";
const GUILD_ID_KEY: &str = "guild-id";

static mut TOKEN: Option<&'static str> = None;

fn parse_guild_id(value: &str) -> Result<GuildId, &'static str> {
	if let Ok(snowflake) = value.parse() {
		Ok(GuildId(snowflake))
	} else {
		Err("expected valid u64")
	}
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Parser)]
#[clap(author, about, version)]
pub struct Config {
	/// The Guild ID to set slash commands in, used for testing
	#[clap(long, short, env = "GUILD_ID", parse(try_from_str = parse_guild_id))]
	pub guild_id: Option<GuildId>,
	/// Whether or not to remove all set slash commands, will use the [`guild_id`] if it's set
	///
	/// [`guild_id`]: Self::guild_id
	#[clap(long, parse(from_flag))]
	pub remove_slash_commands: bool,
}

impl Config {
	pub fn new() -> Result<Self> {
		let matches = App::new(crate_name!())
			.about(crate_description!())
			.author(crate_authors!())
			.license(crate_license!())
			.version(crate_version!())
			.args(&[
				Arg::new(GUILD_ID_KEY)
					.about("Guild Id to use for testing slash commands")
					.env("GUILD_ID")
					.long("guild-id")
					.takes_value(true),
				Arg::new(REMOVE_SLASH_COMMANDS_KEY)
					.about("Removes the global slash commands and exits")
					.env("DELETE_SLASH_COMMANDS")
					.long("delete-slash-commands"),
			])
			.get_matches();

		let guild_id = match matches.value_of_t::<u64>(GUILD_ID_KEY) {
			Ok(g) => GuildId::new(g),
			Err(e) if e.kind == clap::ErrorKind::ArgumentNotFound => None,
			Err(e) => e.exit(),
		};

		let remove_slash_commands = matches.is_present(REMOVE_SLASH_COMMANDS_KEY);

		Ok(Self {
			guild_id,
			remove_slash_commands,
		})
	}

	pub fn application_id() -> Result<Id> {
		let first_part_of_token = Self::token()?.split('.').next().unwrap_or_default();

		let decoded = base64::decode(first_part_of_token).unwrap();

		let value = unsafe { String::from_utf8_unchecked(decoded) }
			.parse()
			.into_diagnostic()?;

		Ok(value)
	}

	#[instrument]
	#[allow(clippy::let_unit_value)]
	pub fn token() -> Result<&'static str> {
		// let token = env::var_os("CREDENTIALS_DIRECTORY").map_or_else(
		// 	|| {
		// 		event!(
		// 			Level::WARN,
		// 			"falling back to `DISCORD_TOKEN` environment variable"
		// 		);
		// 		env::var("DISCORD_TOKEN").into_diagnostic()
		// 	},
		// 	|credential_dir| {
		// 		event!(Level::INFO, "using systemd credential storage");
		// 		let path = [&credential_dir, OsStr::new("token")]
		// 			.iter()
		// 			.collect::<PathBuf>();
		// 		fs::read_to_string(path).into_diagnostic()
		// 	},
		// )?;

		// Ok(unsafe { token.leak() })
		if let Some(token) = unsafe { TOKEN } {
			Ok(token)
		} else {
			let token = env::var("DISCORD_TOKEN").into_diagnostic()?;
			unsafe {
				let leaked: &'static str = token.leak();
				TOKEN = Some(leaked);
				Ok(leaked)
			}
		}
	}
}
